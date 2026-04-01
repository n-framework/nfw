use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateCatalogError {
    InvalidYaml(String),
    MissingField { field: &'static str },
    EmptyField { field: &'static str },
    InvalidField { field: &'static str, reason: String },
    UnsupportedLanguage { value: String },
}

impl Display for TemplateCatalogError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidYaml(error) => {
                write!(f, "template metadata is not valid YAML: {error}")
            }
            Self::MissingField { field } => {
                write!(f, "template metadata missing required field '{field}'")
            }
            Self::EmptyField { field } => {
                write!(f, "template metadata field '{field}' cannot be empty")
            }
            Self::InvalidField { field, reason } => {
                write!(f, "template metadata field '{field}' is invalid: {reason}")
            }
            Self::UnsupportedLanguage { value } => {
                write!(
                    f,
                    "template metadata language '{value}' is not supported; use one of: dotnet, go, rust"
                )
            }
        }
    }
}

impl std::error::Error for TemplateCatalogError {}
