use crate::features::generator_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
use crate::features::generator_management::services::artifact_generation_service::{
    ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::generator_management::services::generator_engine::GeneratorEngine;
use crate::features::generator_management::services::transaction::{FileTracker, YamlBackup};
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;

use super::add_mediator_command::AddMediatorCommand;

use crate::features::generator_management::constants::generation::errors::{
    ERR_FILE_CLEANUP, ERR_INIT_TRACKER, ERR_YAML_BACKUP,
};
use crate::features::generator_management::constants::{generation, workspace};

#[derive(Debug, Clone)]
pub struct AddMediatorCommandHandler<W, R, E> {
    service: ArtifactGenerationService<W, R, E>,
}

impl<W, R, E> AddMediatorCommandHandler<W, R, E>
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

    pub fn handle(&self, command: &AddMediatorCommand) -> Result<(), AddArtifactError> {
        let workspace = command.workspace_context();
        let context = self.service.load_generator_context(
            workspace.clone(),
            command.service_info(),
            AddMediatorCommand::GENERATOR_TYPE,
        )?;

        let namespace = self.service.extract_namespace(workspace.nfw_yaml())?;

        let parameters = GeneratorParameters::new()
            .with_name(command.service_info().name())
            .map_err(AddArtifactError::InvalidParameter)?
            .with_namespace(namespace)
            .map_err(AddArtifactError::InvalidParameter)?
            .with_service(command.service_info().name())
            .map_err(AddArtifactError::InvalidParameter)?;

        let mut parameters = parameters;
        parameters
            .insert_value(
                generation::PRESENTATION_LAYER.to_string(),
                serde_json::Value::String(command.presentation_layer().to_string()),
            )
            .map_err(AddArtifactError::InvalidParameter)?;

        let output_root = workspace
            .workspace_root()
            .join(command.service_info().path());

        let yaml_path = workspace.workspace_root().join(workspace::METADATA_FILE);
        let yaml_backup = YamlBackup::create(&yaml_path)?;

        let file_tracker = FileTracker::new(&output_root).map_err(|e| {
            AddArtifactError::WorkspaceError(format!("{}: {}", ERR_INIT_TRACKER, e))
        })?;

        self.service
            .engine()
            .execute(
                context.config(),
                context.generator_root(),
                &output_root,
                &parameters,
            )
            .map_err(|e| {
                tracing::error!(
                    service = %command.service_info().name(),
                    error = ?e,
                    "Generator execution failed for service '{}', rolling back",
                    command.service_info().name()
                );
                if let Err(cleanup_err) = file_tracker.cleanup_created_files() {
                    tracing::error!("{}: {:?}", ERR_FILE_CLEANUP, cleanup_err);
                }
                AddArtifactError::ExecutionFailed(Box::new(e))
            })?;

        self.service
            .add_service_module(
                workspace.workspace_root(),
                command.service_info().name(),
                AddMediatorCommand::GENERATOR_TYPE,
            )
            .map_err(|e| {
                tracing::error!(
                    service = %command.service_info().name(),
                    error = ?e,
                    "Failed to add service module for '{}', rolling back",
                    command.service_info().name()
                );
                if let Err(cleanup_err) = file_tracker.cleanup_created_files() {
                    tracing::error!("{}: {:?}", ERR_FILE_CLEANUP, cleanup_err);
                }
                if let Err(restore_err) = yaml_backup.restore() {
                    tracing::error!("{}: {:?}", ERR_YAML_BACKUP, restore_err);
                }
                e
            })?;

        Ok(())
    }

    pub fn get_workspace_context(&self) -> Result<WorkspaceContext, AddArtifactError> {
        self.service.get_workspace_context()
    }

    pub fn extract_services(
        &self,
        workspace: &WorkspaceContext,
    ) -> Result<Vec<ServiceInfo>, AddArtifactError> {
        self.service.extract_services(workspace)
    }

    /// Lists the presentation layer names available for the given service by reading the `webapi`
    /// generator generator's step destinations — no hardcoded paths or naming conventions.
    pub fn list_presentation_layers(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<Vec<String>, AddArtifactError> {
        self.service.list_presentation_layers(workspace, service)
    }
}
