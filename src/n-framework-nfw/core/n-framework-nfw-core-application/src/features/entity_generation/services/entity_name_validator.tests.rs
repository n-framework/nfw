use super::EntityNameValidator;
use n_framework_nfw_core_domain::features::entity_generation::errors::entity_generation_error::EntityGenerationError;

#[test]
fn accepts_valid_pascal_case() {
    assert!(EntityNameValidator::validate("Product").is_ok());
    assert!(EntityNameValidator::validate("OrderItem").is_ok());
    assert!(EntityNameValidator::validate("MyEntity123").is_ok());
}

#[test]
fn accepts_underscores() {
    assert!(EntityNameValidator::validate("My_Entity").is_ok());
}

#[test]
fn rejects_empty_name() {
    assert!(matches!(
        EntityNameValidator::validate(""),
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));
}

#[test]
fn rejects_leading_digit() {
    assert!(matches!(
        EntityNameValidator::validate("1Product"),
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));
}

#[test]
fn rejects_special_characters() {
    assert!(matches!(
        EntityNameValidator::validate("My-Entity"),
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));
    assert!(matches!(
        EntityNameValidator::validate("My.Entity"),
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));
}

#[test]
fn rejects_lowercase_start() {
    assert!(matches!(
        EntityNameValidator::validate("product"),
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));
}
