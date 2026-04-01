use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateSelectionError {
    DiscoverTemplatesFailed(String),
    TemplateNotFound {
        identifier: String,
    },
    AmbiguousTemplateIdentifier {
        identifier: String,
        candidates: Vec<String>,
    },
}

impl Display for TemplateSelectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DiscoverTemplatesFailed(reason) => {
                write!(f, "failed to discover templates: {reason}")
            }
            Self::TemplateNotFound { identifier } => {
                write!(
                    f,
                    "template '{identifier}' was not found; use 'nfw templates list' to view available templates"
                )
            }
            Self::AmbiguousTemplateIdentifier {
                identifier,
                candidates,
            } => {
                write!(
                    f,
                    "template '{identifier}' is ambiguous; use a qualified identifier (source/template). candidates: {}",
                    candidates.join(", ")
                )
            }
        }
    }
}

impl std::error::Error for TemplateSelectionError {}
