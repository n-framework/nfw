use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::artifact_generation_service::{
    ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;

use super::add_mediator_command::AddMediatorCommand;

#[derive(Debug, Clone)]
pub struct AddMediatorCommandHandler<W, R, E> {
    service: ArtifactGenerationService<W, R, E>,
}

impl<W, R, E> AddMediatorCommandHandler<W, R, E>
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

    pub fn handle(&self, command: &AddMediatorCommand) -> Result<(), AddArtifactError> {
        let workspace = &command.workspace_context;
        let context = self.service.load_template_context(
            workspace.clone(),
            &command.service_info,
            "mediator",
        )?;

        let namespace = self.service.extract_namespace(&workspace.nfw_yaml)?;

        let parameters =
            n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters::new()
                .with_name(&command.service_info.name)
                .map_err(AddArtifactError::InvalidParameter)?
                .with_namespace(namespace)
                .map_err(AddArtifactError::InvalidParameter)?
                .with_service(&command.service_info.name)
                .map_err(AddArtifactError::InvalidParameter)?;

        let output_root = workspace.workspace_root.join(&command.service_info.path);

        self.service
            .engine()
            .execute(
                &context.config,
                &context.template_root,
                &output_root,
                &parameters,
            )
            .map_err(|e| AddArtifactError::ExecutionFailed(Box::new(e)))?;

        self.service.add_service_module(
            &workspace.workspace_root,
            &command.service_info.name,
            "mediator",
        )?;

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
