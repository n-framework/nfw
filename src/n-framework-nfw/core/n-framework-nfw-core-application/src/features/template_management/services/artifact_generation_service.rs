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

pub const GEN_TYPE_COMMAND: &str = "command";
pub const GEN_TYPE_QUERY: &str = "query";
pub const GEN_TYPE_WEBAPI: &str = "webapi";

#[derive(Debug, Clone)]
pub struct WorkspaceContext {
    workspace_root: PathBuf,
    nfw_yaml: serde_yaml::Value,
    preserved_comments: PreservedComments,
}

impl WorkspaceContext {
    pub fn new(
        workspace_root: PathBuf,
        nfw_yaml: serde_yaml::Value,
        preserved_comments: PreservedComments,
    ) -> Result<Self, AddArtifactError> {
        if !workspace_root.is_dir() {
            return Err(AddArtifactError::WorkspaceError(format!(
                "Workspace root is not a valid directory: {}",
                workspace_root.display()
            )));
        }

        Ok(Self {
            workspace_root,
            nfw_yaml,
            preserved_comments,
        })
    }

    pub fn workspace_root(&self) -> &PathBuf {
        &self.workspace_root
    }

    pub fn nfw_yaml(&self) -> &serde_yaml::Value {
        &self.nfw_yaml
    }

    pub fn preserved_comments(&self) -> &PreservedComments {
        &self.preserved_comments
    }
}

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    pub name: String,
    pub path: String,
    pub template_id: String,
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
    pub workspace: WorkspaceContext,
    pub template_root: PathBuf,
    pub config: TemplateConfig,
    pub service_name: String,
    pub service_path: PathBuf,
}

impl AddArtifactContext {
    pub fn new(
        workspace: WorkspaceContext,
        template_root: PathBuf,
        config: TemplateConfig,
        service_name: String,
        service_path: PathBuf,
    ) -> Result<Self, AddArtifactError> {
        if service_name.is_empty() {
            return Err(AddArtifactError::InvalidParameter(
                "service_name cannot be empty".to_string(),
            ));
        }
        if service_path.as_os_str().is_empty() {
            return Err(AddArtifactError::InvalidParameter(
                "service_path cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            workspace,
            template_root,
            config,
            service_name,
            service_path,
        })
    }

    pub fn workspace_root(&self) -> &PathBuf {
        &self.workspace.workspace_root
    }

    pub fn nfw_yaml(&self) -> &YamlValue {
        &self.workspace.nfw_yaml
    }

    pub fn preserved_comments(&self) -> &PreservedComments {
        &self.workspace.preserved_comments
    }

    pub fn template_root(&self) -> &PathBuf {
        &self.template_root
    }

    pub fn config(&self) -> &TemplateConfig {
        &self.config
    }

    pub fn service_name(&self) -> &str {
        &self.service_name
    }

    pub fn service_path(&self) -> &PathBuf {
        &self.service_path
    }
}

#[derive(Debug, Clone)]
pub struct ArtifactGenerationService<W, R, E> {
    working_dir_provider: W,
    root_resolver: R,
    engine: E,
}

const PRIORITY_APP_LAYER: u8 = 0;
const PRIORITY_PRESENTATION_LAYER: u8 = 1;

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
        self.validate_required_modules(
            &context.config,
            &context.workspace.nfw_yaml,
            &context.service_path,
        )?;

        let parameters = self.build_parameters(
            &context.workspace.nfw_yaml,
            command_name,
            &context.service_name,
            command_feature,
            command_params,
        )?;
        let output_root = context.workspace.workspace_root.join(&context.service_path);

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

        // Return successfully if all required modules are present or if no modules are required.
        // This covers the case where the service list might be empty but no specific modules are enforced.
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
        WorkspaceContext::new(workspace_root, nfw_yaml, preserved_comments)
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
    /// The features root is resolved from the service's own template step destinations — no
    /// hardcoded paths. See `derive_features_root` for the derivation logic.
    pub fn list_features(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<Vec<String>, AddArtifactError> {
        let Some(features_root) = self.derive_features_root(workspace, service)? else {
            return Ok(Vec::new());
        };

        if !features_root.is_dir() {
            return Ok(Vec::new());
        }

        let mut features = Vec::new();

        let entries = std::fs::read_dir(&features_root).map_err(|e| {
            AddArtifactError::WorkspaceError(format!(
                "failed to read features directory {}: {}",
                features_root.display(),
                e
            ))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                AddArtifactError::WorkspaceError(format!(
                    "failed to read directory entry in {}: {}",
                    features_root.display(),
                    e
                ))
            })?;

            match entry.file_type() {
                Ok(ft) if ft.is_dir() => {
                    if let Some(name) = entry.file_name().to_str() {
                        features.push(name.to_string());
                    }
                }
                Ok(_) => {} // skip files
                Err(e) => {
                    tracing::warn!(
                        "Failed to get file type for entry {}: {}",
                        entry.path().display(),
                        e
                    );
                }
            }
        }
        Ok(features)
    }

    /// Finds the root directory where features are stored for a given service.
    ///
    /// Instead of maintaining a hardcoded list of candidate paths (which breaks for non-.NET
    /// languages), this method derives the features root from the service's own template step
    /// destinations. It loads the base template config, iterates every generator type declared in
    /// the `generators` map, and for each one looks for a `Render` step whose destination
    /// contains a `{{ Feature }}` placeholder. The path prefix up to (but not including) that
    /// placeholder segment — with `{{ Service }}` substituted — is the features root.
    ///
    /// Returns `Ok(None)` when no matching template step is found (e.g. the service template
    /// declares no generators with a feature-level destination).
    pub fn derive_features_root(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<Option<std::path::PathBuf>, AddArtifactError> {
        use n_framework_nfw_core_domain::features::template_management::template_config::TemplateStepAction;

        let base_root = self
            .root_resolver
            .resolve(
                &workspace.nfw_yaml,
                &service.template_id,
                &workspace.workspace_root,
            )
            .map_err(|e| {
                AddArtifactError::TemplateNotFound(format!(
                    "Could not resolve base template '{}' for service '{}': {}",
                    service.template_id, service.name, e
                ))
            })?;

        let base_config = self.load_and_validate_template_config(&base_root)?;

        let generators = base_config.generators().ok_or_else(|| {
            AddArtifactError::ConfigError(format!(
                "No generators declared in template '{}' for service '{}' — cannot derive features root.",
                service.template_id,
                service.name
            ))
        })?;

        let mut candidates: Vec<(String, std::path::PathBuf, u8)> = Vec::new();

        for sub_folder in generators.values() {
            let template_root = base_root.join(sub_folder.as_str());
            let config = match self.load_and_validate_template_config(&template_root) {
                Ok(c) => c,
                Err(_) => continue,
            };

            for step in config.steps() {
                let destination = match &step.action {
                    TemplateStepAction::Render { destination, .. } => destination,
                    _ => continue,
                };

                let segments: Vec<&str> = destination.split('/').collect();

                let feature_idx = segments
                    .iter()
                    .position(|s| s.contains("{{ Feature }}") || s.contains("{{Feature}}"));

                let Some(fi) = feature_idx else { continue };

                let namespace = workspace
                    .nfw_yaml
                    .get("workspace")
                    .and_then(|w| w.get("namespace"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(&service.name);

                let prefix_segments: Vec<String> = segments[..fi]
                    .iter()
                    .map(|s| {
                        s.replace("{{ Service }}", &service.name)
                            .replace("{{Service}}", &service.name)
                            .replace("{{ Namespace }}", namespace)
                            .replace("{{Namespace}}", namespace)
                    })
                    .collect();

                let base = workspace.workspace_root.join(&service.path);
                let features_root: PathBuf =
                    prefix_segments.iter().fold(base, |acc, seg| acc.join(seg));

                // Prefer Application/core layer over Presentation layer
                // by ranking generators: command/query/entity = 0 (highest priority), endpoint/webapi = 1
                let is_application_layer = sub_folder.as_str().contains("command")
                    || sub_folder.as_str().contains("query")
                    || sub_folder.as_str().contains("entity");
                let priority = if is_application_layer {
                    PRIORITY_APP_LAYER
                } else {
                    PRIORITY_PRESENTATION_LAYER
                };

                candidates.push((sub_folder.as_str().to_string(), features_root, priority));
            }
        }

        // Sort by priority (application layer first), then by whether the directory exists
        candidates.sort_by(|a, b| {
            a.2.cmp(&b.2).then_with(|| {
                // Secondary sort: prefer directories that exist and have content
                let a_has_content =
                    a.1.is_dir() && a.1.read_dir().is_ok_and(|mut d| d.next().is_some());
                let b_has_content =
                    b.1.is_dir() && b.1.read_dir().is_ok_and(|mut d| d.next().is_some());
                b_has_content.cmp(&a_has_content)
            })
        });

        let mut last_candidate: Option<PathBuf> = None;
        for (generator_name, features_root, _) in candidates {
            last_candidate = Some(features_root.clone());

            if features_root.is_dir() {
                tracing::info!(
                    "Derived features root from template '{}': {}",
                    generator_name,
                    features_root.display()
                );
                return Ok(Some(features_root));
            }

            tracing::debug!(
                "Template-derived features root does not exist yet: {}",
                features_root.display()
            );
        }

        // Return the last candidate path for new feature creation when no existing directory found
        if let Some(path) = last_candidate {
            tracing::debug!(
                "No existing features directory found, returning candidate path for new feature creation: {}",
                path.display()
            );
            return Ok(Some(path));
        }

        tracing::warn!(
            "No generator template for service '{}' declares a '{{{{ Feature }}}}' destination — features root cannot be derived.",
            service.name
        );
        Ok(None)
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
            workspace,
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
                return Err(AddArtifactError::WorkspaceError(format!(
                    "Module '{}' is already registered for service '{}'. No changes were made.",
                    module_name, service_name
                )));
            }
            seq.push(module_value);
        }

        self.write_nfw_yaml(workspace_root, &yaml, &preserved_comments)?;

        Ok(())
    }

    pub fn has_service_module(
        &self,
        workspace_root: &Path,
        service_name: &str,
        module_name: &str,
    ) -> Result<bool, AddArtifactError> {
        let (yaml, _) = self.read_nfw_yaml(workspace_root)?;

        let modules = yaml
            .get("services")
            .and_then(|s| s.get(service_name))
            .and_then(|details| details.get("modules"));

        if let Some(modules_value) = modules
            && let Some(seq) = modules_value.as_sequence()
        {
            let module_value = serde_yaml::Value::String(module_name.to_string());
            return Ok(seq.contains(&module_value));
        }

        Ok(false)
    }

    /// Resolves the concrete directory that holds mediator artifacts (Commands or Queries) for a
    /// specific feature, along with the file suffix used to identify them (e.g. `"Command.cs"`).
    ///
    /// Instead of relying on hardcoded paths, this method reads the `command` or `query` template's
    /// step destinations to discover where the framework places those files for the given service.
    ///
    /// # Returns
    /// `Ok(Some((feature_dir, file_suffix)))` when the template provides a usable destination
    /// pattern, `Ok(None)` when no `Render` step with a `{{ Feature }}` placeholder is found.
    pub fn resolve_mediator_artifact_root(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
        generator_type: &str,
    ) -> Result<Option<(PathBuf, Vec<String>, String)>, AddArtifactError> {
        use n_framework_nfw_core_domain::features::template_management::template_config::TemplateStepAction;

        let context = self.load_template_context(workspace.clone(), service, generator_type)?;

        for step in context.config.steps() {
            let destination = match &step.action {
                TemplateStepAction::Render { destination, .. } => destination,
                _ => continue,
            };

            let segments: Vec<&str> = destination.split('/').collect();

            let feature_idx = segments
                .iter()
                .position(|s| s.contains("{{ Feature }}") || s.contains("{{Feature}}"));

            let Some(fi) = feature_idx else { continue };

            // Locate the first {{ Name }} segment after {{ Feature }}.
            let name_idx = segments[fi + 1..]
                .iter()
                .position(|s| s.contains("{{ Name }}") || s.contains("{{Name}}"))
                .map(|rel| fi + 1 + rel);

            // Build the features root from everything up to (not including) {{ Feature }}, with
            // {{ Service }} and {{ Namespace }} substituted for the actual values.
            let namespace = workspace
                .nfw_yaml
                .get("workspace")
                .and_then(|w| w.get("namespace"))
                .and_then(|v| v.as_str())
                .unwrap_or(&service.name);

            let prefix_segments: Vec<String> = segments[..fi]
                .iter()
                .map(|s| {
                    s.replace("{{ Service }}", &service.name)
                        .replace("{{Service}}", &service.name)
                        .replace("{{ Namespace }}", namespace)
                        .replace("{{Namespace}}", namespace)
                })
                .collect();

            let base = workspace.workspace_root.join(&service.path);
            let features_root: PathBuf =
                prefix_segments.iter().fold(base, |acc, seg| acc.join(seg));

            // Collect static sub-directory segments that sit between {{ Feature }} and {{ Name }}
            // (e.g. `Commands`, `Queries`). These are returned separately so callers can append
            // them after the feature name: `features_root / feature / sub_dirs`.
            let sub_dirs: Vec<String> = if let Some(ni) = name_idx {
                segments[fi + 1..ni]
                    .iter()
                    .filter(|s| !s.contains("{{"))
                    .map(|s| s.to_string())
                    .collect()
            } else {
                Vec::new()
            };

            // Derive the artifact file suffix from the filename segment (the last segment).
            // The filename looks like `{{ Name }}Command.cs` — strip the `{{ Name }}` prefix.
            let file_suffix = segments
                .last()
                .map(|s| {
                    s.strip_prefix("{{ Name }}")
                        .or_else(|| s.strip_prefix("{{Name}}"))
                        .unwrap_or(s)
                        .to_string()
                })
                .unwrap_or_default();

            return Ok(Some((features_root, sub_dirs, file_suffix)));
        }

        tracing::warn!(
            "No Render step with {{ Feature }} placeholder found for generator '{}' in service '{}'",
            generator_type,
            service.name
        );
        Ok(None)
    }

    /// Lists the names of the presentation layers available for the given service by reading the
    /// `webapi` generator template's step destinations — no hardcoded paths or naming conventions.
    ///
    /// The algorithm finds the first step that contains both `{{ Service }}` and a `{{ Service }}`
    /// sub-segment followed by `.` (e.g. `{{ Service }}.Presentation.WebApi`), derives the root
    /// directory that sits before that segment, and then enumerates real subdirectories there whose
    /// names start with the `<ServiceName>.` prefix and strips it to return the layer name.
    ///
    /// Returns an empty `Vec` when the `webapi` generator is not declared in the base template or
    /// when the presentation root does not exist on disk.
    pub fn list_presentation_layers(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<Vec<String>, AddArtifactError> {
        use n_framework_nfw_core_domain::features::template_management::template_config::TemplateStepAction;

        // Try to load the webapi generator template. If the template does not declare a
        // "webapi" generator, return an empty list gracefully.
        let context = self
            .load_template_context(workspace.clone(), service, GEN_TYPE_WEBAPI)
            .map_err(|e| {
                AddArtifactError::ConfigError(format!(
                    "Failed to load context for webapi generator: {}",
                    e
                ))
            })?;

        for step in context.config.steps() {
            // Collect any path-like string from the step that contains {{ Service }} or {{ Namespace }}
            let path_str: Option<&str> = match &step.action {
                TemplateStepAction::Render { destination, .. } => Some(destination),
                TemplateStepAction::RenderIfAbsent { destination, .. } => Some(destination),
                TemplateStepAction::Inject { destination, .. } => Some(destination),
                TemplateStepAction::RunCommand { command, .. } => Some(command),
                TemplateStepAction::RenderFolder { destination, .. } => Some(destination),
            };

            let Some(raw) = path_str else { continue };

            let namespace = workspace
                .nfw_yaml
                .get("workspace")
                .and_then(|w| w.get("namespace"))
                .and_then(|v| v.as_str())
                .unwrap_or(&service.name);

            // We need a segment that looks like `{{ Service }}.Something` or `{{ Namespace }}.Something`
            // to derive the naming convention.
            let segments: Vec<&str> = raw.split('/').collect();

            let (service_dot_idx, effective_name) = {
                let idx_svc = segments.iter().position(|s| {
                    (s.contains("{{ Service }}.") || s.contains("{{Service}}."))
                        && s.len() > "{{ Service }}.".len()
                });
                let idx_ns = segments.iter().position(|s| {
                    (s.contains("{{ Namespace }}.") || s.contains("{{Namespace}}."))
                        && s.len() > "{{ Namespace }}.".len()
                });
                match (idx_svc, idx_ns) {
                    (Some(i), _) => (i, service.name.as_str()),
                    (None, Some(i)) => (i, namespace),
                    _ => continue,
                }
            };

            // Everything before this segment is the presentation root directory.
            let prefix_segs = &segments[..service_dot_idx];
            // The segment itself tells us what follows the service/namespace name
            let seg = segments[service_dot_idx];
            // Strip the placeholder to get the suffix, e.g. `.Presentation.WebApi`
            let dot_suffix = seg
                .strip_prefix("{{ Service }}")
                .or_else(|| seg.strip_prefix("{{Service}}"))
                .or_else(|| seg.strip_prefix("{{ Namespace }}"))
                .or_else(|| seg.strip_prefix("{{Namespace}}"))
                .unwrap_or("");
            let prefix_to_strip = if let Some(last_dot) = dot_suffix.rfind('.') {
                format!("{}{}", effective_name, &dot_suffix[..=last_dot])
            } else {
                format!("{}{}", effective_name, dot_suffix)
            };
            let base = workspace.workspace_root.join(&service.path);
            let pres_root: std::path::PathBuf =
                prefix_segs.iter().fold(base, |acc, seg| acc.join(seg));

            if !pres_root.is_dir() {
                return Ok(Vec::new());
            }

            let mut layers = Vec::new();
            let entries = std::fs::read_dir(&pres_root).map_err(|e| {
                tracing::warn!(
                    "Failed to read presentation root directory {}: {}",
                    pres_root.display(),
                    e
                );
                AddArtifactError::WorkspaceError(format!(
                    "Failed to read presentation root directory: {}",
                    e
                ))
            })?;
            for entry in entries {
                let entry = entry.map_err(|e| {
                    AddArtifactError::WorkspaceError(format!(
                        "failed to read directory entry in {}: {}",
                        pres_root.display(),
                        e
                    ))
                })?;

                if !entry.path().is_dir() {
                    continue;
                }
                if let Some(name) = entry.file_name().to_str()
                    && name.starts_with(&prefix_to_strip)
                {
                    let layer = name[prefix_to_strip.len()..].to_string();
                    if !layer.is_empty() {
                        layers.push(layer);
                    }
                }
            }
            return Ok(layers);
        }

        Ok(Vec::new())
    }

    /// Resolves the concrete path to a named sub-directory within the feature folder for a given
    /// generator type, by reading that generator's template step destinations.
    ///
    /// For example, given the `entity` template whose destination is:
    /// `src/core/{{ Service }}.Core.Domain/Features/{{ Feature }}/Entities/{{ Name }}.cs`
    /// this method returns the path to `Features/<feature>/Entities` under the service root.
    ///
    /// Returns `Ok(None)` when the template cannot be found or declares no suitable step.
    pub fn resolve_artifact_subdir(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
        generator_type: &str,
        feature: &str,
    ) -> Result<Option<std::path::PathBuf>, AddArtifactError> {
        let Some((features_root, sub_dirs, _file_suffix)) =
            self.resolve_mediator_artifact_root(workspace, service, generator_type)?
        else {
            return Ok(None);
        };

        let mut dir = features_root.join(feature);
        for seg in &sub_dirs {
            dir = dir.join(seg);
        }
        Ok(Some(dir))
    }

    /// Searches features for one that contains a file matching `<name>.<ext_suffix>` inside the
    /// sub-directory derived from `generator_type`'s template step destinations.
    ///
    /// Returns all feature names whose derived sub-directory contains a file whose name starts with
    /// `artifact_name` and ends with `file_suffix` (both derived from the template when not
    /// provided — pass `None` to auto-derive from the template step destination).
    ///
    /// Used by `gen_repository_command_handler` to auto-detect which feature holds a given entity
    /// without hardcoding `"Entities"` or `.cs`.
    pub fn find_artifact_in_features(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
        generator_type: &str,
        artifact_name: &str,
    ) -> Result<Vec<String>, AddArtifactError> {
        use n_framework_nfw_core_domain::features::template_management::template_config::TemplateStepAction;

        let context = self
            .load_template_context(workspace.clone(), service, generator_type)
            .map_err(|e| {
                AddArtifactError::ConfigError(format!(
                    "Failed to load context for generator type {}: {}",
                    generator_type, e
                ))
            })?;

        // Find the first Render step with {{ Feature }} to derive the features root and sub-dirs.
        let mut features_root_opt: Option<(std::path::PathBuf, Vec<String>, String)> = None;
        for step in context.config.steps() {
            let destination = match &step.action {
                TemplateStepAction::Render { destination, .. } => destination,
                _ => continue,
            };
            let segments: Vec<&str> = destination.split('/').collect();
            let Some(fi) = segments
                .iter()
                .position(|s| s.contains("{{ Feature }}") || s.contains("{{Feature}}"))
            else {
                continue;
            };
            let ni = segments[fi + 1..]
                .iter()
                .position(|s| s.contains("{{ Name }}") || s.contains("{{Name}}"))
                .map(|rel| fi + 1 + rel);

            let namespace = workspace
                .nfw_yaml
                .get("workspace")
                .and_then(|w| w.get("namespace"))
                .and_then(|v| v.as_str())
                .unwrap_or(&service.name);

            let prefix: std::path::PathBuf = segments[..fi].iter().fold(
                workspace.workspace_root.join(&service.path),
                |acc, s| {
                    acc.join(
                        s.replace("{{ Service }}", &service.name)
                            .replace("{{Service}}", &service.name)
                            .replace("{{ Namespace }}", namespace)
                            .replace("{{Namespace}}", namespace),
                    )
                },
            );

            let sub_dirs: Vec<String> = if let Some(ni) = ni {
                segments[fi + 1..ni]
                    .iter()
                    .filter(|s| !s.contains("{{"))
                    .map(|s| s.to_string())
                    .collect()
            } else {
                Vec::new()
            };

            let file_suffix = segments
                .last()
                .map(|s| {
                    s.strip_prefix("{{ Name }}")
                        .or_else(|| s.strip_prefix("{{Name}}"))
                        .unwrap_or(s)
                        .to_string()
                })
                .unwrap_or_default();

            features_root_opt = Some((prefix, sub_dirs, file_suffix));
            break;
        }

        let Some((features_root, sub_dirs, file_suffix)) = features_root_opt else {
            tracing::warn!(
                "Could not determine features root from template steps for artifact check in service '{}'",
                service.name
            );
            return Ok(Vec::new());
        };

        if !features_root.is_dir() {
            return Ok(Vec::new());
        }

        let mut matched_features = Vec::new();
        let entries = std::fs::read_dir(&features_root).map_err(|e| {
            tracing::warn!(
                "Failed to read features root directory {}: {}",
                features_root.display(),
                e
            );
            AddArtifactError::WorkspaceError(format!("Failed to read features root: {e}"))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                AddArtifactError::WorkspaceError(format!(
                    "failed to read directory entry in {}: {}",
                    features_root.display(),
                    e
                ))
            })?;

            if !entry.path().is_dir() {
                continue;
            }
            let feature_name = match entry.file_name().into_string() {
                Ok(n) => n,
                Err(_) => continue,
            };
            let mut scan_dir = entry.path();
            for seg in &sub_dirs {
                scan_dir = scan_dir.join(seg);
            }

            if !scan_dir.is_dir() {
                continue;
            }
            let inner_entries = std::fs::read_dir(&scan_dir).map_err(|e| {
                tracing::warn!(
                    "Failed to read artifact sub-directory {}: {}",
                    scan_dir.display(),
                    e
                );
                AddArtifactError::WorkspaceError(format!(
                    "Failed to read artifact sub-directory: {e}"
                ))
            })?;

            for inner_entry in inner_entries {
                let inner_entry = inner_entry.map_err(|e| {
                    AddArtifactError::WorkspaceError(format!(
                        "failed to read directory entry in {}: {}",
                        scan_dir.display(),
                        e
                    ))
                })?;

                if let Some(fname) = inner_entry.file_name().to_str()
                    && fname.starts_with(artifact_name)
                    && fname.ends_with(&file_suffix)
                {
                    matched_features.push(feature_name.clone());
                    break;
                }
            }
        }
        Ok(matched_features)
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
