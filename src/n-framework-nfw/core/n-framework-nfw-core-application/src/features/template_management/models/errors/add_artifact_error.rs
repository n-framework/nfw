use crate::features::template_management::models::template_error::TemplateError;
use std::fmt;

/// Errors produced by the artifact adding service.
#[derive(Debug, Clone)]
pub enum AddArtifactError {
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
    ExecutionFailed(Box<TemplateError>),
    /// A required module is not present in the target service.
    MissingRequiredModule(String),
    /// Failed to read nfw.yaml.
    NfwYamlReadError(String),
    /// Failed to parse nfw.yaml.
    NfwYamlParseError(String),
    /// Failed to write nfw.yaml.
    NfwYamlWriteError(String),
}

impl fmt::Display for AddArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIdentifier(msg) => write!(f, "{}", msg),
            Self::WorkspaceError(msg) => write!(f, "workspace error: {}", msg),
            Self::ConfigError(msg) => write!(f, "configuration error: {}", msg),
            Self::TemplateNotFound(msg) => write!(f, "template not found: {}", msg),
            Self::InvalidParameter(msg) => write!(f, "invalid parameter: {}", msg),
            Self::ExecutionFailed(err) => write!(f, "execution failed:\n{}", err),
            Self::MissingRequiredModule(msg) => write!(f, "missing required module: {}", msg),
            Self::NfwYamlReadError(msg) => write!(f, "nfw.yaml read error: {}", msg),
            Self::NfwYamlParseError(msg) => write!(f, "nfw.yaml parse error: {}", msg),
            Self::NfwYamlWriteError(msg) => write!(f, "nfw.yaml write error: {}", msg),
        }
    }
}

impl std::error::Error for AddArtifactError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ExecutionFailed(err) => Some(err),
            _ => None,
        }
    }
}
