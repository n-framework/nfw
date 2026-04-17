use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use n_framework_nfw_core_application::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use serde_yaml::Value as YamlValue;

const WORKSPACE_METADATA_FILE: &str = "nfw.yaml";

/// Errors produced by the `generate` CLI command.
#[derive(Debug)]
pub enum GenerateError {
    /// The `name` or `feature` argument contained invalid characters.
    GenerateInvalidIdentifier(String),
    /// The workspace root could not be located (e.g., `nfw.yaml` not found).
    GenerateWorkspaceError(String),
    /// The workspace configuration (`nfw.yaml`) is missing or invalid.
    GenerateConfigError(String),
    /// The requested template could not be found locally or in the cache.
    GenerateTemplateNotFound(String),
    /// A `--param` argument was malformed (not in `Key=Value` format).
    GenerateInvalidParameter(String),
    /// The underlying template engine reported a failure.
    GenerateExecutionFailed(TemplateError),
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GenerateInvalidIdentifier(msg) => write!(f, "{}", msg),
            Self::GenerateWorkspaceError(msg) => write!(f, "workspace error: {}", msg),
            Self::GenerateConfigError(msg) => write!(f, "configuration error: {}", msg),
            Self::GenerateTemplateNotFound(msg) => write!(f, "template not found: {}", msg),
            Self::GenerateInvalidParameter(msg) => write!(f, "invalid parameter: {}", msg),
            Self::GenerateExecutionFailed(err) => write!(f, "execution failed:\n{}", err),
        }
    }
}

impl std::error::Error for GenerateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::GenerateExecutionFailed(err) => Some(err),
            _ => None,
        }
    }
}

/// CLI command implementation for the `generate` subcommand.
#[derive(Debug, Clone)]
pub struct GenerateCliCommand<E> {
    engine: E,
    base_directory: Option<PathBuf>,
}

/// Request parameters for a generation operation.
#[derive(Debug, Clone)]
pub struct GenerateRequest<'a> {
    /// The type of component to generate (e.g. 'command', 'feature').
    pub generator_type: &'a str,
    /// The name of the new component.
    pub name: &'a str,
    /// Optional feature name to associate the component with.
    pub feature: Option<&'a str>,
    /// Optional arbitrary parameters as 'Key=Value' pairs.
    pub params: Option<&'a str>,
}

impl<E: TemplateEngine> GenerateCliCommand<E> {
    pub fn new(engine: E) -> Self {
        Self {
            engine,
            base_directory: None,
        }
    }

    pub fn with_base_directory(mut self, path: PathBuf) -> Self {
        self.base_directory = Some(path);
        self
    }

    pub fn execute(&self, request: GenerateRequest) -> Result<(), GenerateError> {
        tracing::info!(
            "Starting generation for '{}' named '{}'",
            request.generator_type,
            request.name
        );
        validate_identifiers(&request)?;

        let current_dir = if let Some(ref base) = self.base_directory {
            base.clone()
        } else {
            std::env::current_dir().map_err(|e| {
                let err = GenerateError::GenerateWorkspaceError(format!(
                    "failed to get current directory: {e}"
                ));
                tracing::error!("{}", err);
                err
            })?
        };

        let workspace_root = resolve_workspace_root(&current_dir).map_err(|e| {
            let err = GenerateError::GenerateWorkspaceError(e);
            tracing::error!("{}", err);
            err
        })?;

        let nfw_yaml = load_nfw_yaml(&workspace_root)?;
        tracing::debug!("Loaded nfw.yaml from {}", workspace_root.display());

        let template_id = extract_template_id(&nfw_yaml, request.generator_type)?;
        let template_root = resolve_template_root(&nfw_yaml, template_id, &workspace_root)
            .map_err(|e| {
                let err = GenerateError::GenerateTemplateNotFound(e);
                tracing::error!("{}", err);
                err
            })?;
        tracing::info!(
            "Using template '{}' from {}",
            template_id,
            template_root.display()
        );

        let config = load_and_validate_template_config(&template_root)?;

        let parameters = build_parameters(&nfw_yaml, &workspace_root, &request)?;
        tracing::debug!("Executing template with parameters: {:?}", parameters);

        self.engine
            .execute(&config, &template_root, &workspace_root, &parameters)
            .map_err(|e| {
                tracing::error!("Template execution failed: {}", e);
                GenerateError::GenerateExecutionFailed(e)
            })?;

        println!(
            "Generated '{}' '{}' successfully.",
            request.generator_type, request.name
        );
        tracing::info!(
            "Completed generation for '{}' '{}'",
            request.generator_type,
            request.name
        );
        Ok(())
    }
}

fn validate_identifiers(request: &GenerateRequest) -> Result<(), GenerateError> {
    if request.name.is_empty() {
        return Err(GenerateError::GenerateInvalidIdentifier(
            "name cannot be empty".to_string(),
        ));
    }
    let re = regex::Regex::new("^[a-zA-Z0-9_-]+$").map_err(|e| {
        GenerateError::GenerateWorkspaceError(format!("failed to initialize identifier regex: {e}"))
    })?;
    if !re.is_match(request.name) {
        return Err(GenerateError::GenerateInvalidIdentifier(format!(
            "invalid name '{}'. Only alphanumeric characters, hyphens, and underscores are allowed.",
            request.name
        )));
    }
    if let Some(feature) = request.feature {
        if feature.is_empty() {
            return Err(GenerateError::GenerateInvalidIdentifier(
                "feature cannot be empty if provided".to_string(),
            ));
        }
        if !re.is_match(feature) {
            return Err(GenerateError::GenerateInvalidIdentifier(format!(
                "invalid feature '{}'. Only alphanumeric characters, hyphens, and underscores are allowed.",
                feature
            )));
        }
    }
    Ok(())
}

fn load_nfw_yaml(workspace_root: &Path) -> Result<YamlValue, GenerateError> {
    let nfw_yaml_path = workspace_root.join(WORKSPACE_METADATA_FILE);
    let content = fs::read_to_string(&nfw_yaml_path).map_err(|e| {
        GenerateError::GenerateWorkspaceError(format!("failed to read nfw.yaml: {e}"))
    })?;
    serde_yaml::from_str(&content)
        .map_err(|e| GenerateError::GenerateWorkspaceError(format!("invalid nfw.yaml: {e}")))
}

fn extract_template_id<'a>(
    nfw_yaml: &'a YamlValue,
    generator_type: &str,
) -> Result<&'a str, GenerateError> {
    nfw_yaml
        .get("templates")
        .and_then(|g| g.get(generator_type))
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            GenerateError::GenerateConfigError(format!(
                "generator type '{}' not found in nfw.yaml. Add templates.{}: <template-id> to your nfw.yaml.",
                generator_type, generator_type
            ))
        })
}

fn load_and_validate_template_config(
    template_root: &Path,
) -> Result<TemplateConfig, GenerateError> {
    let path = template_root.join("template.yaml");
    let content = fs::read_to_string(&path).map_err(|e| {
        GenerateError::GenerateConfigError(format!(
            "failed to read template.yaml at {}: {e}",
            path.display()
        ))
    })?;
    let config: TemplateConfig = serde_yaml::from_str(&content).map_err(|e| {
        GenerateError::GenerateConfigError(format!("failed to parse template.yaml: {e}"))
    })?;
    config.validate().map_err(|e| {
        GenerateError::GenerateConfigError(format!("template validation failed: {e}"))
    })?;
    Ok(config)
}

fn build_parameters(
    nfw_yaml: &YamlValue,
    _workspace_root: &Path,
    request: &GenerateRequest,
) -> Result<TemplateParameters, GenerateError> {
    let namespace = resolve_namespace(nfw_yaml)?;

    let mut parameters = TemplateParameters::new()
        .with_name(request.name)
        .with_namespace(namespace);

    if let Some(feature) = request.feature {
        parameters = parameters.with_feature(feature);
    }

    if let Some(params_str) = request.params {
        for param_pair in params_str.split(',') {
            if let Some((key, value)) = param_pair.split_once('=') {
                parameters.insert(key.trim().to_string(), value.trim().to_string());
            } else {
                return Err(GenerateError::GenerateInvalidParameter(format!(
                    "invalid parameter format '{}'. expected Key=Value (multiple allowed, comma-separated, e.g. Name=Value,Type=String)",
                    param_pair
                )));
            }
        }
    }
    Ok(parameters)
}

fn resolve_namespace(nfw_yaml: &YamlValue) -> Result<String, GenerateError> {
    nfw_yaml
        .get("namespace")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| {
            GenerateError::GenerateConfigError(
                "missing 'namespace' in nfw.yaml. this is required for generation.".to_string(),
            )
        })
}

fn resolve_workspace_root(start_directory: &Path) -> Result<PathBuf, String> {
    let mut candidate = start_directory.to_path_buf();
    loop {
        let workspace_file = candidate.join(WORKSPACE_METADATA_FILE);
        if workspace_file.is_file() {
            return Ok(candidate);
        }
        let Some(parent) = candidate.parent() else {
            break;
        };
        candidate = parent.to_path_buf();
    }
    Err("could not find nfw.yaml in current directory or parent directories".to_string())
}

fn resolve_template_root(
    nfw_yaml: &YamlValue,
    template_id: &str,
    workspace_root: &Path,
) -> Result<PathBuf, String> {
    // Check if there is a template_sources.local entry mapping to a local directory
    if let Some(sources) = nfw_yaml.get("template_sources")
        && let Some(local) = sources.get("local")
        && let Some(local_path) = local.as_str()
    {
        let candidate = workspace_root.join(local_path).join(template_id);
        if candidate.is_dir() {
            return Ok(candidate);
        }
    }

    // Fallback: look in the nfw cache directory
    let cache_dir = dirs::cache_dir()
        .ok_or_else(|| "could not determine cache directory".to_string())?
        .join("nfw")
        .join("templates")
        .join(template_id);

    if cache_dir.is_dir() {
        return Ok(cache_dir);
    }

    Err(format!(
        "template '{}' not found locally or in cache",
        template_id
    ))
}
