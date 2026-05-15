use n_framework_core_template_abstractions::TemplateError as CoreTemplateError;
use std::path::PathBuf;

/// Errors that can occur during generator management and execution.
#[derive(Debug, Clone, thiserror::Error)]
pub enum GeneratorError {
    /// Error in generator configuration or metadata.
    /// This error indicates that the definition of the generator (e.g. `generator.yaml`) is missing or invalid.
    #[error("generator configuration error for {}: {message}", .generator_id.as_deref().unwrap_or("unknown"))]
    GeneratorConfigError {
        /// The error message.
        message: String,
        /// Optional identifier for the generator.
        generator_id: Option<String>,
    },

    /// Error during generator rendering or folder processing.
    /// Unlike `GeneratorConfigError`, this occurs when the generator itself is valid, but the
    /// executing engine fails to interpolate placeholders, resolve paths, or write the actual file.
    #[error("generator rendering error at step {} in {}: {message}", 
        .step_index.map(|i| i.to_string()).unwrap_or_else(|| "?".to_string()),
        .generator_id.as_deref().unwrap_or("unknown")
    )]
    GeneratorRenderError {
        /// The error message.
        message: String,
        /// The index of the step that failed.
        step_index: Option<usize>,
        /// The identifier of the generator being rendered.
        generator_id: Option<String>,
        /// The path to the file being rendered.
        file_path: Option<String>,
        /// The underlying core generator error.
        source: Option<Box<CoreTemplateError>>,
    },

    /// Error during content injection into an existing file.
    #[error("generator injection error for {region:?} in {}: {message}", .file_path.as_deref().unwrap_or("unknown"))]
    GeneratorInjectionError {
        /// The error message.
        message: String,
        /// The path to the target file.
        file_path: Option<String>,
        /// The name of the region being injected into.
        region: Option<String>,
        /// The identifier of the generator being injected.
        generator_id: Option<String>,
    },

    /// Low-level I/O error during generator operations.
    #[error("I/O error at {}: {message}", .path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "unknown".to_string()))]
    IoError {
        /// The error message.
        message: String,
        /// The path where the I/O error occurred.
        path: Option<PathBuf>,
    },

    /// Error during shell command execution within a generator step.
    #[error("command execution error at step {step_index} in {generator_id}: {message}\nWorking Dir: {working_dir}\nCommand: {command}\nStdout: {stdout}", 
        generator_id = .0.generator_id.as_deref().unwrap_or("unknown"),
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
    /// The identifier of the generator.
    pub generator_id: Option<String>,
}

impl GeneratorError {
    /// Returns a stable error identifier for telemetry or programmatic checks.
    pub fn error_id(&self) -> &'static str {
        match self {
            GeneratorError::GeneratorConfigError { .. } => "GENERATOR_CONFIG_ERROR",
            GeneratorError::GeneratorRenderError { .. } => "GENERATOR_RENDER_ERROR",
            GeneratorError::GeneratorInjectionError { .. } => "GENERATOR_INJECTION_ERROR",
            GeneratorError::IoError { .. } => "GENERATOR_IO_ERROR",
            GeneratorError::CommandExecutionError { .. } => "COMMAND_EXECUTION_ERROR",
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
