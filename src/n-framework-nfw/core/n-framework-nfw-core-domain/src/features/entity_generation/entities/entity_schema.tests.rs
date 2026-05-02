use super::*;

#[test]
fn new_creates_valid_schema() {
    let schema = EntitySchema::new(
        "Product".to_owned(),
        GeneralType::Uuid,
        &EntityType::Entity,
        vec![
            SchemaProperty {
                name: "Name".to_owned(),
                general_type: GeneralType::String,
                nullable: false,
            },
            SchemaProperty {
                name: "Price".to_owned(),
                general_type: GeneralType::Decimal,
                nullable: false,
            },
        ],
    );

    assert_eq!(schema.entity, "Product");
    assert_eq!(schema.id_type, GeneralType::Uuid);
    assert_eq!(schema.entity_type, "entity");
    assert_eq!(schema.properties.len(), 2);
}

#[test]
fn entity_type_parsed_returns_entity_type() {
    let schema = EntitySchema::new(
        "Order".to_owned(),
        GeneralType::Integer,
        &EntityType::AuditableEntity,
        vec![],
    );

    assert_eq!(
        schema.entity_type_parsed(),
        Some(EntityType::AuditableEntity)
    );
}

#[test]
fn entity_type_parsed_returns_none_for_unknown() {
    let schema = EntitySchema {
        entity: "Foo".to_owned(),
        id_type: GeneralType::Integer,
        entity_type: "unknown-type".to_owned(),
        properties: vec![],
    };

    assert_eq!(schema.entity_type_parsed(), None);
}

#[test]
fn serde_roundtrip_yaml() {
    let schema = EntitySchema::new(
        "Product".to_owned(),
        GeneralType::Uuid,
        &EntityType::Entity,
        vec![
            SchemaProperty {
                name: "Name".to_owned(),
                general_type: GeneralType::String,
                nullable: false,
            },
            SchemaProperty {
                name: "Price".to_owned(),
                general_type: GeneralType::Decimal,
                nullable: true,
            },
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
        &EntityType::Entity,
        vec![SchemaProperty {
            name: "Name".to_owned(),
            general_type: GeneralType::String,
            nullable: false,
        }],
    );

    let yaml = serde_yaml::to_string(&schema).expect("serialize");

    assert!(yaml.contains("entity: Product"));
    assert!(yaml.contains("id_type: uuid"));
    assert!(yaml.contains("entity_type: entity"));
    assert!(yaml.contains("name: Name"));
    assert!(yaml.contains("type: string"));
    assert!(yaml.contains("nullable: false"));
}
