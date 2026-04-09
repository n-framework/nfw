use crate::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use crate::features::workspace_management::models::new_command_request::NewCommandRequest;
use crate::features::workspace_management::services::abstractions::workspace_name_validator::WorkspaceNameValidator;
use nframework_core_cli_abstractions::PromptService;

#[derive(Debug, Clone)]
pub struct InputResolutionService<P, V>
where
    P: PromptService,
    V: WorkspaceNameValidator,
{
    prompt_service: P,
    workspace_name_validator: V,
}

impl<P, V> InputResolutionService<P, V>
where
    P: PromptService,
    V: WorkspaceNameValidator,
{
    pub fn new(prompt_service: P, workspace_name_validator: V) -> Self {
        Self {
            prompt_service,
            workspace_name_validator,
        }
    }

    pub fn resolve_workspace_name(
        &self,
        request: &NewCommandRequest,
    ) -> Result<String, WorkspaceNewError> {
        if let Some(workspace_name) = request.workspace_name.clone() {
            self.validate_workspace_name(&workspace_name)?;
            return Ok(workspace_name);
        }

        if request.no_input || !self.prompt_service.is_interactive() {
            return Err(WorkspaceNewError::MissingWorkspaceName);
        }

        let resolved = self
            .prompt_service
            .text("Workspace name", None)
            .map_err(|error| WorkspaceNewError::PromptFailed(error.to_string()))?;

        self.validate_workspace_name(&resolved)?;

        Ok(resolved)
    }

    fn validate_workspace_name(&self, workspace_name: &str) -> Result<(), WorkspaceNewError> {
        if workspace_name.trim().is_empty() {
            return Err(WorkspaceNewError::MissingWorkspaceName);
        }

        if !self
            .workspace_name_validator
            .is_valid_workspace_name(workspace_name)
        {
            return Err(WorkspaceNewError::InvalidWorkspaceName(
                workspace_name.to_owned(),
            ));
        }

        Ok(())
    }
}
