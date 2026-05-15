use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddServiceError {
    MissingRequiredInput(String),
    InvalidServiceName(String),
    InvalidWorkspaceContext(String),
    GeneratorNotFound(String),
    InvalidGeneratorType {
        generator_id: String,
        generator_type: String,
    },
    AmbiguousGenerator(String),
    PromptFailed(String),
    TargetDirectoryAlreadyExists(String),
    RenderFailed(String),
    DependencyRuleViolation(String),
    HealthEndpointsMissing(String),
    GeneratorReadError(String),
    GeneratorConfigError(String),
    ProvenanceWriteFailed(String),
    CleanupFailed(String),
    Interrupted,
    Internal(String),
}

impl Display for AddServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingRequiredInput(field) => write!(
                f,
                "required input '{field}' is missing in non-interactive mode"
            ),
            Self::InvalidServiceName(service_name) => write!(
                f,
                "service name '{service_name}' is invalid; use letters, numbers, '-' or '_'"
            ),
            Self::InvalidWorkspaceContext(message) => {
                write!(f, "invalid workspace context: {message}")
            }
            Self::GeneratorNotFound(generator) => write!(
                f,
                "service generator '{generator}' was not found; use 'nfw generators list'"
            ),
            Self::InvalidGeneratorType {
                generator_id,
                generator_type,
            } => write!(
                f,
                "generator '{generator_id}' has type '{generator_type}', expected 'service'"
            ),
            Self::AmbiguousGenerator(generator) => {
                write!(f, "generator identifier '{generator}' is ambiguous")
            }
            Self::PromptFailed(reason) => {
                write!(f, "interactive generator prompt failed: {reason}")
            }
            Self::TargetDirectoryAlreadyExists(path) => {
                write!(f, "target directory '{path}' already exists")
            }
            Self::RenderFailed(reason) => write!(f, "failed to render service generator: {reason}"),
            Self::DependencyRuleViolation(reason) => {
                write!(
                    f,
                    "generated service violates layer dependency rules: {reason}"
                )
            }
            Self::HealthEndpointsMissing(reason) => {
                write!(f, "generated API health endpoints are missing: {reason}")
            }
            Self::ProvenanceWriteFailed(reason) => {
                write!(
                    f,
                    "failed to persist service generator provenance: {reason}"
                )
            }
            Self::GeneratorReadError(reason) => {
                write!(f, "failed to read service generator structure: {reason}")
            }
            Self::GeneratorConfigError(reason) => {
                write!(f, "service generator configuration is invalid: {reason}")
            }
            Self::CleanupFailed(reason) => {
                write!(
                    f,
                    "service generation failed and cleanup was incomplete: {reason}"
                )
            }
            Self::Interrupted => write!(f, "service generation interrupted"),
            Self::Internal(reason) => write!(f, "service generation failed: {reason}"),
        }
    }
}

impl std::error::Error for AddServiceError {}
