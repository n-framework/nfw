use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorMetadataValidationError {
    field: &'static str,
    message: String,
}

impl GeneratorMetadataValidationError {
    pub fn missing_field(field: &'static str) -> Self {
        Self {
            field,
            message: format!("'{field}' is required"),
        }
    }

    pub fn invalid_field(field: &'static str, message: &str) -> Self {
        Self {
            field,
            message: message.to_owned(),
        }
    }

    pub fn field(&self) -> &'static str {
        self.field
    }
}

impl Display for GeneratorMetadataValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for GeneratorMetadataValidationError {}
