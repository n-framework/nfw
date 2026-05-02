use super::*;
use crate::features::entity_generation::errors::entity_generation_error::EntityGenerationError;

#[test]
fn try_new_validates_pascal_case_name() {
    let options = EntityGenerationOptions::new(None, "Catalog".to_owned(), false, None, true);
    let default_props = vec![PropertyDefinition::new(
        "Id".to_owned(),
        GeneralType::Uuid,
        false,
    )];

    // Valid name
    let result = AddEntityCommand::try_new(
        "Product".to_owned(),
        default_props.clone(),
        GeneralType::Uuid,
        EntityType::Entity,
        options.clone(),
    );
    assert!(result.is_ok());

    // Invalid: starts with lowercase
    let result = AddEntityCommand::try_new(
        "product".to_owned(),
        default_props.clone(),
        GeneralType::Uuid,
        EntityType::Entity,
        options.clone(),
    );
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));

    // Invalid: contains space
    let result = AddEntityCommand::try_new(
        "Product Name".to_owned(),
        default_props.clone(),
        GeneralType::Uuid,
        EntityType::Entity,
        options.clone(),
    );
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));

    // Invalid: starts with digit
    let result = AddEntityCommand::try_new(
        "1Product".to_owned(),
        default_props.clone(),
        GeneralType::Uuid,
        EntityType::Entity,
        options.clone(),
    );
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));

    // Invalid: empty
    let result = AddEntityCommand::try_new(
        "".to_owned(),
        default_props.clone(),
        GeneralType::Uuid,
        EntityType::Entity,
        options.clone(),
    );
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));

    // Invalid: contains underscore (NEW)
    let result = AddEntityCommand::try_new(
        "My_Entity".to_owned(),
        default_props.clone(),
        GeneralType::Uuid,
        EntityType::Entity,
        options.clone(),
    );
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));

    // Invalid: consecutive uppercase (NEW)
    let result = AddEntityCommand::try_new(
        "MYENTITY".to_owned(),
        default_props,
        GeneralType::Uuid,
        EntityType::Entity,
        options.clone(),
    );
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));
}

#[test]
fn try_new_validates_property_list() {
    let options = EntityGenerationOptions::new(None, "Catalog".to_owned(), false, None, true);

    // Empty properties
    let result = AddEntityCommand::try_new(
        "Product".to_owned(),
        vec![],
        GeneralType::Uuid,
        EntityType::Entity,
        options.clone(),
    );
    assert!(matches!(
        result,
        Err(EntityGenerationError::EmptyProperties)
    ));

    // Duplicate properties
    let props = vec![
        PropertyDefinition::new("Name".to_owned(), GeneralType::String, false),
        PropertyDefinition::new("Name".to_owned(), GeneralType::String, false),
    ];
    let result = AddEntityCommand::try_new(
        "Product".to_owned(),
        props,
        GeneralType::Uuid,
        EntityType::Entity,
        options,
    );
    assert!(matches!(
        result,
        Err(EntityGenerationError::DuplicatePropertyName { .. })
    ));
}

#[test]
fn command_properties_accessor() {
    let options = EntityGenerationOptions::new(None, "Catalog".to_owned(), false, None, true);
    let props = vec![PropertyDefinition::new(
        "Name".to_owned(),
        GeneralType::String,
        false,
    )];
    let command = AddEntityCommand::try_new(
        "Product".to_owned(),
        props.clone(),
        GeneralType::Uuid,
        EntityType::Entity,
        options,
    )
    .unwrap();

    assert_eq!(command.properties(), &props);
}

#[test]
fn entity_type_as_schema_value_returns_correct_strings() {
    assert_eq!(EntityType::Entity.as_schema_value(), "entity");
    assert_eq!(
        EntityType::AuditableEntity.as_schema_value(),
        "auditable_entity"
    );
    assert_eq!(
        EntityType::SoftDeletableEntity.as_schema_value(),
        "soft_deletable_entity"
    );
}

#[test]
fn entity_type_from_str_value_parses_correctly() {
    assert_eq!(
        EntityType::from_str_value("entity"),
        Some(EntityType::Entity)
    );
    assert_eq!(
        EntityType::from_str_value("auditable_entity"),
        Some(EntityType::AuditableEntity)
    );
    assert_eq!(
        EntityType::from_str_value("soft_deletable_entity"),
        Some(EntityType::SoftDeletableEntity)
    );
    assert_eq!(EntityType::from_str_value("unknown"), None);
}

#[test]
fn entity_type_display_matches_schema_value() {
    assert_eq!(EntityType::Entity.to_string(), "entity");
    assert_eq!(EntityType::AuditableEntity.to_string(), "auditable_entity");
}
