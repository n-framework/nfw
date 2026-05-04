use super::*;
use crate::features::entity_generation::entities::add_entity_command::{
    AddEntityCommand, EntityGenerationOptions, EntityType,
};
use crate::features::entity_generation::value_objects::property_definition::PropertyDefinition;

#[test]
fn new_creates_valid_schema() {
    let schema = EntitySchema::new(
        "Product".to_owned(),
        GeneralType::Uuid,
        EntityType::Entity,
        vec![
            SchemaProperty::new("Name".to_owned(), GeneralType::String, false),
            SchemaProperty::new("Price".to_owned(), GeneralType::Decimal, false),
        ],
    );

    assert_eq!(schema.entity(), "Product");
    assert_eq!(schema.id_type(), &GeneralType::Uuid);
    assert_eq!(schema.entity_type(), &EntityType::Entity);
    assert_eq!(schema.properties().len(), 2);
}

#[test]
fn from_command_creates_valid_schema() {
    let command = AddEntityCommand::try_new(
        "Product".to_owned(),
        vec![PropertyDefinition::new(
            "Name".to_owned(),
            GeneralType::String,
            false,
        )],
        GeneralType::Uuid,
        EntityType::Entity,
        EntityGenerationOptions::new(None, "Catalog".to_owned(), false, None, true),
    )
    .expect("valid command");

    let schema = EntitySchema::from_command(&command);

    assert_eq!(schema.entity(), "Product");
    assert_eq!(schema.id_type(), &GeneralType::Uuid);
    assert_eq!(schema.entity_type(), &EntityType::Entity);
    assert_eq!(schema.properties().len(), 1);
    assert_eq!(schema.properties()[0].name(), "Name");
}

#[test]
fn serde_roundtrip_yaml() {
    let schema = EntitySchema::new(
        "Product".to_owned(),
        GeneralType::Uuid,
        EntityType::Entity,
        vec![
            SchemaProperty::new("Name".to_owned(), GeneralType::String, false),
            SchemaProperty::new("Price".to_owned(), GeneralType::Decimal, true),
        ],
    );

    let yaml = serde_yaml::to_string(&schema).expect("serialize");
    let deserialized: EntitySchema = serde_yaml::from_str(&yaml).expect("deserialize");

    assert_eq!(schema, deserialized);
}

#[test]
fn yaml_matches_expected_format() {
    let schema = EntitySchema::new(
        "Product".to_owned(),
        GeneralType::Uuid,
        EntityType::Entity,
        vec![SchemaProperty::new(
            "Name".to_owned(),
            GeneralType::String,
            false,
        )],
    );

    let yaml = serde_yaml::to_string(&schema).expect("serialize");
    println!("YAML output:\n{}", yaml);

    assert!(yaml.contains("$schema: https://raw.githubusercontent.com/n-framework/nfw/main/src/nfw/schemas/entity.schema.json"));
    assert!(yaml.contains("entity: Product"));
    assert!(yaml.contains("id_type: uuid"));
    assert!(yaml.contains("entity_type: entity"));
    assert!(yaml.contains("name: Name"));
    assert!(yaml.contains("type: string"));
    assert!(yaml.contains("nullable: false"));
}

#[test]
fn try_new_validates_entity_name() {
    let result = EntitySchema::try_new(
        "invalid name".to_owned(),
        GeneralType::Uuid,
        EntityType::Entity,
        vec![],
    );

    assert!(result.is_err());
    if let Err(EntityGenerationError::InvalidEntityName { reason, .. }) = result {
        assert!(reason.contains("entity name"));
    } else {
        panic!("Expected InvalidEntityName error");
    }
}

#[test]
fn schema_property_try_new_validates_name() {
    let result = SchemaProperty::try_new("invalid name".to_owned(), GeneralType::String, false);

    assert!(result.is_err());
    if let Err(EntityGenerationError::InvalidEntityName { reason, .. }) = result {
        assert!(reason.contains("property name"));
    } else {
        panic!("Expected InvalidEntityName error");
    }
}
