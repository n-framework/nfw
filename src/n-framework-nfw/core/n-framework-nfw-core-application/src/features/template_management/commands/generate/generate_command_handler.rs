use std::fs;
use std::path::Path;
use serde_yaml::Value as YamlValue;

use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;

use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::template_management::models::errors::generate_error::GenerateError;
use super::generate_command::GenerateCommand;

#[derive(Debug, Clone)]
pub struct GenerateCommandHandler<W, R, E> {
    working_dir_provider: W,
    root_resolver: R,
    engine: E,
}

impl<W, R, E> GenerateCommandHandler<W, R, E>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
{
    pub fn new(working_dir_provider: W, root_resolver: R, engine: E) -> Self {
        Self {
            working_dir_provider,
            root_resolver,
            engine,
        }
    }

    pub fn handle(&self, command: &GenerateCommand) -> Result<(), GenerateError> {
        self.validate_identifiers(command)?;

        let current_dir = self.working_dir_provider.current_dir().map_err(|e| {
            GenerateError::WorkspaceError(format!("failed to get current directory: {e}"))
        })?;

        let workspace_root = self.resolve_workspace_root(&current_dir).ok_or_else(|| {
            GenerateError::WorkspaceError("could not find nfw.yaml in current or parent directories".to_string())
        })?;

        let nfw_yaml = self.load_nfw_yaml(&workspace_root)?;
        
        let parameters = self.build_parameters(&nfw_yaml, command)?;

        let template_id = self.extract_template_id(&nfw_yaml, &command.generator_type)?;
        let template_root = self.root_resolver.resolve(&nfw_yaml, template_id, &workspace_root)
            .map_err(GenerateError::TemplateNotFound)?;

        let config = self.load_and_validate_template_config(&template_root)?;

        self.engine
            .execute(&config, &template_root, &workspace_root, &parameters)
            .map_err(GenerateError::ExecutionFailed)?;

        Ok(())
    }

    fn validate_identifiers(&self, command: &GenerateCommand) -> Result<(), GenerateError> {
        if command.name.is_empty() {
            return Err(GenerateError::InvalidIdentifier("name cannot be empty".to_string()));
        }
        
        let re = get_identifier_regex();
        if !re.is_match(&command.name) {
            return Err(GenerateError::InvalidIdentifier(format!(
                "invalid name '{}'. Only alphanumeric characters, hyphens, and underscores are allowed.",
                command.name
            )));
        }
        
        if let Some(ref feature) = command.feature {
            if feature.is_empty() {
                return Err(GenerateError::InvalidIdentifier("feature cannot be empty if provided".to_string()));
            }
            if !re.is_match(feature) {
                return Err(GenerateError::InvalidIdentifier(format!(
                    "invalid feature '{}'. Only alphanumeric characters, hyphens, and underscores are allowed.",
                    feature
                )));
            }
        }
        Ok(())
    }

    fn load_nfw_yaml(&self, workspace_root: &Path) -> Result<YamlValue, GenerateError> {
        let nfw_yaml_path = workspace_root.join("nfw.yaml");
        let content = fs::read_to_string(&nfw_yaml_path).map_err(|e| {
            GenerateError::WorkspaceError(format!("failed to read nfw.yaml: {e}"))
        })?;
        serde_yaml::from_str(&content)
            .map_err(|e| GenerateError::WorkspaceError(format!("invalid nfw.yaml: {e}")))
    }

    fn extract_template_id<'a>(&self, nfw_yaml: &'a YamlValue, generator_type: &str) -> Result<&'a str, GenerateError> {
        nfw_yaml
            .get("templates")
            .and_then(|g| g.get(generator_type))
            .and_then(|v| v.as_str())
            .ok_or_else(|| {
                GenerateError::ConfigError(format!(
                    "generator type '{}' not found in nfw.yaml. Add templates.{}: <template-id> to your nfw.yaml.",
                    generator_type, generator_type
                ))
            })
    }

    fn load_and_validate_template_config(&self, template_root: &Path) -> Result<TemplateConfig, GenerateError> {
        let path = template_root.join("template.yaml");
        let content = fs::read_to_string(&path).map_err(|e| {
            GenerateError::ConfigError(format!(
                "failed to read template.yaml at {}: {e}",
                path.display()
            ))
        })?;
        let config: TemplateConfig = serde_yaml::from_str(&content).map_err(|e| {
            GenerateError::ConfigError(format!("failed to parse template.yaml: {e}"))
        })?;
        config.validate().map_err(|e| {
            GenerateError::ConfigError(format!("template validation failed: {e}"))
        })?;
        Ok(config)
    }

    fn build_parameters(&self, nfw_yaml: &YamlValue, command: &GenerateCommand) -> Result<TemplateParameters, GenerateError> {
        let namespace = nfw_yaml
            .get("workspace")
            .and_then(|w| w.get("namespace"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                GenerateError::ConfigError("missing 'workspace.namespace' in nfw.yaml. this is required for generation.".to_string())
            })?;

        let mut parameters = TemplateParameters::new()
            .with_name(&command.name)
            .with_namespace(namespace);

        if let Some(ref feature) = command.feature {
            parameters = parameters.with_feature(feature);
        }

        if let Some(ref params_str) = command.params {
            for param_pair in params_str.split(',') {
                if let Some((key, value)) = param_pair.split_once('=') {
                     parameters.insert(key.trim(), value.trim()).map_err(GenerateError::InvalidParameter)?;
                } else {
                    return Err(GenerateError::InvalidParameter(format!(
                        "invalid parameter format '{}'. expected Key=Value",
                        param_pair
                    )));
                }
            }
        }
        Ok(parameters)
    }

    fn resolve_workspace_root(&self, start_directory: &Path) -> Option<std::path::PathBuf> {
        let mut candidate = start_directory.to_path_buf();
        loop {
            if candidate.join("nfw.yaml").is_file() {
                return Some(candidate);
            }
            let parent = candidate.parent()?;
            candidate = parent.to_path_buf();
        }
    }
}

fn get_identifier_regex() -> &'static regex::Regex {
    use std::sync::OnceLock;
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| {
        regex::Regex::new("^[a-zA-Z0-9_-]+$").expect("invalid identifier regex")
    })
}
