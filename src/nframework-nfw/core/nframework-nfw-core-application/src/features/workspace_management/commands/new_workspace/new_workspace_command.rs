use std::path::PathBuf;

use crate::features::workspace_management::models::new_command_request::NewCommandRequest;

/// Command to create a new workspace.
///
/// This command encapsulates all the input needed to create a workspace.
/// The handler is responsible for resolving defaults, validating, and executing.
#[derive(Debug, Clone)]
pub struct NewWorkspaceCommand {
    pub workspace_name: Option<String>,
    pub template_id: Option<String>,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

impl NewWorkspaceCommand {
    pub fn new(
        workspace_name: Option<String>,
        template_id: Option<String>,
        no_input: bool,
        is_interactive_terminal: bool,
    ) -> Self {
        Self {
            workspace_name,
            template_id,
            no_input,
            is_interactive_terminal,
        }
    }

    /// Converts the command to a NewCommandRequest for use by legacy services.
    pub fn to_request(&self) -> NewCommandRequest {
        NewCommandRequest::new(
            self.workspace_name.clone(),
            self.template_id.clone(),
            self.no_input,
            self.is_interactive_terminal,
        )
    }
}

/// Result of successfully executing a NewWorkspaceCommand.
#[derive(Debug, Clone)]
pub struct NewWorkspaceCommandResult {
    pub workspace_name: String,
    pub output_path: PathBuf,
    pub template_id: String,
    pub namespace_base: String,
}
