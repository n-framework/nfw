use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::{Path, PathBuf};

use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use n_framework_nfw_infrastructure_workspace_metadata::{
    PreservedComments, extract_preserved_comments, format_nfw_yaml_document,
};

use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;

#[derive(Debug, Clone)]
pub struct WorkspaceContext {
    workspace_root: PathBuf,
    nfw_yaml: YamlValue,
    preserved_comments: PreservedComments,
}

impl WorkspaceContext {
    pub fn new(
        workspace_root: PathBuf,
        nfw_yaml: YamlValue,
        preserved_comments: PreservedComments,
    ) -> Self {
        Self {
            workspace_root,
            nfw_yaml,
            preserved_comments,
        }
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn nfw_yaml(&self) -> &YamlValue {
        &self.nfw_yaml
    }

    pub fn preserved_comments(&self) -> &PreservedComments {
        &self.preserved_comments
    }
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    name: String,
    path: String,
    template_id: String,
}

impl ServiceInfo {
    pub fn new(name: String, path: String, template_id: String) -> Result<Self, String> {
        if name.is_empty() {
            return Err("Service name cannot be empty".to_string());
        }
        if path.is_empty() {
            return Err("Service path cannot be empty".to_string());
        }
        if template_id.is_empty() {
            return Err("Service template ID cannot be empty".to_string());
        }
        Ok(Self {
            name,
            path,
            template_id,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn template_id(&self) -> &str {
        &self.template_id
    }
}

#[derive(Debug, Clone)]
pub struct AddArtifactContext {
    pub workspace_root: PathBuf,
    pub nfw_yaml: YamlValue,
    pub preserved_comments: PreservedComments,
    pub template_root: PathBuf,
    pub config: TemplateConfig,
    pub service_name: String,
    pub service_path: PathBuf,
}

/// Service for generating artifacts from templates.
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
    /// Creates a new instance of `ArtifactGenerationService`.
    pub fn new(working_dir_provider: W, root_resolver: R, engine: E) -> Self {
        Self {
            working_dir_provider,
            root_resolver,
            engine,
        }
    }

    pub fn engine(&self) -> &E {
        &self.engine
    }

    /// Executes the artifact generation process.
    pub fn execute_generation(
        &self,
        command_name: &str,
        command_feature: Option<&str>,
        command_params: &Option<serde_json::Value>,
        context: &AddArtifactContext,
    ) -> Result<(), AddArtifactError> {
        self.validate_identifiers(command_name, command_feature)?;
        self.validate_required_modules(&context.config, &context.nfw_yaml, &context.service_path)?;

        let parameters = self.build_parameters(
            &context.nfw_yaml,
            command_name,
            &context.service_name,
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
            .map_err(|e| AddArtifactError::ExecutionFailed(Box::new(e)))?;

        Ok(())
    }

    /// Validates that the required modules are installed.
    pub fn validate_required_modules(
        &self,
        config: &TemplateConfig,
        nfw_yaml: &YamlValue,
        service_path: &Path,
    ) -> Result<(), AddArtifactError> {
        let required = config.required_modules();
        if required.is_empty() {
            return Ok(());
        }

        let installed = self.get_service_modules(nfw_yaml, service_path)?;

        for module in required {
            if !installed.iter().any(|m| m == module) {
                return Err(AddArtifactError::MissingRequiredModule(format!(
                    "module '{}' is required but not installed. Run: nfw add {}",
                    module, module
                )));
            }
        }

        Ok(())
    }

    fn get_service_modules(
        &self,
        nfw_yaml: &YamlValue,
        service_path: &Path,
    ) -> Result<Vec<String>, AddArtifactError> {
        let service_path_str = service_path.to_string_lossy();
        let services = nfw_yaml
            .get("services")
            .and_then(|s| s.as_mapping())
            .ok_or_else(|| {
                AddArtifactError::ConfigError(
                    "nfw.yaml is missing its 'services' mapping".to_string(),
                )
            })?;

        for (name, details) in services {
            let path = details
                .get("path")
                .and_then(|v| v.as_str())
                .ok_or_else(|| {
                    let service_name = name.as_str().unwrap_or("unknown");
                    AddArtifactError::ConfigError(format!(
                        "service '{}' is missing its 'path' field",
                        service_name
                    ))
                })?;

            if path == service_path_str {
                return Ok(details
                    .get("modules")
                    .and_then(|m| m.as_sequence())
                    .map(|seq| {
                        seq.iter()
                            .filter_map(|v| v.as_str())
                            .map(|s| s.to_string())
                            .collect()
                    })
                    .unwrap_or_default());
            }
        }

        Ok(Vec::new())
    }

    /// Retrieves the workspace context for the current directory.
    pub fn get_workspace_context(&self) -> Result<WorkspaceContext, AddArtifactError> {
        let current_dir = self.working_dir_provider.current_dir().map_err(|e| {
            AddArtifactError::WorkspaceError(format!("failed to get current directory: {e}"))
        })?;

        let workspace_root = self.resolve_workspace_root(&current_dir).ok_or_else(|| {
            AddArtifactError::WorkspaceError(
                "could not find nfw.yaml in current or parent directories".to_string(),
            )
        })?;

        let (nfw_yaml, preserved_comments) = self.read_nfw_yaml(&workspace_root)?;
        Ok(WorkspaceContext {
            workspace_root,
            nfw_yaml,
            preserved_comments,
        })
    }

    /// Extracts all services defined in the workspace.
    pub fn extract_services(
        &self,
        workspace: &WorkspaceContext,
    ) -> Result<Vec<ServiceInfo>, AddArtifactError> {
        let mut result = Vec::new();
        if let Some(services) = workspace.nfw_yaml.get("services") {
            let map = services.as_mapping().ok_or_else(|| {
                AddArtifactError::ConfigError(
                    "nfw.yaml is missing its 'services' mapping".to_string(),
                )
            })?;
            for (name_val, details_val) in map {
                let name = name_val.as_str().ok_or_else(|| {
                    AddArtifactError::ConfigError(
                        "service key in nfw.yaml must be a string".to_string(),
                    )
                })?;
                let details = details_val.as_mapping().ok_or_else(|| {
                    AddArtifactError::ConfigError(format!(
                        "service '{name}' details must be a mapping"
                    ))
                })?;

                let path = details
                    .get("path")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        AddArtifactError::ConfigError(format!(
                            "service '{name}' is missing its 'path' field"
                        ))
                    })?
                    .to_string();

                let template_id = details
                    .get("template")
                    .and_then(|t| t.as_mapping())
                    .and_then(|t| t.get("id"))
                    .and_then(|id| id.as_str())
                    .ok_or_else(|| {
                        AddArtifactError::ConfigError(format!(
                            "service '{name}' is missing its 'template.id' field"
                        ))
                    })?
                    .to_string();

                result.push(ServiceInfo {
                    name: name.to_string(),
                    path,
                    template_id,
                });
            }
        }
        Ok(result)
    }

    /// Lists all features available in the workspace.
    ///
    /// This method searches for the `Features` directory in the service's src folder,
    /// traversing the logical namespace path if necessary.
    pub fn list_features(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<Vec<String>, AddArtifactError> {
        let namespace = self.extract_namespace(&workspace.nfw_yaml)?;
        let service_root = workspace.workspace_root.join(&service.path);

        let possible_roots = vec![
            service_root
                .join("src")
                .join("core")
                .join(format!("{}.Core.Application", namespace))
                .join("Features"),
            service_root
                .join("src")
                .join("core")
                .join(format!("{}.Core.Domain", namespace))
                .join("Features"),
            service_root
                .join("src")
                .join("Application")
                .join("Features"),
            service_root.join("src").join("Features"),
            service_root.join("Features"),
            service_root.join("specs").join("features"),
        ];

        let mut features_root = None;
        for root in possible_roots {
            tracing::debug!("Checking for Features root at: {}", root.display());
            if root.is_dir() {
                tracing::info!("Found Features root at: {}", root.display());
                features_root = Some(root);
                break;
            }
        }

        let Some(features_root) = features_root else {
            tracing::warn!(
                "No Features directory found in service '{}' ({})",
                service.name,
                service.path
            );
            return Ok(Vec::new());
        };

        let mut features = Vec::new();
        match fs::read_dir(&features_root) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let _path = entry.path();
                    if let Ok(file_type) = entry.file_type()
                        && file_type.is_dir()
                        && let Some(name) = entry.file_name().to_str()
                    {
                        features.push(name.to_string());
                    }
                }
            }
            Err(e) => {
                tracing::error!(
                    "Failed to read directory {}: {}",
                    features_root.display(),
                    e
                );
                return Err(AddArtifactError::WorkspaceError(format!(
                    "failed to read features directory {}: {}",
                    features_root.display(),
                    e
                )));
            }
        }
        Ok(features)
    }

    /// Extracts the namespace from the workspace configuration.
    pub fn extract_namespace(&self, nfw_yaml: &YamlValue) -> Result<String, AddArtifactError> {
        nfw_yaml
            .get("workspace")
            .and_then(|w| w.get("namespace"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| {
                AddArtifactError::ConfigError(
                    "Missing 'workspace.namespace' in nfw.yaml. This is required for feature discovery."
                        .to_string(),
                )
            })
    }

    /// Two-Tier Dynamic Sub-Template Resolution Strategy:
    /// This method resolves the correct template configuration for secondary artifact generation (like persistence or mediator modules).
    ///
    /// 1. Primary Resolution (Base Catalog): It leverages the `TemplateRootResolver` to find the base template directory (either local or from the global catalog).
    /// 2. Secondary Resolution (Sub-folder extraction): It parses the base `template.yaml` located at the primary root.
    ///    It uses the `generators` map in the base configuration to map the requested `generator_type` (e.g., 'persistence')
    ///    to the relative sub-folder. If no such mapping exists, it assumes the sub-folder matches the `generator_type` perfectly.
    /// 3. Context Contextualization: Constructs the final `AddArtifactContext` containing the nested configuration to be used by the engine.
    pub fn load_template_context(
        &self,
        workspace: WorkspaceContext,
        service: &ServiceInfo,
        generator_type: &str,
    ) -> Result<AddArtifactContext, AddArtifactError> {
        let base_root = self
            .root_resolver
            .resolve(
                &workspace.nfw_yaml,
                &service.template_id,
                &workspace.workspace_root,
            )
            .map_err(|_| {
                AddArtifactError::TemplateNotFound(format!(
                    "Could not resolve base template '{}' for service '{}'.",
                    service.template_id, service.name
                ))
            })?;

        let base_config = self.load_and_validate_template_config(&base_root)?;

        let sub_folder = base_config
            .generators()
            .and_then(|g| g.get(generator_type))
            .map(|s| s.as_str())
            .ok_or_else(|| {
                AddArtifactError::ConfigError(format!(
                    "Base template '{}' does not support generator type '{}'. Check 'generators' mapping in template.yaml.",
                    service.template_id, generator_type
                ))
            })?;

        let template_root = base_root.join(sub_folder);

        let config = self.load_and_validate_template_config(&template_root)?;

        Ok(AddArtifactContext {
            workspace_root: workspace.workspace_root,
            nfw_yaml: workspace.nfw_yaml,
            preserved_comments: workspace.preserved_comments,
            template_root,
            config,
            service_name: service.name.clone(),
            service_path: PathBuf::from(&service.path),
        })
    }

    /// Validates the identifiers provided for artifact generation.
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

    fn read_nfw_yaml(
        &self,
        workspace_root: &Path,
    ) -> Result<(serde_yaml::Value, PreservedComments), AddArtifactError> {
        let nfw_yaml_path = workspace_root.join("nfw.yaml");
        let content = fs::read_to_string(&nfw_yaml_path).map_err(|e| {
            tracing::error!(
                "Failed to read workspace config at {}: {}",
                nfw_yaml_path.display(),
                e
            );
            AddArtifactError::NfwYamlReadError(format!(
                "failed to read workspace config at {}: {e}",
                nfw_yaml_path.display()
            ))
        })?;
        let preserved_comments = extract_preserved_comments(&content);
        let value = serde_yaml::from_str(&content).map_err(|e| {
            tracing::error!(
                "Failed to parse workspace config at {}: {}",
                nfw_yaml_path.display(),
                e
            );
            AddArtifactError::NfwYamlParseError(format!(
                "failed to parse workspace config at {}: {e}",
                nfw_yaml_path.display()
            ))
        })?;
        Ok((value, preserved_comments))
    }

    fn write_nfw_yaml(
        &self,
        workspace_root: &Path,
        yaml: &serde_yaml::Value,
        preserved_comments: &PreservedComments,
    ) -> Result<(), AddArtifactError> {
        let nfw_yaml_path = workspace_root.join("nfw.yaml");
        let serialized = serde_yaml::to_string(yaml).map_err(|e| {
            tracing::error!(
                "Failed to serialize workspace config for {}: {}",
                nfw_yaml_path.display(),
                e
            );
            AddArtifactError::NfwYamlWriteError(format!(
                "failed to serialize workspace config for {}: {e}",
                nfw_yaml_path.display()
            ))
        })?;

        let output = format_nfw_yaml_document(&serialized, preserved_comments);

        fs::write(&nfw_yaml_path, output).map_err(|e| {
            tracing::error!(
                "Failed to write workspace config at {}: {}",
                nfw_yaml_path.display(),
                e
            );
            AddArtifactError::NfwYamlWriteError(format!(
                "failed to write workspace config at {}: {e}",
                nfw_yaml_path.display()
            ))
        })?;

        Ok(())
    }

    fn load_and_validate_template_config(
        &self,
        template_root: &Path,
    ) -> Result<TemplateConfig, AddArtifactError> {
        let path = template_root.join("template.yaml");
        let content = fs::read_to_string(&path).map_err(|e| {
            tracing::error!(
                "Failed to read template config at {}: {}",
                path.display(),
                e
            );
            AddArtifactError::ConfigError(format!(
                "failed to read template config at {}: {e}",
                path.display()
            ))
        })?;
        let config: TemplateConfig = serde_yaml::from_str(&content).map_err(|e| {
            AddArtifactError::ConfigError(format!(
                "failed to parse template config at {}: {e}",
                path.display()
            ))
        })?;
        config.validate().map_err(|e| {
            AddArtifactError::ConfigError(format!(
                "Template validation failed for {}: {e}",
                path.display()
            ))
        })?;
        Ok(config)
    }

    fn build_parameters(
        &self,
        nfw_yaml: &YamlValue,
        name: &str,
        service_name: &str,
        feature: Option<&str>,
        params: &Option<serde_json::Value>,
    ) -> Result<TemplateParameters, AddArtifactError> {
        let namespace = self.extract_namespace(nfw_yaml)?;

        let parameters = TemplateParameters::new()
            .with_name(name)
            .map_err(AddArtifactError::InvalidParameter)?
            .with_namespace(namespace)
            .map_err(AddArtifactError::InvalidParameter)?
            .with_service(service_name)
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

    /// Adds a module to the service in the workspace configuration.
    pub fn add_service_module(
        &self,
        workspace_root: &Path,
        service_name: &str,
        module_name: &str,
    ) -> Result<(), AddArtifactError> {
        let (mut yaml, preserved_comments) = self.read_nfw_yaml(workspace_root)?;

        let services = yaml
            .get_mut("services")
            .and_then(|s| s.as_mapping_mut())
            .ok_or_else(|| {
                AddArtifactError::WorkspaceError(
                    "nfw.yaml is missing 'services' mapping".to_string(),
                )
            })?;

        let service_key = YamlValue::String(service_name.to_string());
        let details = services
            .get_mut(&service_key)
            .and_then(|d| d.as_mapping_mut())
            .ok_or_else(|| {
                AddArtifactError::WorkspaceError(format!(
                    "service '{service_name}' not found in nfw.yaml"
                ))
            })?;

        let modules = details
            .entry(YamlValue::String("modules".to_string()))
            .or_insert_with(|| YamlValue::Sequence(Vec::new()));

        let module_value = YamlValue::String(module_name.to_string());
        if let Some(seq) = modules.as_sequence_mut() {
            if seq.contains(&module_value) {
                tracing::info!(
                    "Module '{}' is already registered for service '{}', skipping.",
                    module_name,
                    service_name
                );
            } else {
                seq.push(module_value);
            }
        }

        self.write_nfw_yaml(workspace_root, &yaml, &preserved_comments)?;

        Ok(())
    }
}

fn get_identifier_regex() -> &'static regex::Regex {
    use std::sync::OnceLock;
    static RE: OnceLock<regex::Regex> = OnceLock::new();
    RE.get_or_init(|| regex::Regex::new("^[a-zA-Z0-9_-]+$").expect("invalid identifier regex"))
}

#[cfg(test)]
#[path = "artifact_generation_service.tests.rs"]
mod tests;
