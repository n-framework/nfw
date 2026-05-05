use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::artifact_generation_service::{
    ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::template_management::services::transaction::{FileTracker, YamlBackup};
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;

use super::add_webapi_command::AddWebApiCommand;

#[derive(Debug, Clone)]
pub struct AddWebApiCommandHandler<W, R, E> {
    service: ArtifactGenerationService<W, R, E>,
}

impl<W, R, E> AddWebApiCommandHandler<W, R, E>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
{
    pub fn new(working_dir_provider: W, root_resolver: R, engine: E) -> Self {
        Self {
            service: ArtifactGenerationService::new(working_dir_provider, root_resolver, engine),
        }
    }

    /// Handles the `add webapi` command workflow.
    pub fn handle(&self, cmd: &AddWebApiCommand) -> Result<(), AddArtifactError> {
        let workspace = cmd.workspace_context();

        if self.service.has_service_module(
            workspace.workspace_root(),
            cmd.service_info().name(),
            AddWebApiCommand::GENERATOR_TYPE,
        )? {
            return Err(AddArtifactError::WorkspaceError(format!(
                "WebAPI module already exists for service '{}'. No changes were made.",
                cmd.service_info().name()
            )));
        }

        let output_root = workspace.workspace_root().join(cmd.service_info().path());

        let context = self.service.load_template_context(
            workspace.clone(),
            cmd.service_info(),
            AddWebApiCommand::GENERATOR_TYPE,
        )?;

        let namespace = self.service.extract_namespace(workspace.nfw_yaml())?;

        let config = cmd.config();
        let mut parameters = TemplateParameters::new()
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

        let yaml_path = workspace.workspace_root().join("nfw.yaml");
        let yaml_backup = YamlBackup::create(&yaml_path)?;

        let file_tracker = FileTracker::new(&output_root).map_err(|e| {
            AddArtifactError::WorkspaceError(format!("Failed to initialize file tracking: {}", e))
        })?;

        self.service
            .engine()
            .execute(
                &context.config,
                &context.template_root,
                &output_root,
                &parameters,
            )
            .map_err(|e| {
                tracing::error!(
                    service = %cmd.service_info().name(),
                    error = ?e,
                    "Template execution failed for service '{}', rolling back",
                    cmd.service_info().name()
                );
                let _ = file_tracker.cleanup_created_files();
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
                let _ = file_tracker.cleanup_created_files();
                let _ = yaml_backup.restore();
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
