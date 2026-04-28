use n_framework_core_template_abstractions::TemplateError as CoreTemplateError;
use std::path::PathBuf;

/// Errors that can occur during template management and execution.
#[derive(Debug, Clone, thiserror::Error)]
pub enum TemplateError {
    /// Error in template configuration or metadata.
    /// This error indicates that the definition of the template (e.g. `template.yaml`) is missing or invalid.
    #[error("template configuration error for {}: {message}", .template_id.as_deref().unwrap_or("unknown"))]
    TemplateConfigError {
        /// The error message.
        message: String,
        /// Optional identifier for the template.
        template_id: Option<String>,
    },

    /// Error during template rendering or folder processing.
    /// Unlike `TemplateConfigError`, this occurs when the template itself is valid, but the
    /// executing engine fails to interpolate placeholders, resolve paths, or write the actual file.
    #[error("template rendering error at step {} in {}: {message}", 
        .step_index.map(|i| i.to_string()).unwrap_or_else(|| "?".to_string()),
        .template_id.as_deref().unwrap_or("unknown")
    )]
    TemplateRenderError {
        /// The error message.
        message: String,
        /// The index of the step that failed.
        step_index: Option<usize>,
        /// The identifier of the template being rendered.
        template_id: Option<String>,
        /// The path to the file being rendered.
        file_path: Option<String>,
        /// The underlying core template error.
        source: Option<Box<CoreTemplateError>>,
    },

    /// Error during content injection into an existing file.
    #[error("template injection error for {region:?} in {}: {message}", .file_path.as_deref().unwrap_or("unknown"))]
    TemplateInjectionError {
        /// The error message.
        message: String,
        /// The path to the target file.
        file_path: Option<String>,
        /// The name of the region being injected into.
        region: Option<String>,
        /// The identifier of the template being injected.
        template_id: Option<String>,
    },

    /// Low-level I/O error during template operations.
    #[error("I/O error at {}: {message}", .path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "unknown".to_string()))]
    IoError {
        /// The error message.
        message: String,
        /// The path where the I/O error occurred.
        path: Option<PathBuf>,
    },

    /// Error during shell command execution within a template step.
    #[error("command execution error at step {step_index} in {template_id}: {message}\nWorking Dir: {working_dir}\nCommand: {command}\nStdout: {stdout}", 
        template_id = .0.template_id.as_deref().unwrap_or("unknown"),
        step_index = .0.step_index,
        message = .0.message,
        command = .0.command,
        working_dir = .0.working_directory.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "unknown".to_string()),
        stdout = .0.stdout.as_deref().unwrap_or("(none)")
    )]
    CommandExecutionError(Box<CommandExecutionContext>),
}

/// Rich context for a failed command execution.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct CommandExecutionContext {
    /// The error message (includes stderr output).
    pub message: String,
    /// The command that was executed.
    pub command: String,
    /// The stdout output of the process.
    pub stdout: Option<String>,
    /// The execution directory.
    pub working_directory: Option<PathBuf>,
    /// The exit code returned by the process, if available.
    pub exit_code: Option<i32>,
    /// The index of the step that failed.
    pub step_index: usize,
    /// The identifier of the template.
    pub template_id: Option<String>,
}

impl TemplateError {
    /// Returns a stable error identifier for telemetry or programmatic checks.
    pub fn error_id(&self) -> &'static str {
        match self {
            TemplateError::TemplateConfigError { .. } => "TEMPLATE_CONFIG_ERROR",
            TemplateError::TemplateRenderError { .. } => "TEMPLATE_RENDER_ERROR",
            TemplateError::TemplateInjectionError { .. } => "TEMPLATE_INJECTION_ERROR",
            TemplateError::IoError { .. } => "TEMPLATE_IO_ERROR",
            TemplateError::CommandExecutionError { .. } => "COMMAND_EXECUTION_ERROR",
        }
    }

    /// Convenience method to create a wrapped I/O error.
    pub fn io(message: impl Into<String>, path: impl Into<PathBuf>) -> Self {
        Self::IoError {
            message: message.into(),
            path: Some(path.into()),
        }
    }
}
