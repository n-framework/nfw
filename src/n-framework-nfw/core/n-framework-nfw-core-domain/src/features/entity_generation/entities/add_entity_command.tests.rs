use super::*;
use crate::features::entity_generation::errors::entity_generation_error::EntityGenerationError;

#[test]
fn try_new_validates_pascal_case_name() {
    let options = EntityGenerationOptions::new(None, "Catalog".to_owned(), false, None, true);

    // Valid name
    let result = AddEntityCommand::try_new(
        "Product".to_owned(),
        vec![],
        GeneralType::Uuid,
        EntityType::Entity,
        options.clone(),
    );
    assert!(result.is_ok());

    // Invalid: starts with lowercase
    let result = AddEntityCommand::try_new(
        "product".to_owned(),
        vec![],
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
        vec![],
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
        vec![],
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
        vec![],
        GeneralType::Uuid,
        EntityType::Entity,
        options,
    );
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidEntityName { .. })
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
