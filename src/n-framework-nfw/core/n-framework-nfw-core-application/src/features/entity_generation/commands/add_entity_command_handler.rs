use std::path::Path;

use n_framework_nfw_core_domain::features::entity_generation::entities::add_entity_command::AddEntityCommand;
use n_framework_nfw_core_domain::features::entity_generation::entities::entity_schema::EntitySchema;
use n_framework_nfw_core_domain::features::entity_generation::errors::entity_generation_error::EntityGenerationError;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::general_type::GeneralType;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::global_constants::GlobalConstants;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::service_info::ServiceInfo;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::workspace_context::WorkspaceContext;

use crate::features::entity_generation::abstractions::entity_schema_store::EntitySchemaStore;
use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::artifact_generation_service::{
    AddArtifactContext, ArtifactGenerationService, ServiceInfo as TemplateServiceInfo,
    WorkspaceContext as TemplateWorkspaceContext,
};
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use std::path::PathBuf;

const PARAM_PROPERTIES: &str = "Properties";
const PARAM_ID_TYPE: &str = "IdType";
const PARAM_ENTITY_TYPE: &str = "EntityType";

/// Orchestrates the process of adding a new entity.
///
/// This handler coordinates several steps:
/// 1. Validating the feature directory and persistence module.
/// 2. Handling schema creation/reading (including `--from-schema`).
/// 3. Invoking the template engine to generate code artifacts.
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

    /// Executes the command to add a new entity.
    ///
    /// Returns the generated `EntitySchema` and the path to the schema file.
    pub fn handle(
        &self,
        command: &AddEntityCommand,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<(EntitySchema, PathBuf), EntityGenerationError> {
        self.validate_feature(command, workspace, service)?;
        self.validate_persistence_module(service)?;
        self.validate_id_type(command)?;

        if let Some(schema_path) = command.from_schema() {
            let schema = self.handle_from_schema(schema_path, command)?;
            if !command.is_schema_only() {
                self.invoke_template_engine(command, workspace, service)?;
            }

            return Ok((schema, schema_path.clone()));
        }

        let schema = EntitySchema::from_command(command);
        let specs_dir = service
            .path()
            .join(GlobalConstants::SPECS_DIR)
            .join(GlobalConstants::FEATURES_DIR)
            .join(command.feature())
            .join(GlobalConstants::ENTITIES_DIR);
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
            return Ok((schema, schema_file));
        }

        self.invoke_template_engine(command, workspace, service)?;

        Ok((schema, schema_file))
    }

    pub fn validate_id_type(
        &self,
        command: &AddEntityCommand,
    ) -> Result<(), EntityGenerationError> {
        match command.id_type() {
            GeneralType::Integer | GeneralType::Uuid | GeneralType::String => Ok(()),
            other => Err(EntityGenerationError::UnsupportedIdType {
                id_type: other.to_string(),
            }),
        }
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

    fn validate_feature(
        &self,
        command: &AddEntityCommand,
        _workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<(), EntityGenerationError> {
        let feature_name = command.feature();
        let service_root = service.path();

        // Use the same robust discovery logic as ArtifactGenerationService
        let possible_feature_roots = vec![
            service_root.join("src").join("core").join("Features"), // Simplified for brevity in mapping
            service_root
                .join("src")
                .join("Application")
                .join("Features"),
            service_root.join("src").join("Features"),
            service_root.join("Features"),
        ];

        let mut feature_path = None;
        for root in possible_feature_roots {
            let candidate = root.join(feature_name);
            tracing::debug!("Checking for feature directory at: {}", candidate.display());
            if candidate.is_dir() {
                tracing::info!("Found feature directory at: {}", candidate.display());
                feature_path = Some(candidate);
                break;
            }
        }

        if feature_path.is_none() {
            // Fallback to the default C#-style path if not found, to preserve original intent if namespace is involved
            // Note: AddEntityCommandHandler doesn't have easy access to the namespace string without going through artifact_service
            // but we can try to resolve it from the workspace if needed.
            tracing::error!(
                "Feature directory '{}' not found in service '{}' ({})",
                feature_name,
                service.name(),
                service_root.display()
            );

            return Err(EntityGenerationError::FeatureNotFound {
                feature: feature_name.to_string(),
            });
        }

        Ok(())
    }

    fn validate_persistence_module(
        &self,
        service: &ServiceInfo,
    ) -> Result<(), EntityGenerationError> {
        if !service.modules().iter().any(|m| m == "persistence") {
            return Err(EntityGenerationError::MissingPersistenceModule {
                service_name: service.name().to_string(),
            });
        }
        Ok(())
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
        tracing::debug!(
            "Invoking template engine for entity '{}'",
            command.entity_name()
        );

        let app_workspace = self
            .artifact_service
            .get_workspace_context()
            .map_err(map_add_artifact_error)?;

        let services = self
            .artifact_service
            .extract_services(&app_workspace)
            .map_err(map_add_artifact_error)?;

        let app_service = services
            .into_iter()
            .find(|s| s.name() == service.name())
            .ok_or_else(|| {
                tracing::error!("Service '{}' not found in workspace", service.name());
                EntityGenerationError::ServiceNotFound {
                    name: service.name().to_string(),
                }
            })?;

        let artifact_context = self
            .artifact_service
            .load_template_context(app_workspace, &app_service, GlobalConstants::ENTITY_LABEL)
            .map_err(|e| {
                tracing::error!("Failed to load template context: {e}");
                EntityGenerationError::TemplateExecutionError {
                    reason: format!("Failed to load template context for entity generator: {e}"),
                }
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
            PARAM_PROPERTIES.to_string(),
            serde_json::Value::Array(properties),
        );
        params.insert(
            PARAM_ID_TYPE.to_string(),
            serde_json::Value::String(command.id_type().to_string()),
        );
        params.insert(
            PARAM_ENTITY_TYPE.to_string(),
            serde_json::Value::String(command.entity_type().as_schema_value().to_string()),
        );

        let command_params = Some(serde_json::Value::Object(params));

        tracing::info!(
            "Executing template generation for entity '{}' in feature '{}' of service '{}'",
            command.entity_name(),
            command.feature(),
            service.name()
        );

        self.artifact_service
            .execute_generation(
                command.entity_name(),
                Some(command.feature()),
                &command_params,
                &artifact_context,
            )
            .map_err(|e| {
                tracing::error!(
                    "Template generation failed for entity '{}': {}",
                    command.entity_name(),
                    e
                );
                EntityGenerationError::TemplateExecutionError {
                    reason: format!("Template generation failed: {e}"),
                }
            })?;

        Ok(())
    }
}

#[cfg(test)]
#[path = "add_entity_command_handler.tests.rs"]
mod tests;

fn map_add_artifact_error(e: AddArtifactError) -> EntityGenerationError {
    match e {
        AddArtifactError::InvalidIdentifier(msg) => EntityGenerationError::InvalidEntityName {
            name: "".to_string(), // Name unknown in mapping context
            reason: msg,
        },
        AddArtifactError::WorkspaceError(reason) => {
            EntityGenerationError::WorkspaceError { reason }
        }
        AddArtifactError::ConfigError(reason) => EntityGenerationError::ConfigError { reason },
        AddArtifactError::TemplateNotFound(name) => EntityGenerationError::TemplateExecutionError {
            reason: format!("Template not found: {name}"),
        },
        AddArtifactError::InvalidParameter(reason) => {
            EntityGenerationError::TemplateExecutionError {
                reason: format!("Invalid template parameter: {reason}"),
            }
        }
        AddArtifactError::ExecutionFailed(err) => EntityGenerationError::TemplateExecutionError {
            reason: err.to_string(),
        },
        AddArtifactError::MissingRequiredModule(reason) => {
            EntityGenerationError::TemplateExecutionError {
                reason: format!("Missing required module: {reason}"),
            }
        }
        AddArtifactError::NfwYamlReadError(reason) => EntityGenerationError::ConfigError {
            reason: format!("Failed to read nfw.yaml: {reason}"),
        },
        AddArtifactError::NfwYamlParseError(reason) => EntityGenerationError::ConfigError {
            reason: format!("Failed to parse nfw.yaml: {reason}"),
        },
        AddArtifactError::NfwYamlWriteError(reason) => EntityGenerationError::ConfigError {
            reason: format!("Failed to write nfw.yaml: {reason}"),
        },
    }
}
