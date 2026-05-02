use std::path::Path;

use n_framework_nfw_core_domain::features::entity_generation::entities::add_entity_command::AddEntityCommand;
use n_framework_nfw_core_domain::features::entity_generation::entities::entity_schema::{
    EntitySchema, SchemaProperty,
};
use n_framework_nfw_core_domain::features::entity_generation::errors::entity_generation_error::EntityGenerationError;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::general_type::GeneralType;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::service_info::ServiceInfo;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::workspace_context::WorkspaceContext;

use crate::features::entity_generation::abstractions::entity_schema_store::EntitySchemaStore;
use crate::features::entity_generation::services::entity_name_validator::EntityNameValidator;
use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::artifact_generation_service::{
    AddArtifactContext, ArtifactGenerationService, ServiceInfo as TemplateServiceInfo,
    WorkspaceContext as TemplateWorkspaceContext,
};
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;

#[derive(Debug, Clone)]
pub struct AddEntityCommandHandler<W, R, E, S> {
    artifact_service: ArtifactGenerationService<W, R, E>,
    schema_store: S,
}

impl<W, R, E, S> AddEntityCommandHandler<W, R, E, S>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
    S: EntitySchemaStore,
{
    pub fn new(working_dir_provider: W, root_resolver: R, engine: E, schema_store: S) -> Self {
        Self {
            artifact_service: ArtifactGenerationService::new(
                working_dir_provider,
                root_resolver,
                engine,
            ),
            schema_store,
        }
    }

    pub fn handle(
        &self,
        command: &AddEntityCommand,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<EntitySchema, EntityGenerationError> {
        EntityNameValidator::validate(command.entity_name())?;
        self.validate_id_type(command)?;
        self.validate_persistence_module(service)?;

        if let Some(schema_path) = command.from_schema() {
            return self.handle_from_schema(schema_path, command);
        }

        let schema = self.build_schema(command);
        let specs_dir = service
            .path()
            .join("specs")
            .join("features")
            .join(command.feature())
            .join("entities");
        let schema_file = specs_dir.join(format!("{}.yaml", command.entity_name()));

        if self.schema_store.schema_exists(&schema_file) {
            return Err(EntityGenerationError::SchemaFileConflict { path: schema_file });
        }

        self.schema_store.write_schema(&specs_dir, &schema)?;

        if command.is_schema_only() {
            tracing::info!(
                "Schema file created at {}. Skipping template invocation (--schema-only).",
                schema_file.display()
            );
            return Ok(schema);
        }

        self.invoke_template_engine(command, workspace, service)?;

        Ok(schema)
    }

    pub fn get_workspace_context(&self) -> Result<TemplateWorkspaceContext, AddArtifactError> {
        self.artifact_service.get_workspace_context()
    }

    pub fn extract_services(
        &self,
        workspace: &TemplateWorkspaceContext,
    ) -> Result<Vec<TemplateServiceInfo>, AddArtifactError> {
        self.artifact_service.extract_services(workspace)
    }

    pub fn load_template_context(
        &self,
        workspace: TemplateWorkspaceContext,
        service: &TemplateServiceInfo,
        generator_type: &str,
    ) -> Result<AddArtifactContext, AddArtifactError> {
        self.artifact_service
            .load_template_context(workspace, service, generator_type)
    }

    pub fn list_features(
        &self,
        workspace: &TemplateWorkspaceContext,
        service: &TemplateServiceInfo,
    ) -> Result<Vec<String>, AddArtifactError> {
        self.artifact_service.list_features(workspace, service)
    }

    fn validate_id_type(&self, command: &AddEntityCommand) -> Result<(), EntityGenerationError> {
        match command.id_type() {
            GeneralType::Integer | GeneralType::Uuid | GeneralType::String => Ok(()),
            other => Err(EntityGenerationError::UnsupportedIdType {
                id_type: other.to_string(),
            }),
        }
    }

    fn validate_persistence_module(
        &self,
        service: &ServiceInfo,
    ) -> Result<(), EntityGenerationError> {
        if !service.has_module("persistence") {
            return Err(EntityGenerationError::MissingPersistenceModule {
                service_name: service.name().to_owned(),
            });
        }
        Ok(())
    }

    fn build_schema(&self, command: &AddEntityCommand) -> EntitySchema {
        let properties = command
            .properties()
            .iter()
            .map(|p| SchemaProperty {
                name: p.name().to_owned(),
                general_type: p.general_type().clone(),
                nullable: p.nullable(),
            })
            .collect();

        EntitySchema::new(
            command.entity_name().to_owned(),
            command.id_type().clone(),
            command.entity_type(),
            properties,
        )
    }

    fn handle_from_schema(
        &self,
        schema_path: &Path,
        command: &AddEntityCommand,
    ) -> Result<EntitySchema, EntityGenerationError> {
        if !self.schema_store.schema_exists(schema_path) {
            return Err(EntityGenerationError::SchemaFileNotFound {
                path: schema_path.to_path_buf(),
            });
        }

        let schema = self.schema_store.read_schema(schema_path)?;

        if command.is_schema_only() {
            tracing::info!(
                "Schema already exists at {}. Nothing to do (--schema-only + --from-schema).",
                schema_path.display()
            );
        }

        Ok(schema)
    }

    fn invoke_template_engine(
        &self,
        command: &AddEntityCommand,
        _workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<(), EntityGenerationError> {
        let app_workspace = self
            .artifact_service
            .get_workspace_context()
            .map_err(|e| EntityGenerationError::Internal(e.to_string()))?;

        let services = self
            .artifact_service
            .extract_services(&app_workspace)
            .map_err(|e| EntityGenerationError::Internal(e.to_string()))?;

        let app_service = services
            .into_iter()
            .find(|s| s.name() == service.name())
            .ok_or_else(|| EntityGenerationError::ServiceNotFound {
                name: service.name().to_string(),
            })?;

        let artifact_context = self
            .artifact_service
            .load_template_context(app_workspace, &app_service, "entity")
            .map_err(|e| EntityGenerationError::TemplateExecutionError {
                reason: format!("Failed to load template context for entity generator: {e}"),
            })?;

        let mut properties = Vec::new();
        for prop in command.properties() {
            let mut prop_map = serde_json::Map::new();
            prop_map.insert(
                "name".to_string(),
                serde_json::Value::String(prop.name().to_string()),
            );
            prop_map.insert(
                "type".to_string(),
                serde_json::Value::String(prop.general_type().to_string()),
            );
            prop_map.insert(
                "nullable".to_string(),
                serde_json::Value::Bool(prop.nullable()),
            );
            properties.push(serde_json::Value::Object(prop_map));
        }

        let mut params = serde_json::Map::new();
        params.insert(
            "Properties".to_string(),
            serde_json::Value::Array(properties),
        );
        params.insert(
            "IdType".to_string(),
            serde_json::Value::String(command.id_type().to_string()),
        );
        params.insert(
            "EntityType".to_string(),
            serde_json::Value::String(command.entity_type().as_schema_value().to_string()),
        );

        let command_params = Some(serde_json::Value::Object(params));

        self.artifact_service
            .execute_generation(
                command.entity_name(),
                Some(command.feature()),
                &command_params,
                &artifact_context,
            )
            .map_err(|e| EntityGenerationError::TemplateExecutionError {
                reason: format!("Template generation failed: {e}"),
            })?;

        Ok(())
    }
}
