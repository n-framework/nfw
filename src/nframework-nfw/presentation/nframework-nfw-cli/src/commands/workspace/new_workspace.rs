use nframework_nfw_core_application::features::workspace_management::commands::new_workspace::new_workspace_command::NewWorkspaceCommand;
use nframework_nfw_core_application::features::workspace_management::commands::new_workspace::new_workspace_command_handler::NewWorkspaceCommandHandler;
use nframework_nfw_core_application::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use nframework_core_cli_abstractions::PromptService;
use nframework_nfw_core_application::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use nframework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use nframework_nfw_core_application::features::workspace_management::services::abstractions::workspace_name_validator::WorkspaceNameValidator;
use nframework_nfw_core_application::features::workspace_management::services::abstractions::workspace_writer::WorkspaceWriter;

/// Thin CLI presentation layer for workspace creation.
/// Delegates all business logic to the application layer command handler.
#[derive(Debug, Clone)]
pub struct NewWorkspaceCliCommand<H> {
    handler: H,
}

impl<H> NewWorkspaceCliCommand<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

impl<P, V, T, W, D, PS> NewWorkspaceCliCommand<NewWorkspaceCommandHandler<P, V, T, W, D, PS>>
where
    P: PromptService,
    V: WorkspaceNameValidator + Clone,
    T: TemplateCatalogDiscoveryService + Clone,
    W: WorkspaceWriter,
    D: WorkingDirectoryProvider,
    PS: PromptService + Clone,
{
    pub fn execute(
        &self,
        workspace_name: Option<&str>,
        template_id: Option<&str>,
        no_input: bool,
        is_interactive_terminal: bool,
    ) -> Result<(), WorkspaceNewError> {
        let command = NewWorkspaceCommand::new(
            workspace_name.map(ToOwned::to_owned),
            template_id.map(ToOwned::to_owned),
            no_input,
            is_interactive_terminal,
        );

        let result = self.handler.handle(&command)?;

        println!(
            "Created workspace '{}' at '{}'.",
            result.workspace_name,
            result.output_path.display()
        );
        println!("Template: {}", result.template_id);
        println!("Namespace: {}", result.namespace_base);

        Ok(())
    }
}
