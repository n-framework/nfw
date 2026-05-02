use std::fmt::{Display, Formatter};

use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;
use n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;
use n_framework_nfw_core_domain::features::entity_generation::errors::entity_generation_error::EntityGenerationError;

/// CLI error that carries both the error message and the appropriate exit code.
/// This replaces the fragile string protocol "[exit:N] message" with a proper type.
#[derive(Debug, Clone)]
pub struct CliError {
    pub exit_code: i32,
    pub message: String,
    pub is_silent: bool,
}

impl CliError {
    pub fn new(exit_code: i32, message: impl Into<String>) -> Self {
        Self {
            exit_code,
            message: message.into(),
            is_silent: false,
        }
    }

    pub fn silent(exit_code: i32, message: impl Into<String>) -> Self {
        Self {
            exit_code,
            message: message.into(),
            is_silent: true,
        }
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            exit_code: 1,
            message: message.into(),
            is_silent: false,
        }
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CliError {}

impl From<AddArtifactError> for CliError {
    fn from(error: AddArtifactError) -> Self {
        Self::new(
            ExitCodes::from_add_artifact_error(&error) as i32,
            error.to_string(),
        )
    }
}

impl From<AddServiceError> for CliError {
    fn from(error: AddServiceError) -> Self {
        Self::new(
            ExitCodes::from_add_service_error(&error) as i32,
            error.to_string(),
        )
    }
}

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

impl From<EntityGenerationError> for CliError {
    fn from(error: EntityGenerationError) -> Self {
        Self::new(ExitCodes::ValidationError as i32, error.to_string())
    }
}
