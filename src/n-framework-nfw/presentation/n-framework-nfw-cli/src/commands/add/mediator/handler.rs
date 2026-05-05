use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use crate::cli_error::CliError;
use crate::commands::add::utils::find_presentation_layers;
use crate::startup::cli_service_collection_factory::CliServiceCollection;
use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;
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

    pub fn execute(&self, request: AddMediatorRequest) -> Result<(), CliError> {
        self.prompt
            .intro("Add Mediator Module")
            .map_err(|e| CliError::internal(e.to_string()))?;

        let workspace_context = self.handler.get_workspace_context()?;
        let services = self.handler.extract_services(&workspace_context)?;

        if services.is_empty() {
            return Err(AddArtifactError::WorkspaceError(
                "No services found in workspace. Add a service first.".to_string(),
            )
            .into());
        }

        let selected_service = if let Some(name) = request.service_name {
            services
                .into_iter()
                .find(|s| s.name() == name)
                .ok_or_else(|| {
                    AddArtifactError::WorkspaceError(format!(
                        "Service '{}' not found in workspace.",
                        name
                    ))
                })?
        } else if (request.no_input || !request.is_interactive_terminal) && services.len() == 1 {
            services.into_iter().next().ok_or_else(|| {
                AddArtifactError::WorkspaceError("No service found in workspace.".to_string())
            })?
        } else {
            let options: Vec<SelectOption> = services
                .iter()
                .map(|s| SelectOption::new(s.name(), s.name()))
                .collect();
            let selected = self
                .prompt
                .select("Select a service to add mediator to:", &options, Some(0))
                .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

            services
                .into_iter()
                .find(|s| s.name() == selected.value())
                .ok_or_else(|| {
                    AddArtifactError::WorkspaceError(format!(
                        "Selected service '{}' not found in workspace.",
                        selected.value()
                    ))
                })?
        };

        let layers = find_presentation_layers(
            workspace_context.workspace_root(),
            selected_service.path(),
            selected_service.name(),
        )?;

        if layers.is_empty() {
            return Err(CliError::internal(
                "No presentation layer found. Please run `nfw add webapi` first.",
            ));
        }

        let selected_layer = if layers.len() == 1 {
            layers[0].clone()
        } else if request.no_input || !request.is_interactive_terminal {
            return Err(CliError::internal(
                "Multiple presentation layers found. Please run in interactive mode to select one.",
            ));
        } else {
            let options: Vec<SelectOption> =
                layers.iter().map(|l| SelectOption::new(l, l)).collect();
            let selected = self
                .prompt
                .select("Select presentation layer:", &options, Some(0))
                .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;
            selected.value().to_string()
        };

        let spinner = self
            .prompt
            .spinner(&format!(
                "Adding mediator module to '{}'...",
                selected_service.name()
            ))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        let command =
            AddMediatorCommand::new(selected_service.clone(), workspace_context, selected_layer)
                .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        if let Err(e) = self.handler.handle(&command) {
            let error_id = format!(
                "{:x}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_else(|_| std::time::Duration::from_secs(0))
                    .as_micros()
            );
            spinner.error(&format!(
                "Failed to add mediator (Log ID: {}): {}",
                error_id, e
            ));
            tracing::error!("[{}] Failed to add mediator: {:?}", error_id, e);
            return Err(CliError::silent(
                ExitCodes::from_add_artifact_error(&e) as i32,
                e.to_string(),
            ));
        }

        spinner.success(&format!(
            "Mediator module added to '{}'",
            selected_service.name()
        ));

        self.prompt
            .outro(&format!(
                "Successfully added Mediator module to '{}'.",
                selected_service.name()
            ))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        Ok(())
    }
}

impl AddMediatorCliCommand<(), (), (), n_framework_core_cli_cliclack::CliclackPromptService> {
    pub fn handle(
        command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        use std::io::{self, IsTerminal};
        let is_interactive_terminal = io::stdin().is_terminal() && io::stdout().is_terminal();

        AddMediatorCliCommand::new(
            context.add_mediator_command_handler.clone(),
            n_framework_core_cli_cliclack::CliclackPromptService::new(),
        )
        .execute(AddMediatorRequest {
            no_input: command.option("no-input").is_some(),
            is_interactive_terminal,
            service_name: command.option("service"),
        })
        .map_err(|error| error.to_string())
    }
}
