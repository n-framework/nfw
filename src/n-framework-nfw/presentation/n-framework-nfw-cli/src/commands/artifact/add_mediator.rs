use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use n_framework_nfw_core_application::features::template_management::commands::add_mediator::add_mediator_command::AddMediatorCommand;
use n_framework_nfw_core_application::features::template_management::commands::add_mediator::add_mediator_command_handler::AddMediatorCommandHandler;
pub use n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;

#[derive(Debug, Clone)]
pub struct AddMediatorCliCommand<W, R, E, P> {
    handler: AddMediatorCommandHandler<W, R, E>,
    prompt: P,
}

pub struct AddMediatorRequest<'a> {
    pub no_input: bool,
    pub is_interactive_terminal: bool,
    pub service_name: Option<&'a str>,
}

impl<W, R, E, P> AddMediatorCliCommand<W, R, E, P>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
    P: InteractivePrompt + Logger,
{
    pub fn new(handler: AddMediatorCommandHandler<W, R, E>, prompt: P) -> Self {
        Self { handler, prompt }
    }

    pub fn execute(&self, request: AddMediatorRequest) -> Result<(), AddArtifactError> {
        self.prompt
            .intro("Add Mediator Module")
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        let workspace_context = self.handler.get_workspace_context()?;
        let services = self.handler.extract_services(&workspace_context)?;

        if services.is_empty() {
            return Err(AddArtifactError::WorkspaceError(
                "No services found in workspace. Add a service first.".to_string(),
            ));
        }

        let selected_service =
            if (request.no_input || !request.is_interactive_terminal) && services.len() == 1 {
                services.into_iter().next().unwrap()
            } else if let Some(name) = request.service_name {
                services
                    .into_iter()
                    .find(|s| s.name == name)
                    .ok_or_else(|| {
                        AddArtifactError::WorkspaceError(format!(
                            "Service '{}' not found in workspace.",
                            name
                        ))
                    })?
            } else {
                let options: Vec<SelectOption> = services
                    .iter()
                    .map(|s| SelectOption::new(&s.name, &s.name))
                    .collect();
                let selected = self
                    .prompt
                    .select("Select a service to add mediator to:", &options, Some(0))
                    .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

                services
                    .into_iter()
                    .find(|s| s.name == selected.value())
                    .unwrap()
            };

        let spinner = self
            .prompt
            .spinner(&format!(
                "Adding mediator module to '{}'...",
                selected_service.name
            ))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        let service_display_name = selected_service.name.clone();

        let command = AddMediatorCommand::new(&service_display_name);

        self.handler
            .handle(&command, workspace_context, &selected_service)
            .map_err(|e| {
                spinner.error(&format!("Failed to add mediator: {}", e));
                e
            })?;

        spinner.success(&format!(
            "Mediator module added to '{}'",
            service_display_name
        ));

        self.prompt
            .outro(&format!(
                "Successfully added Mediator module to '{}'.",
                service_display_name
            ))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        Ok(())
    }
}
