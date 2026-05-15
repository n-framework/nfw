use crate::features::generator_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
use crate::features::generator_management::services::artifact_generation_service::{
    ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::generator_management::services::generator_engine::GeneratorEngine;
use crate::features::generator_management::services::transaction::{FileTracker, YamlBackup};
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;
use std::path::Path;

use super::add_webapi_command::AddWebApiCommand;
use crate::features::generator_management::constants::generation::errors::{
    ERR_FILE_CLEANUP, ERR_INIT_TRACKER, ERR_MODULE_EXISTS, ERR_YAML_BACKUP,
};
use crate::features::generator_management::constants::workspace;

#[derive(Debug, Clone)]
pub struct AddWebApiCommandHandler<W, R, E> {
    service: ArtifactGenerationService<W, R, E>,
}

impl<W, R, E> AddWebApiCommandHandler<W, R, E>
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

    /// Handles the `add webapi` command workflow.
    pub fn handle(&self, cmd: &AddWebApiCommand) -> Result<(), AddArtifactError> {
        let workspace = cmd.workspace_context();

        let context = self.service.load_generator_context(
            workspace.clone(),
            cmd.service_info(),
            AddWebApiCommand::GENERATOR_TYPE,
        )?;

        self.service.validate_required_modules(
            context.config(),
            workspace.nfw_yaml(),
            Path::new(cmd.service_info().path()),
        )?;

        if self.service.has_service_module(
            workspace.workspace_root(),
            cmd.service_info().name(),
            AddWebApiCommand::GENERATOR_TYPE,
        )? {
            return Err(AddArtifactError::WorkspaceError(format!(
                "WebAPI {} '{}'. No changes were made.",
                ERR_MODULE_EXISTS,
                cmd.service_info().name()
            )));
        }

        let output_root = workspace.workspace_root().join(cmd.service_info().path());

        let namespace = self.service.extract_namespace(workspace.nfw_yaml())?;

        let config = cmd.config();
        let mut parameters = GeneratorParameters::new()
            .with_name(cmd.service_info().name())
            .map_err(AddArtifactError::InvalidParameter)?
            .with_namespace(namespace)
            .map_err(AddArtifactError::InvalidParameter)?
            .with_service(cmd.service_info().name())
            .map_err(AddArtifactError::InvalidParameter)?;

        parameters
            .insert("UseOpenApi", config.use_openapi.to_string())
            .map_err(AddArtifactError::InvalidParameter)?;
        parameters
            .insert("UseHealthChecks", config.use_health_checks.to_string())
            .map_err(AddArtifactError::InvalidParameter)?;
        parameters
            .insert("UseCors", config.use_cors.to_string())
            .map_err(AddArtifactError::InvalidParameter)?;
        parameters
            .insert("UseProblemDetails", config.use_problem_details.to_string())
            .map_err(AddArtifactError::InvalidParameter)?;

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
                    service = %cmd.service_info().name(),
                    error = ?e,
                    "Generator execution failed for service '{}', rolling back",
                    cmd.service_info().name()
                );
                let rollback_err = file_tracker.cleanup_created_files().err();
                if let Some(cleanup_err) = rollback_err {
                    tracing::error!("{}: {:?}", ERR_FILE_CLEANUP, cleanup_err);
                    return AddArtifactError::WorkspaceError(format!(
                        "Generator execution failed AND rollback failed. Original error: {}. Rollback error: {}",
                        e, cleanup_err
                    ));
                }
                AddArtifactError::ExecutionFailed(Box::new(e))
            })?;

        self.service
            .add_service_module(
                workspace.workspace_root(),
                cmd.service_info().name(),
                AddWebApiCommand::GENERATOR_TYPE,
            )
            .map_err(|e| {
                tracing::error!(
                    service = %cmd.service_info().name(),
                    error = ?e,
                    "Failed to add service module for '{}', rolling back",
                    cmd.service_info().name()
                );
                let cleanup_res = file_tracker.cleanup_created_files();
                let restore_res = yaml_backup.restore();

                if let Err(cleanup_err) = cleanup_res {
                    tracing::error!("{}: {:?}", ERR_FILE_CLEANUP, cleanup_err);
                    return AddArtifactError::WorkspaceError(format!(
                        "Rollback failed during cleanup after module update error. Original error: {}. Cleanup error: {}",
                        e, cleanup_err
                    ));
                }
                if let Err(restore_err) = restore_res {
                    tracing::error!("{}: {:?}", ERR_YAML_BACKUP, restore_err);
                    return AddArtifactError::WorkspaceError(format!(
                        "Rollback failed during YAML restore after module update error. Original error: {}. Restore error: {}",
                        e, restore_err
                    ));
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
}

#[cfg(test)]
#[path = "add_webapi_command_handler.tests.rs"]
mod tests;
