use n_framework_nfw_core_application::features::check::commands::check::check_command_handler::CheckCommandHandler;
use n_framework_nfw_core_application::features::check::models::ExitOutcome;
use n_framework_nfw_core_application::features::check::models::check_command_request::CheckCommandRequest;

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

pub struct RunCheckCliCommand<'a, L> {
    handler: &'a CheckCommandHandler,
    formatter: CheckOutputFormatter,
    logger: L,
}

impl<'a, L> RunCheckCliCommand<'a, L>
where
    L: n_framework_core_cli_abstractions::Logger,
{
    pub fn new(handler: &'a CheckCommandHandler, logger: L) -> Self {
        Self {
            handler,
            formatter: CheckOutputFormatter::new(),
            logger,
        }
    }

    pub fn execute(&self) -> Result<(), RunCheckError> {
        self.logger
            .intro("Architecture Check")
            .map_err(|e| RunCheckError::CommandError(e.to_string()))?;

        tracing::info!("Starting architecture check");
        let request = CheckCommandRequest::current_dir().map_err(|e| {
            tracing::error!("Failed to resolve current directory: {}", e);
            RunCheckError::CurrentDirectoryUnavailable(e)
        })?;

        let spinner = self
            .logger
            .spinner("Checking workspace architecture...")
            .map_err(|e| RunCheckError::CommandError(e.to_string()))?;

        let result = self.handler.execute(&request).map_err(|e| {
            spinner.error(&format!("Check failed: {e}"));
            tracing::error!("Check command execution failed: {}", e);
            RunCheckError::CommandError(e)
        })?;

        tracing::info!(
            "Architecture check completed with outcome: {:?}",
            result.summary.exit_outcome
        );

        match result.summary.exit_outcome {
            ExitOutcome::Success => {
                spinner.success("No architecture violations found");
                self.logger
                    .outro(&self.formatter.success_message(&result))
                    .map_err(|e| RunCheckError::CommandError(e.to_string()))?;
                Ok(())
            }
            ExitOutcome::ViolationFound => {
                spinner.error(&format!(
                    "Found {} architecture violation(s)",
                    result.summary.total_findings
                ));
                self.logger
                    .outro(&self.formatter.failure_message(&result))
                    .map_err(|e| RunCheckError::CommandError(e.to_string()))?;
                tracing::warn!("Architecture check found violations");
                Err(RunCheckError::ValidationFailed)
            }
            ExitOutcome::ExecutionInterrupted => {
                spinner.cancel("Architecture check interrupted");
                self.logger
                    .outro(&self.formatter.failure_message(&result))
                    .map_err(|e| RunCheckError::CommandError(e.to_string()))?;
                tracing::warn!("Architecture check was interrupted");
                Err(RunCheckError::Interrupted)
            }
        }
    }
}
