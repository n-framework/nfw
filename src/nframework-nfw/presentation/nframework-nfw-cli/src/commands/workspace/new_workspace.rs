use nframework_core_cli_abstraction::PromptService;
use nframework_nfw_application::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use nframework_nfw_application::features::workspace_management::models::new_command_request::NewCommandRequest;
use nframework_nfw_application::features::workspace_management::services::workspace_initialization_service::WorkspaceInitializationService;
use nframework_nfw_application::features::workspace_management::services::abstraction::working_directory_provider::WorkingDirectoryProvider;

#[derive(Debug, Clone)]
pub struct NewWorkspaceCliCommand<S> {
    workspace_initialization_service: S,
}

impl<S> NewWorkspaceCliCommand<S> {
    pub fn new(workspace_initialization_service: S) -> Self {
        Self {
            workspace_initialization_service,
        }
    }
}

impl<P, V, T, W, D, PS> NewWorkspaceCliCommand<WorkspaceInitializationService<P, V, T, W, D, PS>>
where
    P: PromptService,
    V: nframework_nfw_application::features::workspace_management::services::abstraction::workspace_name_validator::WorkspaceNameValidator
        + Clone,
    T: nframework_nfw_application::features::template_management::services::abstraction::template_catalog_discovery_service::TemplateCatalogDiscoveryService
        + Clone,
    W: nframework_nfw_application::features::workspace_management::services::abstraction::workspace_writer::WorkspaceWriter,
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
        let request = NewCommandRequest::new(
            workspace_name.map(ToOwned::to_owned),
            template_id.map(ToOwned::to_owned),
            no_input,
            is_interactive_terminal,
        );

        let result = self.workspace_initialization_service.execute(request)?;

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
