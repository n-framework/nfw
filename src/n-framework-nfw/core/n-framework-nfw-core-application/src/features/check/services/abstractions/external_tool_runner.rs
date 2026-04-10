use std::path::Path;

use crate::features::check::models::ValidationFinding;

/// Result of running an external tool like `make lint` or `make test`.
pub struct ExternalToolResult {
    /// Whether the tool execution succeeded
    pub success: bool,
    /// Standard output from the tool
    pub stdout: String,
    /// Standard error from the tool
    pub stderr: String,
    /// Exit code if available
    pub exit_code: Option<i32>,
}

/// Trait for running external tools as part of validation.
/// This abstraction allows the application layer to remain pure while
/// implementations handle process execution in the infrastructure layer.
pub trait ExternalToolRunner: Send + Sync {
    /// Runs `make lint` in the given directory and returns the result.
    /// Returns None if linting succeeded, or a ValidationFinding if it failed.
    fn run_make_lint(&self, workspace_root: &Path) -> Option<ValidationFinding>;

    /// Runs `make test` in the given service directory and returns the result.
    /// Returns None if tests passed, or a ValidationFinding if they failed.
    fn run_make_test(&self, service_root: &Path) -> Option<ValidationFinding>;

    /// Runs a generic external command and returns the structured result.
    /// This is a lower-level method for more complex scenarios.
    fn run_command(
        &self,
        command: &str,
        args: &[&str],
        working_dir: &Path,
    ) -> Result<ExternalToolResult, String>;
}
