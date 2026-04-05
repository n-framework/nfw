use nframework_nfw_application::features::check::commands::check::check_command_handler::CheckCommandHandler;
use nframework_nfw_application::features::check::models::ExitOutcome;
use nframework_nfw_application::features::check::models::check_command_request::CheckCommandRequest;

use crate::commands::check::check_output_formatter::CheckOutputFormatter;

#[derive(Debug)]
pub enum RunCheckError {
    ValidationFailed,
    CommandError(String),
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

pub struct RunCheckCliCommand<'a> {
    handler: &'a CheckCommandHandler,
    formatter: CheckOutputFormatter,
}

impl<'a> RunCheckCliCommand<'a> {
    pub fn new(handler: &'a CheckCommandHandler) -> Self {
        Self {
            handler,
            formatter: CheckOutputFormatter::new(),
        }
    }

    pub fn execute(&self) -> Result<(), RunCheckError> {
        let request = CheckCommandRequest::current_dir()
            .map_err(RunCheckError::CurrentDirectoryUnavailable)?;

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
