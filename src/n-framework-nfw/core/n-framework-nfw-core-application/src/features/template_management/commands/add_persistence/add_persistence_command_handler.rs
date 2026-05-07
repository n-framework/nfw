use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::artifact_generation_service::{
    ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::template_management::services::transaction::{FileTracker, YamlBackup};
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;

use super::add_persistence_command::AddPersistenceCommand;

use crate::features::template_management::constants::generation::PRESENTATION_LAYER;
use crate::features::template_management::constants::generation::errors::{
    ERR_FILE_CLEANUP, ERR_INIT_TRACKER, ERR_YAML_BACKUP,
};

#[derive(Debug, Clone)]
pub struct AddPersistenceCommandHandler<W, R, E> {
    service: ArtifactGenerationService<W, R, E>,
}

impl<W, R, E> AddPersistenceCommandHandler<W, R, E>
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

    /// Handles the `add persistence` command workflow.
    ///
    /// ## Workflow Context
    /// 1. Extracts variables required for rendering template content and names, including identifying target service properties.
    /// 2. Performs a robust template resolution algorithm to locate the appropriate templates on disk or fallback paths.
    /// 3. Validates naming rules matching NFramework identifiers against CLI payload properties.
    /// 4. Executes code generation using the templating engine.
    pub fn handle(&self, cmd: &AddPersistenceCommand) -> Result<(), AddArtifactError> {
        let workspace = cmd.workspace_context();
        let context = self.service.load_template_context(
            workspace.clone(),
            cmd.service_info(),
            AddPersistenceCommand::GENERATOR_TYPE,
        )?;

        let namespace = self.service.extract_namespace(workspace.nfw_yaml())?;

        let parameters = TemplateParameters::new()
            .with_name(cmd.service_info().name())
            .map_err(AddArtifactError::InvalidParameter)?
            .with_namespace(namespace)
            .map_err(AddArtifactError::InvalidParameter)?
            .with_service(cmd.service_info().name())
            .map_err(AddArtifactError::InvalidParameter)?;

        let mut parameters = parameters;
        parameters
            .insert_value(
                PRESENTATION_LAYER.to_string(),
                serde_json::Value::String(cmd.presentation_layer().to_string()),
            )
            .map_err(AddArtifactError::InvalidParameter)?;

        let output_root = workspace.workspace_root().join(cmd.service_info().path());

        let yaml_path = workspace.workspace_root().join("nfw.yaml");
        let yaml_backup = YamlBackup::create(&yaml_path)?;

        let file_tracker = FileTracker::new(&output_root).map_err(|e| {
            AddArtifactError::WorkspaceError(format!("{}: {}", ERR_INIT_TRACKER, e))
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
                if let Err(cleanup_err) = file_tracker.cleanup_created_files() {
                    tracing::error!("{}: {:?}", ERR_FILE_CLEANUP, cleanup_err);
                }
                AddArtifactError::ExecutionFailed(Box::new(e))
            })?;

        self.service
            .add_service_module(
                workspace.workspace_root(),
                cmd.service_info().name(),
                AddPersistenceCommand::GENERATOR_TYPE,
            )
            .map_err(|e| {
                tracing::error!(
                    service = %cmd.service_info().name(),
                    error = ?e,
                    "Failed to add service module for '{}', rolling back",
                    cmd.service_info().name()
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
}

#[cfg(test)]
#[path = "add_persistence_command_handler.tests.rs"]
mod tests;
