use std::fs;

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

    pub fn handle(
        &self,
        command: &AddMediatorCommand,
        workspace: WorkspaceContext,
        selected_service: &ServiceInfo,
    ) -> Result<(), AddArtifactError> {
        let context =
            self.service
                .load_template_context(workspace.clone(), selected_service, "mediator")?;

        let namespace = self.service.extract_namespace(&workspace.nfw_yaml)?;

        let parameters =
            n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters::new()
                .with_name(&command.service_name)
                .map_err(AddArtifactError::InvalidParameter)?
                .with_namespace(namespace)
                .map_err(AddArtifactError::InvalidParameter)?;

        let output_root = workspace.workspace_root.join(&selected_service.path);

        self.service
            .engine()
            .execute(
                &context.config,
                &context.template_root,
                &output_root,
                &parameters,
            )
            .map_err(AddArtifactError::ExecutionFailed)?;

        self.update_nfw_yaml_modules(&workspace, &selected_service.name, "mediator")?;

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

    fn update_nfw_yaml_modules(
        &self,
        workspace: &WorkspaceContext,
        service_name: &str,
        module_name: &str,
    ) -> Result<(), AddArtifactError> {
        let nfw_yaml_path = workspace.workspace_root.join("nfw.yaml");
        let content = fs::read_to_string(&nfw_yaml_path).map_err(|e| {
            AddArtifactError::WorkspaceError(format!("failed to read nfw.yaml: {e}"))
        })?;

        let mut yaml: serde_yaml::Value = serde_yaml::from_str(&content)
            .map_err(|e| AddArtifactError::WorkspaceError(format!("invalid nfw.yaml: {e}")))?;

        let service_key = serde_yaml::Value::String(service_name.to_string());
        let modules_key = serde_yaml::Value::String("modules".to_string());
        let module_value = serde_yaml::Value::String(module_name.to_string());

        if let Some(details) = yaml
            .get_mut("services")
            .and_then(|s| s.as_mapping_mut())
            .and_then(|services| services.get_mut(&service_key))
            .and_then(|d| d.as_mapping_mut())
        {
            let modules = details
                .entry(modules_key)
                .or_insert_with(|| serde_yaml::Value::Sequence(Vec::new()));

            if let Some(seq) = modules.as_sequence_mut()
                && !seq.contains(&module_value)
            {
                seq.push(module_value);
            }
        }

        let output = serde_yaml::to_string(&yaml).map_err(|e| {
            AddArtifactError::WorkspaceError(format!("failed to serialize nfw.yaml: {e}"))
        })?;

        fs::write(&nfw_yaml_path, output).map_err(|e| {
            AddArtifactError::WorkspaceError(format!("failed to write nfw.yaml: {e}"))
        })?;

        Ok(())
    }
}
