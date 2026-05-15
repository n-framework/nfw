use thiserror::Error;

/// Errors related to generator configuration.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum GeneratorConfigError {
    /// No steps defined in the generator.
    #[error("generator must have at least one step")]
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
    #[error("invalid generator configuration: {0}")]
    General(String),
    /// A field has an invalid format.
    #[error("invalid format for field '{field}': {message}")]
    InvalidFormat {
        /// The name of the invalid field.
        field: String,
        /// Description of why the format is invalid.
        message: String,
    },
    /// A specific input has validation errors.
    #[error("input '{id}': {message}")]
    InvalidInput {
        /// The identifier of the invalid input.
        id: String,
        /// Description of the validation failure.
        message: String,
    },
}
