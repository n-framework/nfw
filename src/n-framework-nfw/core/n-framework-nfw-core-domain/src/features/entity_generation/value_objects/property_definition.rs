use super::general_type::GeneralType;
use super::global_constants::GlobalConstants;
use super::validation_utils::ValidationUtils;
use crate::features::entity_generation::errors::entity_generation_error::EntityGenerationError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyDefinition {
    name: String,
    general_type: GeneralType,
    nullable: bool,
}

impl PropertyDefinition {
    pub fn new(name: String, general_type: GeneralType, nullable: bool) -> Self {
        Self {
            name,
            general_type,
            nullable,
        }
    }

    pub fn try_new(
        name: String,
        general_type: GeneralType,
        nullable: bool,
    ) -> Result<Self, EntityGenerationError> {
        ValidationUtils::validate_pascal_case(&name, GlobalConstants::PROPERTY_LABEL)?;
        Ok(Self::new(name, general_type, nullable))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn general_type(&self) -> &GeneralType {
        &self.general_type
    }

    pub fn cli_type(&self) -> String {
        self.general_type.to_string()
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_new_validates_name() {
        let result =
            PropertyDefinition::try_new("invalid name".to_owned(), GeneralType::String, false);
        assert!(result.is_err());

        let result =
            PropertyDefinition::try_new("ValidProperty".to_owned(), GeneralType::String, false);
        assert!(result.is_ok());
    }
}
