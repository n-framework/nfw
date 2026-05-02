use super::*;
use crate::features::template_management::models::template_error::TemplateError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::entity_generation::entities::add_entity_command::{
    AddEntityCommand, EntityGenerationOptions, EntityType,
};
use n_framework_nfw_core_domain::features::entity_generation::value_objects::general_type::GeneralType;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::service_info::ServiceInfo;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::workspace_context::WorkspaceContext;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use serde_yaml::Value as YamlValue;
use std::path::PathBuf;
use tempfile;

// --- Mocks ---

struct MockWorkingDir;
impl WorkingDirectoryProvider for MockWorkingDir {
    fn current_dir(&self) -> Result<PathBuf, String> {
        Ok(PathBuf::from("/workspace"))
    }
}

struct MockResolver;
impl TemplateRootResolver for MockResolver {
    fn resolve(&self, _yaml: &YamlValue, _id: &str, _root: &Path) -> Result<PathBuf, String> {
        Ok(PathBuf::from("/templates"))
    }
}

struct MockEngine;
impl TemplateEngine for MockEngine {
    fn execute(
        &self,
        _config: &TemplateConfig,
        _root: &Path,
        _output: &Path,
        _params: &TemplateParameters,
    ) -> Result<(), TemplateError> {
        Ok(())
    }
}

struct MockSchemaStore {
    exists: bool,
}
impl EntitySchemaStore for MockSchemaStore {
    fn write_schema(
        &self,
        _dir: &Path,
        _schema: &EntitySchema,
    ) -> Result<(), EntityGenerationError> {
        Ok(())
    }
    fn read_schema(&self, _path: &Path) -> Result<EntitySchema, EntityGenerationError> {
        Ok(EntitySchema::new(
            "Test".to_owned(),
            GeneralType::Uuid,
            EntityType::Entity,
            vec![],
        ))
    }
    fn schema_exists(&self, _path: &Path) -> bool {
        self.exists
    }
}

// --- Tests ---

fn setup_handler(
    exists: bool,
) -> AddEntityCommandHandler<MockWorkingDir, MockResolver, MockEngine, MockSchemaStore> {
    AddEntityCommandHandler::new(
        MockWorkingDir,
        MockResolver,
        MockEngine,
        MockSchemaStore { exists },
    )
}

#[test]
fn validate_id_type_supports_common_types() {
    let handler = setup_handler(false);

    let command = AddEntityCommand::try_new(
        "Product".to_owned(),
        vec![],
        GeneralType::Uuid,
        EntityType::Entity,
        EntityGenerationOptions::default(),
    )
    .unwrap();
    assert!(handler.validate_id_type(&command).is_ok());

    let command = AddEntityCommand::try_new(
        "Product".to_owned(),
        vec![],
        GeneralType::Integer,
        EntityType::Entity,
        EntityGenerationOptions::default(),
    )
    .unwrap();
    assert!(handler.validate_id_type(&command).is_ok());

    let command = AddEntityCommand::try_new(
        "Product".to_owned(),
        vec![],
        GeneralType::String,
        EntityType::Entity,
        EntityGenerationOptions::default(),
    )
    .unwrap();
    assert!(handler.validate_id_type(&command).is_ok());
}

#[test]
fn validate_id_type_fails_on_unsupported_types() {
    let handler = setup_handler(false);

    let unsupported_types = vec![
        GeneralType::Decimal,
        GeneralType::Boolean,
        GeneralType::DateTime,
        GeneralType::Bytes,
    ];

    for id_type in unsupported_types {
        let command = AddEntityCommand::try_new(
            "Product".to_owned(),
            vec![],
            id_type.clone(),
            EntityType::Entity,
            EntityGenerationOptions::default(),
        )
        .unwrap();

        let result = handler.validate_id_type(&command);
        assert!(
            result.is_err(),
            "Expected error for ID type {:?}, but got Ok",
            id_type
        );
        match result {
            Err(EntityGenerationError::UnsupportedIdType {
                id_type: err_type, ..
            }) => {
                assert_eq!(err_type, id_type.to_string().to_lowercase());
            }
            _ => panic!("Expected UnsupportedIdType error, got {:?}", result),
        }
    }
}

#[test]
fn validate_persistence_module_checks_service() {
    let handler = setup_handler(false);

    let service = ServiceInfo::new(
        "Catalog".to_owned(),
        PathBuf::from("/src/Catalog"),
        vec!["persistence".to_owned()],
    );
    assert!(handler.validate_persistence_module(&service).is_ok());

    let service = ServiceInfo::new(
        "Catalog".to_owned(),
        PathBuf::from("/src/Catalog"),
        vec!["api".to_owned()],
    );
    let result = handler.validate_persistence_module(&service);
    assert!(result.is_err());
    if let Err(EntityGenerationError::MissingPersistenceModule { service_name }) = result {
        assert_eq!(service_name, "Catalog");
    } else {
        panic!("Expected MissingPersistenceModule error");
    }
}

#[test]
fn handle_fails_if_schema_already_exists() {
    let temp = tempfile::tempdir().unwrap();
    let service_path = temp.path().join("src/Catalog");
    let feature_path = service_path.join("Features").join("Catalog");
    std::fs::create_dir_all(&feature_path).unwrap();

    let handler = setup_handler(true); // schema exists
    let command = AddEntityCommand::try_new(
        "Product".to_owned(),
        vec![],
        GeneralType::Uuid,
        EntityType::Entity,
        EntityGenerationOptions::new(None, "Catalog".to_owned(), false, None, true),
    )
    .unwrap();
    let workspace = WorkspaceContext::new(PathBuf::from("/"), vec![]);
    let service = ServiceInfo::new(
        "Catalog".to_owned(),
        service_path.clone(),
        vec!["persistence".to_owned()],
    );

    // We need MockSchemaStore to return true for the specific schema file path
    // The handler builds the path as: service.path()/specs/features/Catalog/entities/Product.yaml
    let specs_dir = service_path.join("specs/features/Catalog/entities");
    let schema_file = specs_dir.join("Product.yaml");

    // The MockSchemaStore in setup_handler returns self.exists regardless of path
    let result = handler.handle(&command, &workspace, &service);
    assert!(result.is_err());
    match result {
        Err(EntityGenerationError::SchemaFileConflict { path }) => {
            assert_eq!(path, schema_file);
        }
        Err(e) => panic!("Expected SchemaFileConflict error, got {:?}", e),
        _ => panic!("Expected SchemaFileConflict error"),
    }
}

#[test]
fn map_add_artifact_error_preserves_context() {
    let err = AddArtifactError::WorkspaceError("not found".to_owned());
    let mapped = map_add_artifact_error(err);
    match mapped {
        EntityGenerationError::WorkspaceError { reason } => assert_eq!(reason, "not found"),
        _ => panic!("Expected WorkspaceError"),
    }

    let err = AddArtifactError::NfwYamlParseError("invalid yaml".to_owned());
    let mapped = map_add_artifact_error(err);
    match mapped {
        EntityGenerationError::ConfigError { reason } => {
            assert_eq!(reason, "Failed to parse nfw.yaml: invalid yaml")
        }
        _ => panic!("Expected ConfigError"),
    }
}
