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
        // Allow up to 3 consecutive uppercase letters for common abbreviations (PDF, XML, ID)
        let chars: Vec<char> = name.chars().collect();
        let mut consecutive_upper = 0;
        for c in chars {
            if c.is_ascii_uppercase() {
                consecutive_upper += 1;
                if consecutive_upper > 3 {
                    return Err(EntityGenerationError::InvalidEntityName {
                        name: name.to_owned(),
                        reason: format!(
                            "{} name must be in PascalCase (e.g., 'MyEntity'). More than 3 consecutive uppercase letters like in '{}' are not allowed.",
                            entity_type_label, name
                        ),
                    });
                }
            } else {
                consecutive_upper = 0;
            }
        }

        Ok(())
    }
}
