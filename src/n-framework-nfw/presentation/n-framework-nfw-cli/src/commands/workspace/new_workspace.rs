use n_framework_nfw_core_application::features::workspace_management::commands::new_workspace::new_workspace_command::NewWorkspaceCommand;
use n_framework_nfw_core_application::features::workspace_management::commands::new_workspace::new_workspace_command_handler::NewWorkspaceCommandHandler;
use n_framework_nfw_core_application::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use n_framework_core_cli_abstractions::{InteractivePrompt, Logger};
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::workspace_name_validator::WorkspaceNameValidator;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::workspace_writer::WorkspaceWriter;

/// Thin CLI presentation layer for workspace creation.
/// Delegates all business logic to the application layer command handler.
pub struct NewWorkspaceCliCommand<H, P>
where
    P: Logger,
{
    handler: H,
    logger: P,
}

impl<H, P> NewWorkspaceCliCommand<H, P>
where
    P: Logger,
{
    pub fn new(handler: H, logger: P) -> Self {
        Self { handler, logger }
    }
}

impl<P, V, T, W, D, PS, L> NewWorkspaceCliCommand<NewWorkspaceCommandHandler<P, V, T, W, D, PS>, L>
where
    P: InteractivePrompt + Logger,
    V: WorkspaceNameValidator + Clone,
    T: TemplateCatalogDiscoveryService + Clone,
    W: WorkspaceWriter,
    D: WorkingDirectoryProvider,
    PS: InteractivePrompt + Logger + Clone,
    L: Logger,
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

        let success_message = format!(
            "Created workspace '{}' at '{}'.\nTemplate: {}\nNamespace: {}",
            result.workspace_name,
            result.output_path.display(),
            result.template_id,
            result.namespace_base
        );

        let _ = self.logger.outro(&success_message);

        Ok(())
    }
}
