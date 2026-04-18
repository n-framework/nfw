use crate::features::template_management::models::template_error::TemplateError;
use std::fmt;

/// Errors produced by the code generation service.
#[derive(Debug, Clone)]
pub enum GenerateError {
    /// The 'name' or 'feature' argument contained invalid characters.
    InvalidIdentifier(String),
    /// The workspace root could not be located.
    WorkspaceError(String),
    /// The workspace configuration (nfw.yaml) is missing or invalid.
    ConfigError(String),
    /// The requested template could not be found locally or in the cache.
    TemplateNotFound(String),
    /// A custom parameter was malformed.
    InvalidParameter(String),
    /// The underlying template engine reported a failure.
    ExecutionFailed(TemplateError),
}

impl fmt::Display for GenerateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier(msg) => write!(f, "{}", msg),
            Self::WorkspaceError(msg) => write!(f, "workspace error: {}", msg),
            Self::ConfigError(msg) => write!(f, "configuration error: {}", msg),
            Self::TemplateNotFound(msg) => write!(f, "template not found: {}", msg),
            Self::InvalidParameter(msg) => write!(f, "invalid parameter: {}", msg),
            Self::ExecutionFailed(err) => write!(f, "execution failed:\n{}", err),
        }
    }
}

impl std::error::Error for GenerateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ExecutionFailed(err) => Some(err),
            _ => None,
        }
    }
}
