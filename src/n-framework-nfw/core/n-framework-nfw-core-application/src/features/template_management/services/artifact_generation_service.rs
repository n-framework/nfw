use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};

use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;

use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;

#[derive(Debug, Clone)]
pub struct WorkspaceContext {
    pub workspace_root: PathBuf,
    pub nfw_yaml: YamlValue,
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub path: String,
    pub template_id: String,
}

pub struct AddArtifactContext {
    pub workspace_root: PathBuf,
    pub nfw_yaml: YamlValue,
    pub template_root: PathBuf,
    pub config: TemplateConfig,
    pub service_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct ArtifactGenerationService<W, R, E> {
    working_dir_provider: W,
    root_resolver: R,
    engine: E,
}

impl<W, R, E> ArtifactGenerationService<W, R, E>
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

    pub fn execute_generation(
        &self,
        command_name: &str,
        command_feature: Option<&str>,
        command_params: &Option<serde_json::Value>,
        context: &AddArtifactContext,
    ) -> Result<(), AddArtifactError> {
        self.validate_identifiers(command_name, command_feature)?;

        let parameters = self.build_parameters(
            &context.nfw_yaml,
            command_name,
            command_feature,
            command_params,
        )?;
        let output_root = context.workspace_root.join(&context.service_path);

        self.engine
            .execute(
                &context.config,
                &context.template_root,
                &output_root,
                &parameters,
            )
            .map_err(AddArtifactError::ExecutionFailed)?;

        Ok(())
    }

    pub fn get_workspace_context(&self) -> Result<WorkspaceContext, AddArtifactError> {
        let current_dir = self.working_dir_provider.current_dir().map_err(|e| {
            AddArtifactError::WorkspaceError(format!("failed to get current directory: {e}"))
        })?;

        let workspace_root = self.resolve_workspace_root(&current_dir).ok_or_else(|| {
            AddArtifactError::WorkspaceError(
                "could not find nfw.yaml in current or parent directories".to_string(),
            )
        })?;

        let nfw_yaml = self.load_nfw_yaml(&workspace_root)?;
        Ok(WorkspaceContext {
            workspace_root,
            nfw_yaml,
        })
    }

    pub fn extract_services(
        &self,
        workspace: &WorkspaceContext,
    ) -> Result<Vec<ServiceInfo>, AddArtifactError> {
        let mut result = Vec::new();
        if let Some(services) = workspace.nfw_yaml.get("services")
            && let Some(map) = services.as_mapping()
        {
            for (name_val, details_val) in map {
                if let (Some(name), Some(details)) = (name_val.as_str(), details_val.as_mapping()) {
                    let path = details
                        .get("path")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let template_id = details
                        .get("template")
                        .and_then(|t| t.as_mapping())
                        .and_then(|t| t.get("id"))
                        .and_then(|id| id.as_str())
                        .unwrap_or("")
                        .to_string();
                    result.push(ServiceInfo {
                        name: name.to_string(),
                        path,
                        template_id,
                    });
                }
            }
        }
        Ok(result)
    }

    pub fn list_features(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<Vec<String>, AddArtifactError> {
        let namespace = self.extract_namespace(&workspace.nfw_yaml)?;
        let features_root = workspace
            .workspace_root
            .join(&service.path)
            .join("src")
            .join("core")
            .join(format!("{}.Core.Application", namespace))
            .join("Features");

        if !features_root.is_dir() {
            return Ok(Vec::new());
        }

        let mut features = Vec::new();
        if let Ok(entries) = fs::read_dir(features_root) {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type()
                    && file_type.is_dir()
                    && let Some(name) = entry.file_name().to_str()
                {
                    features.push(name.to_string());
                }
            }
        }

        features.sort();
        Ok(features)
    }

    pub fn extract_namespace(&self, nfw_yaml: &YamlValue) -> Result<String, AddArtifactError> {
        nfw_yaml
            .get("workspace")
            .and_then(|w| w.get("namespace"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                AddArtifactError::ConfigError(
                    "missing 'workspace.namespace' in nfw.yaml. this is required for feature discovery."
                        .to_string(),
                )
            })
    }

    pub fn load_template_context(
        &self,
        workspace: WorkspaceContext,
        service: &ServiceInfo,
        generator_type: &str,
    ) -> Result<AddArtifactContext, AddArtifactError> {
        let template_id = format!("{}/{}", service.template_id, generator_type);

        let template_root = self
            .root_resolver
            .resolve(&workspace.nfw_yaml, &template_id, &workspace.workspace_root)
            .map_err(|_| {
                AddArtifactError::TemplateNotFound(format!(
                    "Could not resolve nested template '{}' for service '{}'.",
                    template_id, service.name
                ))
            })?;

        let config = self.load_and_validate_template_config(&template_root)?;

        Ok(AddArtifactContext {
            workspace_root: workspace.workspace_root,
            nfw_yaml: workspace.nfw_yaml,
            template_root,
            config,
            service_path: PathBuf::from(&service.path),
        })
    }

    pub fn validate_identifiers(
        &self,
        name: &str,
        feature: Option<&str>,
    ) -> Result<(), AddArtifactError> {
        if name.is_empty() {
            return Err(AddArtifactError::InvalidIdentifier(
                "name cannot be empty".to_string(),
            ));
        }

        let re = get_identifier_regex();
        if !re.is_match(name) {
            return Err(AddArtifactError::InvalidIdentifier(format!(
                "invalid name '{}'. Only alphanumeric characters, hyphens, and underscores are allowed.",
                name
            )));
        }

        if let Some(feat) = feature {
            if feat.is_empty() {
                return Err(AddArtifactError::InvalidIdentifier(
                    "feature cannot be empty if provided".to_string(),
                ));
            }
            if !re.is_match(feat) {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "invalid feature '{}'. Only alphanumeric characters, hyphens, and underscores are allowed.",
                    feat
                )));
            }
        }
        Ok(())
    }

    fn load_nfw_yaml(&self, workspace_root: &Path) -> Result<YamlValue, AddArtifactError> {
        let nfw_yaml_path = workspace_root.join("nfw.yaml");
        let content = fs::read_to_string(&nfw_yaml_path).map_err(|e| {
            AddArtifactError::WorkspaceError(format!("failed to read nfw.yaml: {e}"))
        })?;
        serde_yaml::from_str(&content)
            .map_err(|e| AddArtifactError::WorkspaceError(format!("invalid nfw.yaml: {e}")))
    }

    fn load_and_validate_template_config(
        &self,
        template_root: &Path,
    ) -> Result<TemplateConfig, AddArtifactError> {
        let path = template_root.join("template.yaml");
        let content = fs::read_to_string(&path).map_err(|e| {
            AddArtifactError::ConfigError(format!(
                "failed to read template.yaml at {}: {e}",
                path.display()
            ))
        })?;
        let config: TemplateConfig = serde_yaml::from_str(&content).map_err(|e| {
            AddArtifactError::ConfigError(format!("failed to parse template.yaml: {e}"))
        })?;
        config.validate().map_err(|e| {
            AddArtifactError::ConfigError(format!("template validation failed: {e}"))
        })?;
        Ok(config)
    }

    fn build_parameters(
        &self,
        nfw_yaml: &YamlValue,
        name: &str,
        feature: Option<&str>,
        params: &Option<serde_json::Value>,
    ) -> Result<TemplateParameters, AddArtifactError> {
        let namespace = self.extract_namespace(nfw_yaml)?;

        let parameters = TemplateParameters::new()
            .with_name(name)
            .map_err(AddArtifactError::InvalidParameter)?
            .with_namespace(namespace)
            .map_err(AddArtifactError::InvalidParameter)?;

        let mut parameters = parameters;
        if let Some(feat) = feature {
            parameters = parameters
                .with_feature(feat)
                .map_err(AddArtifactError::InvalidParameter)?;
        }

        if let Some(val) = params {
            if let serde_json::Value::Object(map) = val {
                for (key, value) in map {
                    parameters
                        .insert_value(key.clone(), value.clone())
                        .map_err(AddArtifactError::InvalidParameter)?;
                }
            } else {
                return Err(AddArtifactError::InvalidParameter(
                    "params must be a JSON object".to_string(),
                ));
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
    RE.get_or_init(|| regex::Regex::new("^[a-zA-Z0-9_-]+$").expect("invalid identifier regex"))
}
