use thiserror::Error;

/// Errors related to template configuration.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TemplateConfigError {
    /// No steps defined in the template.
    #[error("template must have at least one step")]
    NoSteps,
    /// A step has invalid empty fields.
    #[error("step {index}: {message}")]
    InvalidStep {
        /// The index of the invalid step.
        index: usize,
        /// Description of the validation failure.
        message: String,
    },
    /// General configuration error.
    #[error("invalid template configuration: {0}")]
    General(String),
    /// A field has an invalid format.
    #[error("invalid format for field '{field}': {message}")]
    InvalidFormat {
        /// The name of the invalid field.
        field: String,
        /// Description of why the format is invalid.
        message: String,
    },
}
