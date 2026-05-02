use crate::features::entity_generation::errors::entity_generation_error::EntityGenerationError;

pub struct ValidationUtils;

impl ValidationUtils {
    pub fn validate_pascal_case(
        name: &str,
        entity_type_label: &str,
    ) -> Result<(), EntityGenerationError> {
        if name.is_empty() {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: format!("{} name cannot be empty", entity_type_label),
            });
        }

        if name.starts_with(|c: char| c.is_ascii_digit()) {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: format!("{} name must not start with a digit", entity_type_label),
            });
        }

        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: format!(
                    "{} name must contain only alphanumeric characters and underscores",
                    entity_type_label
                ),
            });
        }

        if !name.starts_with(|c: char| c.is_uppercase()) {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: format!(
                    "{} name must start with an uppercase letter (PascalCase)",
                    entity_type_label
                ),
            });
        }

        Ok(())
    }
}
