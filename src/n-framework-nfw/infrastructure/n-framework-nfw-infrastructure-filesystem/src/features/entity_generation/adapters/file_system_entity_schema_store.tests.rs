use super::*;
use n_framework_nfw_core_domain::features::entity_generation::entities::add_entity_command::EntityType;
use n_framework_nfw_core_domain::features::entity_generation::entities::entity_schema::{
    EntitySchema, SchemaProperty,
};
use n_framework_nfw_core_domain::features::entity_generation::value_objects::general_type::GeneralType;
use tempfile::TempDir;

fn sample_schema() -> EntitySchema {
    EntitySchema::new(
        "Product".to_owned(),
        GeneralType::Uuid,
        EntityType::Entity,
        vec![
            SchemaProperty::new("Name".to_owned(), GeneralType::String, false),
            SchemaProperty::new("Price".to_owned(), GeneralType::Decimal, false),
        ],
    )
}

#[test]
fn write_and_read_roundtrip() {
    let temp = TempDir::new().unwrap();
    let specs_dir = temp.path().join("specs");
    let store = FileSystemEntitySchemaStore::new();

    let schema = sample_schema();
    store
        .write_schema(&specs_dir, &schema)
        .expect("write should succeed");

    let schema_path = specs_dir.join("Product.yaml");
    assert!(schema_path.is_file());

    let read_back = store
        .read_schema(&schema_path)
        .expect("read should succeed");
    assert_eq!(read_back.entity(), schema.entity());
    assert_eq!(read_back.id_type(), schema.id_type());
    assert_eq!(read_back.properties().len(), 2);
}

#[test]
fn schema_exists_returns_true_for_existing_file() {
    let temp = TempDir::new().unwrap();
    let specs_dir = temp.path().join("specs");
    let store = FileSystemEntitySchemaStore::new();

    let schema = sample_schema();
    store.write_schema(&specs_dir, &schema).unwrap();

    let schema_path = specs_dir.join("Product.yaml");
    assert!(store.schema_exists(&schema_path));
}

#[test]
fn schema_exists_returns_false_for_missing_file() {
    let temp = TempDir::new().unwrap();
    let store = FileSystemEntitySchemaStore::new();
    assert!(!store.schema_exists(&temp.path().join("Missing.yaml")));
}

#[test]
fn read_nonexistent_schema_returns_error() {
    let temp = TempDir::new().unwrap();
    let store = FileSystemEntitySchemaStore::new();
    let result = store.read_schema(&temp.path().join("Ghost.yaml"));
    assert!(matches!(
        result,
        Err(EntityGenerationError::SchemaReadError { .. })
    ));
}

#[test]
fn creates_nested_directories() {
    let temp = TempDir::new().unwrap();
    let nested = temp.path().join("a").join("b").join("specs");
    let store = FileSystemEntitySchemaStore::new();

    store
        .write_schema(&nested, &sample_schema())
        .expect("should create nested dirs");

    assert!(nested.join("Product.yaml").is_file());
}
