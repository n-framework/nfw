use crate::features::entity_generation::abstractions::entity_schema_store::EntitySchemaStore;
use crate::features::generator_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
use crate::features::generator_management::services::artifact_generation_service::{
    AddArtifactContext, ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::generator_management::services::generator_engine::GeneratorEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::global_constants::GlobalConstants;

use super::gen_crud_command::GenCrudCommand;

#[derive(Debug, Clone)]
pub struct GenCrudCommandHandler<W, R, E, S> {
    service: ArtifactGenerationService<W, R, E>,
    schema_store: S,
}

impl<W, R, E, S> GenCrudCommandHandler<W, R, E, S>
where
    W: WorkingDirectoryProvider,
    R: GeneratorRootResolver,
    E: GeneratorEngine,
    S: EntitySchemaStore,
{
    pub fn new(working_dir_provider: W, root_resolver: R, engine: E, schema_store: S) -> Self {
        Self {
            service: ArtifactGenerationService::new(working_dir_provider, root_resolver, engine),
            schema_store,
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
        let service_root = workspace.workspace_root().join(service.path());
        let domain_path = service_root
            .join("src")
            .join("core")
            .join(format!("{}.Core.Domain", service.name()));

        let entity_file = format!("{}.cs", entity_name);

        // Check flat layout: Entities/{Name}.cs
        if domain_path.join("Entities").join(&entity_file).exists() {
            return Ok(true);
        }

        // Check feature-based layout: Features/*/Entities/{Name}.cs
        let features_dir = domain_path.join("Features");
        if features_dir.is_dir()
            && let Ok(entries) = std::fs::read_dir(&features_dir)
        {
            for entry in entries.flatten() {
                if entry.path().is_dir()
                    && entry.path().join("Entities").join(&entity_file).exists()
                {
                    return Ok(true);
                }
            }
        }

        Ok(false)
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

    pub fn handle(&self, command: &GenCrudCommand) -> Result<(), AddArtifactError> {
        let mut final_params = command
            .params
            .clone()
            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));

        // Load entity schema to pass properties down to the templates
        let specs_dir = command
            .context
            .workspace
            .workspace_root()
            .join(&command.context.service_path)
            .join(GlobalConstants::NFW_DIR)
            .join(GlobalConstants::ENTITIES_DIR);

        let schema_file = specs_dir.join(format!("{}.yaml", command.name));

        if self.schema_store.schema_exists(&schema_file) {
            match self.schema_store.read_schema(&schema_file) {
                Ok(schema) => {
                    let mut properties = Vec::new();
                    for prop in schema.properties() {
                        let mut prop_map = serde_json::Map::new();
                        prop_map.insert(
                            "name".to_string(),
                            serde_json::Value::String(prop.name().to_string()),
                        );
                        prop_map.insert(
                            "type".to_string(),
                            serde_json::Value::String(prop.general_type().to_csharp_type().to_string()),
                        );
                        prop_map.insert(
                            "nullable".to_string(),
                            serde_json::Value::Bool(prop.nullable()),
                        );
                        properties.push(serde_json::Value::Object(prop_map));
                    }

                    if let Some(obj) = final_params.as_object_mut() {
                        obj.insert(
                            "Properties".to_string(),
                            serde_json::Value::Array(properties),
                        );
                        obj.insert(
                            "IdType".to_string(),
                            serde_json::Value::String(schema.id_type().to_csharp_type().to_string()),
                        );
                        obj.insert(
                            "EntityType".to_string(),
                            serde_json::Value::String(schema.entity_type().to_string()),
                        );
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to read entity schema for '{}': {}", command.name, e)
                }
            }
        } else {
            tracing::warn!("Entity schema file not found at {}", schema_file.display());
            // Insert default IdType if missing
            if let Some(obj) = final_params.as_object_mut()
                && !obj.contains_key("IdType")
            {
                obj.insert(
                    "IdType".to_string(),
                    serde_json::Value::String("Uuid".to_string()),
                );
            }
        }

        self.service.execute_generation(
            &command.name,
            command.feature.as_deref(),
            &Some(final_params),
            &command.context,
        )
    }
}
