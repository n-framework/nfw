use std::fmt::{Display, Formatter};

/// Errors that can occur during architecture validation.
/// Each error variant has an associated error ID for Sentry tracking.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CheckError {
    /// Error ID: validation.workspace.invalid_context
    /// The workspace context could not be resolved (e.g., nfw.yaml not found).
    InvalidWorkspaceContext(String),

    /// Error ID: validation.interrupted
    /// The validation process was interrupted by the user.
    Interrupted,

    /// Error ID: validation.internal
    /// An internal error occurred during validation. This indicates a bug in nfw.
    Internal(String),
}

impl Display for CheckError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWorkspaceContext(message) => {
                write!(
                    f,
                    "invalid workspace context: {message}. Ensure nfw.yaml exists and contains valid workspace configuration."
                )
            }
            Self::Interrupted => {
                write!(
                    f,
                    "architecture validation interrupted by user. Rerun `nfw check` to complete validation."
                )
            }
            Self::Internal(message) => {
                write!(
                    f,
                    "internal validation error: {message}. This is a bug in nfw. Please report this issue with the command output and workspace configuration."
                )
            }
        }
    }
}

impl std::error::Error for CheckError {}

impl CheckError {
    /// Returns the error ID for Sentry tracking.
    pub fn error_id(&self) -> &'static str {
        match self {
            Self::InvalidWorkspaceContext(_) => "validation.workspace.invalid_context",
            Self::Interrupted => "validation.interrupted",
            Self::Internal(_) => "validation.internal",
        }
    }
}
