use crate::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use crate::features::workspace_management::models::new_command_request::NewCommandRequest;
use crate::features::workspace_management::services::abstractions::workspace_name_validator::WorkspaceNameValidator;

#[derive(Debug, Clone)]
pub struct NewCommandValidator<V>
where
    V: WorkspaceNameValidator,
{
    workspace_name_validator: V,
}

impl<V> NewCommandValidator<V>
where
    V: WorkspaceNameValidator,
{
    pub fn new(workspace_name_validator: V) -> Self {
        Self {
            workspace_name_validator,
        }
    }

    pub fn validate_request(&self, request: &NewCommandRequest) -> Result<(), WorkspaceNewError> {
        if request.no_input && !request.is_interactive_terminal && request.workspace_name.is_none()
        {
            return Err(WorkspaceNewError::MissingRequiredInput(
                "workspace-name".to_owned(),
            ));
        }

        if let Some(workspace_name) = &request.workspace_name
            && !self
                .workspace_name_validator
                .is_valid_workspace_name(workspace_name)
        {
            return Err(WorkspaceNewError::InvalidWorkspaceName(
                workspace_name.clone(),
            ));
        }

        Ok(())
    }
}
