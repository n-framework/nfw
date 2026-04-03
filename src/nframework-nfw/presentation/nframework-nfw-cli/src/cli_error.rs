use std::fmt::{Display, Formatter};

/// CLI error that carries both the error message and the appropriate exit code.
/// This replaces the fragile string protocol "[exit:N] message" with a proper type.
#[derive(Debug, Clone)]
pub struct CliError {
    pub exit_code: i32,
    pub message: String,
}

impl CliError {
    pub fn new(exit_code: i32, message: impl Into<String>) -> Self {
        Self {
            exit_code,
            message: message.into(),
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            exit_code: 1,
            message: message.into(),
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliError {}

impl From<String> for CliError {
    fn from(message: String) -> Self {
        Self::internal(message)
    }
}

impl From<&str> for CliError {
    fn from(message: &str) -> Self {
        Self::internal(message.to_owned())
    }
}
