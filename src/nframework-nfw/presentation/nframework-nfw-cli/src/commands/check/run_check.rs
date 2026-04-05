use nframework_nfw_application::features::architecture_validation::commands::check::check_command_handler::CheckCommandHandler;
use nframework_nfw_application::features::architecture_validation::models::check_command_request::CheckCommandRequest;
use nframework_nfw_application::features::architecture_validation::models::errors::ArchitectureValidationError;
use nframework_nfw_application::features::architecture_validation::models::ExitOutcome;

use crate::commands::check::check_output_formatter::CheckOutputFormatter;

#[derive(Debug)]
pub enum RunCheckError {
    ValidationFailed,
    CommandError(ArchitectureValidationError),
    Interrupted,
    CurrentDirectoryUnavailable(String),
}

impl std::fmt::Display for RunCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidationFailed => write!(f, "architecture validation failed"),
            Self::CommandError(error) => write!(f, "{error}"),
            Self::Interrupted => write!(f, "architecture validation interrupted"),
            Self::CurrentDirectoryUnavailable(error) => {
                write!(f, "failed to resolve current directory: {error}")
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct RunCheckCliCommand {
    handler: CheckCommandHandler,
    formatter: CheckOutputFormatter,
}

impl RunCheckCliCommand {
    pub fn new(handler: CheckCommandHandler) -> Self {
        Self {
            handler,
            formatter: CheckOutputFormatter::new(),
        }
    }

    pub fn execute(&self) -> Result<(), RunCheckError> {
        let start_directory = std::env::current_dir()
            .map_err(|error| RunCheckError::CurrentDirectoryUnavailable(error.to_string()))?;

        let request = CheckCommandRequest::new(start_directory);
        let result = self
            .handler
            .execute(&request)
            .map_err(RunCheckError::CommandError)?;

        match result.summary.exit_outcome {
            ExitOutcome::Success => {
                println!("{}", self.formatter.success_message(&result));
                Ok(())
            }
            ExitOutcome::ViolationFound => {
                eprintln!("{}", self.formatter.failure_message(&result));
                Err(RunCheckError::ValidationFailed)
            }
            ExitOutcome::ExecutionInterrupted => {
                eprintln!("{}", self.formatter.failure_message(&result));
                Err(RunCheckError::Interrupted)
            }
        }
    }
}
