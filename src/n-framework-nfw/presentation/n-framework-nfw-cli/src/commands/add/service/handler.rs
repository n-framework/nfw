use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;
use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command::AddServiceCommand;
use n_framework_nfw_core_application::features::service_management::commands::add_service::add_service_command_handler::AddServiceCommandHandler;
use n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_provenance_store::ServiceProvenanceStore;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_prompt::ServiceTemplatePrompt;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_renderer::ServiceTemplateRenderer;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_selector::ServiceTemplateSelector;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_core_cli_abstractions::{InteractivePrompt, Logger};
use crate::cli_error::CliError;

#[derive(Debug, Clone)]
pub struct AddServiceCliCommand<H, P> {
    handler: H,
    prompt: P,
}

impl<H, P> AddServiceCliCommand<H, P> {
    pub fn new(handler: H, prompt: P) -> Self {
        Self { handler, prompt }
    }
}

impl<D, S, P, Q, R, PS, Q2> AddServiceCliCommand<AddServiceCommandHandler<D, S, P, Q, R, PS>, Q2>
where
    D: WorkingDirectoryProvider,
    S: ServiceTemplateSelector,
    P: ServiceTemplatePrompt,
    Q: InteractivePrompt + Logger,
    R: ServiceTemplateRenderer,
    PS: ServiceProvenanceStore,
    Q2: InteractivePrompt + Logger,
{
    pub fn execute(
        &self,
        service_name: Option<&str>,
        template_id: Option<&str>,
        no_input: bool,
        is_interactive_terminal: bool,
    ) -> Result<(), CliError> {
        self.prompt
            .intro("Add Service")
            .map_err(|e| CliError::internal(e.to_string()))?;

        tracing::info!(
            "Executing add-service command (name: {:?}, template: {:?}, no_input: {})",
            service_name,
            template_id,
            no_input
        );

        let command = AddServiceCommand::new(
            service_name.map(ToOwned::to_owned),
            template_id.map(ToOwned::to_owned),
            no_input,
            is_interactive_terminal,
        );

        let plan = self.handler.prepare_generation_plan(&command)?;

        let spinner = self
            .prompt
            .spinner(&format!("Adding service '{}'...", plan.service_name))
            .map_err(|e| AddServiceError::Internal(e.to_string()))?;

        let result = self.handler.execute_generation_plan(&plan).map_err(|e| {
            spinner.error(&format!("Failed to add service: {e}"));
            tracing::error!("Failed to add service: {}", e);
            e
        });

        if let Err(e) = result {
            return Err(CliError::silent(
                ExitCodes::from_add_service_error(&e) as i32,
                e.to_string(),
            ));
        }
        let result = result.unwrap();

        spinner.success(&format!(
            "Service '{}' added successfully",
            result.service_name
        ));

        self.prompt
            .outro(&format!(
                "Created service '{}' at '{}'.\nTemplate: {}\nTemplate version: {}",
                result.service_name,
                result.output_path.display(),
                result.template_id,
                result.template_version
            ))
            .map_err(|e| AddServiceError::Internal(e.to_string()))?;

        tracing::info!(
            "Successfully created service '{}' using template '{}'",
            result.service_name,
            result.template_id
        );

        Ok(())
    }
}

impl AddServiceCliCommand<(), n_framework_core_cli_cliclack::CliclackPromptService> {
    pub fn handle(
        command: &dyn n_framework_core_cli_abstractions::Command,
        context: &crate::startup::cli_service_collection_factory::CliServiceCollection,
    ) -> Result<(), String> {
        use std::io::{self, IsTerminal};
        let no_input = command.option("no-input").is_some();
        let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

        AddServiceCliCommand::new(
            context.add_service_command_handler.clone(),
            n_framework_core_cli_cliclack::CliclackPromptService::new(),
        )
        .execute(
            command.option("name"),
            command.option("template"),
            no_input,
            is_interactive_terminal,
        )
        .map_err(|error| error.to_string())
    }
}
