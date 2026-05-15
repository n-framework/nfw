use crate::features::generator_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
use crate::features::generator_management::services::artifact_generation_service::{
    AddArtifactContext, ArtifactGenerationService,
};
use crate::features::generator_management::services::generator_engine::GeneratorEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use tracing::info;

use crate::features::generator_management::models::generator_error::GeneratorError;

use super::gen_endpoint_command::GenEndpointCommand;
use crate::features::generator_management::constants::generation::DEFAULT_FEATURE_NAME;
use crate::features::generator_management::constants::yaml_keys;

#[derive(Debug, Clone)]
pub struct GenEndpointCommandHandler<W, R, E> {
    pub(crate) service: ArtifactGenerationService<W, R, E>,
}

impl<W, R, E> GenEndpointCommandHandler<W, R, E>
where
    W: WorkingDirectoryProvider,
    R: GeneratorRootResolver,
    E: GeneratorEngine,
{
    pub fn new(working_dir_provider: W, root_resolver: R, engine: E) -> Self {
        Self {
            service: ArtifactGenerationService::new(working_dir_provider, root_resolver, engine),
        }
    }

    pub fn handle(&self, command: GenEndpointCommand) -> Result<String, AddArtifactError> {
        info!("Handling GenEndpointCommand for name: {}", command.name());

        let workspace_context = command.context().workspace.clone();

        let services = self.service.extract_services(&workspace_context)?;
        let service = services
            .into_iter()
            .find(|s| s.path() == command.context().service_path().to_str().unwrap_or(""))
            .ok_or_else(|| {
                AddArtifactError::ConfigError(format!(
                    "Service not found for path: {}",
                    command.context().service_path().display()
                ))
            })?;

        // Validate required modules
        self.service.validate_required_modules(
            &command.context().config,
            command.context().workspace.nfw_yaml(),
            &command.context().service_path,
        )?;

        let feature_name = command.feature().unwrap_or(DEFAULT_FEATURE_NAME);

        let mediator_sources = command.context().config().mediator_sources().to_vec();

        if !mediator_sources.is_empty() && command.attach_to_mediator() {
            // Validate that the mediator artifact (Command or Query) exists for the given name,
            // deriving the expected location from the command/query generator step destinations.
            let mediator_ok = self.mediator_artifact_exists(
                &workspace_context,
                &service,
                feature_name,
                command.name(),
                &mediator_sources,
            )?;
            if !mediator_ok {
                return Err(AddArtifactError::ExecutionFailed(Box::new(
                    GeneratorError::io(
                        format!(
                            "No Command or Query artifact found for '{}' in feature '{}'. \
                             Generate the command or query first.",
                            command.name(),
                            feature_name
                        ),
                        std::path::PathBuf::new(),
                    ),
                )));
            }
        }

        // Validate that the endpoint output file does not already exist, deriving the expected
        // destination from the endpoint generator step.
        self.assert_endpoint_not_exists(&command, &service)?;

        let context = AddArtifactContext {
            workspace: workspace_context.clone(),
            generator_root: command.context().generator_root().clone(),
            config: command.context().config().clone(),
            service_name: command.context().service_name().to_string(),
            service_path: command.context().service_path().clone(),
        };

        let mut final_params = command
            .params()
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        if let Some(obj) = final_params.as_object_mut() {
            obj.insert(
                "OperationType".to_string(),
                serde_json::json!(command.operation_type().to_string()),
            );
            obj.insert(
                "AttachToMediator".to_string(),
                serde_json::json!(command.attach_to_mediator()),
            );
        }

        self.service.execute_generation(
            command.name(),
            command.feature(),
            &Some(final_params),
            &context,
        )?;

        Ok(format!(
            "Successfully generated endpoint {}",
            command.name()
        ))
    }

    /// Checks whether a Command or Query artifact already exists for the given name and feature by
    /// reading the mediator source generator types declared in the endpoint generator config and
    /// probing each one's step destinations — no hardcoded paths or generator type names.
    fn mediator_artifact_exists(
        &self,
        workspace_context: &crate::features::generator_management::services::artifact_generation_service::WorkspaceContext,
        service: &crate::features::generator_management::services::artifact_generation_service::ServiceInfo,
        feature_name: &str,
        artifact_name: &str,
        mediator_sources: &[String],
    ) -> Result<bool, AddArtifactError> {
        for generator_type in mediator_sources {
            match self.service.resolve_mediator_artifact_root(
                workspace_context,
                service,
                generator_type,
            ) {
                Ok(Some((features_root, sub_dirs, file_suffix))) => {
                    // Path: features_root / feature / sub_dirs... / {{ Name }}{{ suffix }}
                    let mut candidate = features_root.join(feature_name);
                    for seg in &sub_dirs {
                        candidate = candidate.join(seg);
                    }
                    // The generator nests the artifact under a {{ Name }} sub-directory as well.
                    candidate = candidate
                        .join(artifact_name)
                        .join(format!("{}{}", artifact_name, file_suffix));
                    if candidate.is_file() {
                        return Ok(true);
                    }
                }
                Ok(None) => {
                    tracing::debug!(
                        "Generator step destination does not resolve properly for generator '{}'",
                        generator_type
                    );
                }
                Err(e) => {
                    tracing::error!(
                        "Generator resolution failure for mediator artifact '{}': {}",
                        generator_type,
                        e
                    );
                    return Err(e);
                }
            }
        }
        Ok(false)
    }

    /// Asserts that the endpoint output file does not already exist, resolving its path from the
    /// endpoint generator's step destination pattern instead of a hardcoded path.
    fn assert_endpoint_not_exists(
        &self,
        command: &GenEndpointCommand,
        service: &crate::features::generator_management::services::artifact_generation_service::ServiceInfo,
    ) -> Result<(), AddArtifactError> {
        use n_framework_nfw_core_domain::features::generator_management::generator_config::GeneratorStepAction;

        let feature_name = command.feature().unwrap_or(DEFAULT_FEATURE_NAME);

        for step in command.context().config().steps() {
            let destination = match &step.action {
                GeneratorStepAction::Render { destination, .. } => destination,
                _ => continue,
            };

            let namespace = command
                .context()
                .workspace
                .nfw_yaml()
                .get(yaml_keys::WORKSPACE)
                .and_then(|w| w.get("namespace"))
                .and_then(|v| v.as_str())
                .unwrap_or(&service.name);

            let resolved = destination
                .replace("{{ Service }}", &service.name)
                .replace("{{Service}}", &service.name)
                .replace("{{ Namespace }}", namespace)
                .replace("{{Namespace}}", namespace)
                .replace("{{ Feature }}", feature_name)
                .replace("{{Feature}}", feature_name)
                .replace("{{ Name }}", command.name())
                .replace("{{Name}}", command.name());

            let endpoint_file = command
                .context()
                .workspace_root()
                .join(command.context().service_path())
                .join(&resolved);

            if endpoint_file.is_file() {
                return Err(AddArtifactError::ExecutionFailed(Box::new(
                    GeneratorError::io(
                        format!(
                            "Target endpoint file already exists: {}",
                            endpoint_file.display()
                        ),
                        std::path::PathBuf::new(),
                    ),
                )));
            }
        }

        Ok(())
    }

    pub fn get_workspace_context(&self) -> Result<crate::features::generator_management::services::artifact_generation_service::WorkspaceContext, AddArtifactError>{
        self.service.get_workspace_context()
    }

    pub fn has_service_module(
        &self,
        workspace_context: &crate::features::generator_management::services::artifact_generation_service::WorkspaceContext,
        service: &crate::features::generator_management::services::artifact_generation_service::ServiceInfo,
        module_name: &str,
    ) -> Result<bool, AddArtifactError> {
        self.service.has_service_module(
            workspace_context.workspace_root(),
            &service.name,
            module_name,
        )
    }

    /// Lists mediator artifacts (Commands or Queries) in the given feature folder, deriving the
    /// search directory and file suffix from the generator step destinations — no hardcoded paths.
    pub fn get_mediator_items(
        &self,
        workspace_context: &crate::features::generator_management::services::artifact_generation_service::WorkspaceContext,
        service: &crate::features::generator_management::services::artifact_generation_service::ServiceInfo,
        feature: &str,
        is_query: bool,
    ) -> Result<Vec<String>, AddArtifactError> {
        let generator_type = if is_query { "query" } else { "command" };

        let Some((features_root, sub_dirs, file_suffix)) = self
            .service
            .resolve_mediator_artifact_root(workspace_context, service, generator_type)?
        else {
            return Ok(Vec::new());
        };

        // Build the concrete scan directory: features_root / feature / sub_dirs...
        let mut scan_dir = features_root.join(feature);
        for seg in &sub_dirs {
            scan_dir = scan_dir.join(seg);
        }

        if !scan_dir.is_dir() {
            return Ok(Vec::new());
        }

        let mut items = Vec::new();
        // The generator nests each artifact under its own {{ Name }} sub-directory, so scan one
        // level of sub-directories and look for a file matching the suffix inside each one.
        let entries = std::fs::read_dir(&scan_dir).map_err(|e| {
            AddArtifactError::WorkspaceError(format!(
                "failed to read directory {}: {}",
                scan_dir.display(),
                e
            ))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                AddArtifactError::WorkspaceError(format!("failed to read directory entry: {}", e))
            })?;

            let entry_path = entry.path();
            if entry_path.is_dir() {
                // Sub-directory named after the artifact (e.g. `CreateProduct/`)
                let inner = std::fs::read_dir(&entry_path).map_err(|e| {
                    AddArtifactError::WorkspaceError(format!(
                        "failed to read sub-directory {}: {}",
                        entry_path.display(),
                        e
                    ))
                })?;

                for inner_entry in inner {
                    let inner_entry = inner_entry.map_err(|e| {
                        AddArtifactError::WorkspaceError(format!(
                            "failed to read sub-directory entry: {}",
                            e
                        ))
                    })?;

                    if let Some(name) = inner_entry.file_name().to_str()
                        && name.ends_with(&file_suffix)
                    {
                        let class_name = &name[..name.len() - file_suffix.len()];
                        if !class_name.is_empty() {
                            items.push(class_name.to_string());
                        }
                    }
                }
            } else if let Some(name) = entry.file_name().to_str() {
                // Flat layout — file is directly in the scan dir
                if name.ends_with(&file_suffix) {
                    let class_name = &name[..name.len() - file_suffix.len()];
                    if !class_name.is_empty() {
                        items.push(class_name.to_string());
                    }
                }
            }
        }
        Ok(items)
    }

    pub fn generate_mediator_item(
        &self,
        name: &str,
        feature: Option<&str>,
        params: &Option<serde_json::Value>,
        context: &AddArtifactContext,
    ) -> Result<(), AddArtifactError> {
        self.service
            .execute_generation(name, feature, params, context)
    }

    pub fn extract_services(
        &self,
        workspace_context: &crate::features::generator_management::services::artifact_generation_service::WorkspaceContext,
    ) -> Result<Vec<crate::features::generator_management::services::artifact_generation_service::ServiceInfo>, AddArtifactError>{
        self.service.extract_services(workspace_context)
    }

    pub fn load_generator_context(
        &self,
        generator_id: &str,
        service: &crate::features::generator_management::services::artifact_generation_service::ServiceInfo,
        workspace_context: &crate::features::generator_management::services::artifact_generation_service::WorkspaceContext,
    ) -> Result<AddArtifactContext, AddArtifactError> {
        self.service
            .load_generator_context(workspace_context.clone(), service, generator_id)
    }

    pub fn validate_required_modules(
        &self,
        context: &AddArtifactContext,
    ) -> Result<(), AddArtifactError> {
        self.service.validate_required_modules(
            context.config(),
            context.nfw_yaml(),
            context.service_path(),
        )
    }

    pub fn list_features(
        &self,
        workspace_context: &crate::features::generator_management::services::artifact_generation_service::WorkspaceContext,
        service: &crate::features::generator_management::services::artifact_generation_service::ServiceInfo,
    ) -> Result<Vec<String>, AddArtifactError> {
        self.service.list_features(workspace_context, service)
    }

    /// Returns true if the service has at least one mediator source available — determined by
    /// loading each source generator's generator config and checking whether the service satisfies
    /// its `required_modules`. This replaces any hardcoded module-name check in the CLI layer.
    pub fn has_mediator_sources(
        &self,
        workspace_context: &crate::features::generator_management::services::artifact_generation_service::WorkspaceContext,
        service: &crate::features::generator_management::services::artifact_generation_service::ServiceInfo,
        mediator_sources: &[String],
    ) -> bool {
        for generator_type in mediator_sources {
            if let Ok(ctx) = self.service.load_generator_context(
                workspace_context.clone(),
                service,
                generator_type,
            ) && self
                .service
                .validate_required_modules(ctx.config(), ctx.nfw_yaml(), ctx.service_path())
                .is_ok()
            {
                return true;
            }
        }
        false
    }
}
