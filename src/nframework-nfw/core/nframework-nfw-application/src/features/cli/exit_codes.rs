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
}
