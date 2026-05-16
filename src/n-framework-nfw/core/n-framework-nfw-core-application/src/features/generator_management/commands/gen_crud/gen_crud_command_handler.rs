use crate::features::generator_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
use crate::features::generator_management::services::artifact_generation_service::{
    AddArtifactContext, ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::generator_management::services::generator_engine::GeneratorEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;

use super::gen_crud_command::GenCrudCommand;

#[derive(Debug, Clone)]
pub struct GenCrudCommandHandler<W, R, E> {
    service: ArtifactGenerationService<W, R, E>,
}

impl<W, R, E> GenCrudCommandHandler<W, R, E>
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

    pub fn get_workspace_context(&self) -> Result<WorkspaceContext, AddArtifactError> {
        self.service.get_workspace_context()
    }

    pub fn extract_services(
        &self,
        workspace: &WorkspaceContext,
    ) -> Result<Vec<ServiceInfo>, AddArtifactError> {
        self.service.extract_services(workspace)
    }

    pub fn list_features(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<Vec<String>, AddArtifactError> {
        self.service.list_features(workspace, service)
    }

    pub fn load_generator_context(
        &self,
        workspace: WorkspaceContext,
        service: &ServiceInfo,
        generator_type: &str,
    ) -> Result<AddArtifactContext, AddArtifactError> {
        self.service
            .load_generator_context(workspace, service, generator_type)
    }

    // T003: Implement initial argument validation logic
    pub fn validate_entity_identifier(&self, entity_name: &str) -> Result<(), AddArtifactError> {
        if entity_name.is_empty() {
            return Err(AddArtifactError::InvalidIdentifier(
                "Entity name cannot be empty".into(),
            ));
        }

        let first_char = entity_name.chars().next().unwrap();
        if !first_char.is_alphabetic() && first_char != '_' {
            return Err(AddArtifactError::InvalidIdentifier(format!(
                "Entity name '{}' must start with a letter or underscore",
                entity_name
            )));
        }

        if !entity_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(AddArtifactError::InvalidIdentifier(format!(
                "Entity name '{}' can only contain alphanumeric characters and underscores",
                entity_name
            )));
        }

        Ok(())
    }

    // T004: Workspace checking logic to verify Entity exists
    pub fn check_entity_exists(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
        entity_name: &str,
    ) -> Result<bool, AddArtifactError> {
        let domain_path = workspace
            .workspace_root()
            .join("src")
            .join("core")
            .join(format!("{}.Domain", service.name()));

        let entity_path = domain_path
            .join("Entities")
            .join(format!("{}.cs", entity_name));

        Ok(entity_path.exists())
    }

    // T005: Artifact conflict detection
    pub fn check_artifacts_exist(
        &self,
        _workspace: &WorkspaceContext,
        _service: &ServiceInfo,
        _entity_name: &str,
        _feature: Option<&str>,
    ) -> Result<bool, AddArtifactError> {
        // TODO: Detailed collision checking in Phase 2
        // Return false for now to permit generation
        Ok(false)
    }

    pub fn handle(&self, _command: &GenCrudCommand) -> Result<(), AddArtifactError> {
        // TODO: Phase 3 - Orchestration
        Ok(())
    }
}
