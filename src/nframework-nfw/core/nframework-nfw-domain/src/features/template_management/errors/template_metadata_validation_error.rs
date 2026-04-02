use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateMetadataValidationError {
    field: &'static str,
    message: String,
}

impl TemplateMetadataValidationError {
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

impl Display for TemplateMetadataValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for TemplateMetadataValidationError {}
