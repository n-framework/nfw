use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCodes {
    Success = 0,
    ValidationError = 2,
    NotFound = 3,
    Conflict = 4,
    ExternalDependencyFailure = 10,
    InternalError = 1,
}

impl ExitCodes {
    pub fn from_workspace_new_error(error: &WorkspaceNewError) -> Self {
        match error {
            WorkspaceNewError::InvalidWorkspaceName(_)
            | WorkspaceNewError::MissingWorkspaceName
            | WorkspaceNewError::MissingRequiredInput(_)
            | WorkspaceNewError::InvalidOptionCombination(_) => Self::ValidationError,
            WorkspaceNewError::TemplateNotFound(_) => Self::NotFound,
            WorkspaceNewError::AmbiguousTemplate(_)
            | WorkspaceNewError::TargetDirectoryNotEmpty(_) => Self::Conflict,
            WorkspaceNewError::PromptFailed(_) | WorkspaceNewError::WriteFailed(_) => {
                Self::ExternalDependencyFailure
            }
            WorkspaceNewError::Internal(_) => Self::InternalError,
        }
    }

    pub fn from_add_service_error(error: &AddServiceError) -> Self {
        match error {
            AddServiceError::MissingRequiredInput(_)
            | AddServiceError::InvalidServiceName(_)
            | AddServiceError::InvalidTemplateType { .. } => Self::ValidationError,
            AddServiceError::InvalidWorkspaceContext(_)
            | AddServiceError::TargetDirectoryAlreadyExists(_)
            | AddServiceError::AmbiguousTemplate(_) => Self::Conflict,
            AddServiceError::TemplateNotFound(_) => Self::NotFound,
            AddServiceError::PromptFailed(_)
            | AddServiceError::RenderFailed(_)
            | AddServiceError::ProvenanceWriteFailed(_)
            | AddServiceError::CleanupFailed(_) => Self::ExternalDependencyFailure,
            AddServiceError::DependencyRuleViolation(_)
            | AddServiceError::HealthEndpointsMissing(_)
            | AddServiceError::Internal(_) => Self::InternalError,
            AddServiceError::Interrupted => Self::Success,
        }
    }
}
