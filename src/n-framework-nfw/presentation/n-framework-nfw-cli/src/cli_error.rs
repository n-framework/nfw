use std::fmt::{Display, Formatter};

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

impl From<n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError>
    for CliError
{
    fn from(
        error: n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError,
    ) -> Self {
        use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;
        Self::new(
            ExitCodes::from_add_artifact_error(&error) as i32,
            error.to_string(),
        )
    }
}

impl From<n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError>
    for CliError
{
    fn from(
        error: n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError,
    ) -> Self {
        use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;
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
