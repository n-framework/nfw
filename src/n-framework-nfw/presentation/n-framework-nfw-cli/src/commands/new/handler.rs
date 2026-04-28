use std::io::{self, IsTerminal};
use n_framework_nfw_core_application::features::workspace_management::commands::new_workspace::new_workspace_command::NewWorkspaceCommand;
use n_framework_core_cli_abstractions::Command;

pub struct NewWorkspaceCliCommand {}

impl NewWorkspaceCliCommand {
    pub fn handle(
        command: &dyn Command,
        context: &crate::startup::cli_service_collection_factory::CliServiceCollection,
    ) -> Result<(), String> {
        let workspace_name = command.option("workspace-name").map(|s| s.to_string());
        let template_id = command.option("template").map(|s| s.to_string());
        let no_input = command.option("no-input").is_some();
        let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

        let cmd = NewWorkspaceCommand::new(
            workspace_name,
            template_id,
            no_input,
            is_interactive_terminal,
        );

        context
            .new_workspace_command_handler
            .handle(&cmd)
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}
