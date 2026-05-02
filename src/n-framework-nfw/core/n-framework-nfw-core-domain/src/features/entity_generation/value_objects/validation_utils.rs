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

        if !name.chars().next().is_some_and(|c| c.is_ascii_uppercase()) {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: format!(
                    "{} name must start with an uppercase letter (PascalCase)",
                    entity_type_label
                ),
            });
        }

        if name.chars().any(|c| !c.is_ascii_alphanumeric()) {
            return Err(EntityGenerationError::InvalidEntityName {
                name: name.to_owned(),
                reason: format!(
                    "{} name must contain only alphanumeric characters (no underscores or special characters)",
                    entity_type_label
                ),
            });
        }

        // Check for consecutive uppercase letters to reject MYENTITY-style names
        let chars: Vec<char> = name.chars().collect();
        for i in 0..chars.len() - 1 {
            if chars[i].is_ascii_uppercase() && chars[i + 1].is_ascii_uppercase() {
                return Err(EntityGenerationError::InvalidEntityName {
                    name: name.to_owned(),
                    reason: format!(
                        "{} name must be in PascalCase (e.g., 'MyEntity'). Consecutive uppercase letters like in '{}' are not allowed.",
                        entity_type_label, name
                    ),
                });
            }
        }

        Ok(())
    }
}
