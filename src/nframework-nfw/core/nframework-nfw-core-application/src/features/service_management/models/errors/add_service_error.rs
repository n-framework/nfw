use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddServiceError {
    MissingRequiredInput(String),
    InvalidServiceName(String),
    InvalidWorkspaceContext(String),
    TemplateNotFound(String),
    InvalidTemplateType {
        template_id: String,
        template_type: String,
    },
    AmbiguousTemplate(String),
    PromptFailed(String),
    TargetDirectoryAlreadyExists(String),
    RenderFailed(String),
    DependencyRuleViolation(String),
    HealthEndpointsMissing(String),
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
            Self::TemplateNotFound(template) => write!(
                f,
                "service template '{template}' was not found; use 'nfw templates list'"
            ),
            Self::InvalidTemplateType {
                template_id,
                template_type,
            } => write!(
                f,
                "template '{template_id}' has type '{template_type}', expected 'service'"
            ),
            Self::AmbiguousTemplate(template) => {
                write!(f, "template identifier '{template}' is ambiguous")
            }
            Self::PromptFailed(reason) => write!(f, "interactive template prompt failed: {reason}"),
            Self::TargetDirectoryAlreadyExists(path) => {
                write!(f, "target directory '{path}' already exists")
            }
            Self::RenderFailed(reason) => write!(f, "failed to render service template: {reason}"),
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
                write!(f, "failed to persist service template provenance: {reason}")
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
