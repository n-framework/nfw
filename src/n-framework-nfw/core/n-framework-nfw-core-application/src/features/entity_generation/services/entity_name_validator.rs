use n_framework_nfw_core_domain::features::entity_generation::errors::entity_generation_error::EntityGenerationError;

/// Validates entity names against general identifier rules.
#[derive(Debug, Clone)]
pub struct EntityNameValidator;

impl EntityNameValidator {
    pub fn validate(name: &str) -> Result<(), EntityGenerationError> {
        if name.is_empty() {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: "entity name cannot be empty".to_owned(),
            });
        }

        if name.starts_with(|c: char| c.is_ascii_digit()) {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: "entity name must not start with a digit".to_owned(),
            });
        }

        if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: "entity name must contain only alphanumeric characters and underscores"
                    .to_owned(),
            });
        }

        if !name.starts_with(|c: char| c.is_uppercase()) {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: "entity name must start with an uppercase letter (PascalCase)".to_owned(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
#[path = "entity_name_validator.tests.rs"]
mod tests;
