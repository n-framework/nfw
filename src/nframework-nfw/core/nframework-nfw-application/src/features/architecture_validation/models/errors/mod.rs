use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArchitectureValidationError {
    InvalidWorkspaceContext(String),
    Interrupted,
    Internal(String),
}

impl Display for ArchitectureValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWorkspaceContext(message) => write!(f, "invalid workspace context: {message}"),
            Self::Interrupted => write!(f, "architecture validation interrupted"),
            Self::Internal(message) => write!(f, "internal validation error: {message}"),
        }
    }
}

impl std::error::Error for ArchitectureValidationError {}
