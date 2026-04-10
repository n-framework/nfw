use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command::AddServiceCommand;
use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command_handler::AddServiceCommandHandler;
use n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_provenance_store::ServiceProvenanceStore;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_prompt::ServiceTemplatePrompt;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_renderer::ServiceTemplateRenderer;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_selector::ServiceTemplateSelector;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_core_cli_abstractions::PromptService;

#[derive(Debug, Clone)]
pub struct AddServiceCliCommand<H> {
    handler: H,
}

impl<H> AddServiceCliCommand<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

impl<D, S, P, Q, R, PS> AddServiceCliCommand<AddServiceCommandHandler<D, S, P, Q, R, PS>>
where
    D: WorkingDirectoryProvider,
    S: ServiceTemplateSelector,
    P: ServiceTemplatePrompt,
    Q: PromptService,
    R: ServiceTemplateRenderer,
    PS: ServiceProvenanceStore,
{
    pub fn execute(
        &self,
        service_name: Option<&str>,
        template_id: Option<&str>,
        no_input: bool,
        is_interactive_terminal: bool,
    ) -> Result<(), AddServiceError> {
        let command = AddServiceCommand::new(
            service_name.map(ToOwned::to_owned),
            template_id.map(ToOwned::to_owned),
            no_input,
            is_interactive_terminal,
        );

        let result = self.handler.handle(&command)?;
        println!(
            "Created service '{}' at '{}'.",
            result.service_name,
            result.output_path.display()
        );
        println!("Template: {}", result.template_id);
        println!("Template version: {}", result.template_version);

        Ok(())
    }
}
