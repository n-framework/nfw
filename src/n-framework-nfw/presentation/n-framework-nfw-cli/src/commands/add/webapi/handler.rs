use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use crate::cli_error::CliError;
use crate::startup::cli_service_collection_factory::CliServiceCollection;
use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;
use n_framework_nfw_core_application::features::template_management::commands::add_webapi::add_webapi_command::{AddWebApiCommand, WebApiConfig};
use n_framework_nfw_core_application::features::template_management::commands::add_webapi::add_webapi_command_handler::AddWebApiCommandHandler;
pub use n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use crate::utils::generate_error_id;

/// Controls whether the command runs interactively or in headless mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractivityMode {
    /// User will be prompted for missing information
    Interactive,
    /// All prompts are disabled; automation-friendly
    NonInteractive,
}

impl InteractivityMode {
    pub fn is_interactive(self) -> bool {
        matches!(self, Self::Interactive)
    }

    pub fn is_non_interactive(self) -> bool {
        matches!(self, Self::NonInteractive)
    }
}

#[derive(Debug, Clone)]
pub struct AddWebApiCliCommand<W, R, E, P> {
    handler: AddWebApiCommandHandler<W, R, E>,
    prompt: P,
}

pub struct AddWebApiRequest<'a> {
    pub mode: InteractivityMode,
    pub service_name: Option<&'a str>,
}

impl<W, R, E, P> AddWebApiCliCommand<W, R, E, P>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
    P: InteractivePrompt + Logger,
{
    pub fn new(handler: AddWebApiCommandHandler<W, R, E>, prompt: P) -> Self {
        Self { handler, prompt }
    }

    pub fn execute(&self, request: AddWebApiRequest) -> Result<(), CliError> {
        if request.mode.is_non_interactive() && request.service_name.is_none() {
            return Err(CliError::internal(
                "Service name is required when --no-input is set. Use the --service option or run in interactive mode.",
            ));
        }

        self.prompt
            .intro("Add WebAPI Module")
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
        } else if request.mode.is_non_interactive() && services.len() == 1 {
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
                .select("Select a service to add WebAPI to:", &options, Some(0))
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

        let spinner = self
            .prompt
            .spinner(&format!(
                "Adding WebAPI module to '{}'...",
                selected_service.name()
            ))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        let command = AddWebApiCommand::new(
            selected_service.clone(),
            workspace_context,
            WebApiConfig::default(),
        );

        if let Err(e) = self.handler.handle(&command) {
            let error_id = generate_error_id();
            spinner.error(&format!(
                "Failed to add WebAPI (Log ID: {}): {}",
                error_id, e
            ));
            tracing::error!("[{}] Failed to add WebAPI: {:?}", error_id, e);
            return Err(CliError::silent(
                ExitCodes::from_add_artifact_error(&e) as i32,
                e.to_string(),
            ));
        }

        spinner.success(&format!(
            "WebAPI module added to '{}'",
            selected_service.name()
        ));

        self.prompt
            .outro(&format!(
                "Successfully added WebAPI module to '{}'.",
                selected_service.name()
            ))
            .map_err(|e| AddArtifactError::WorkspaceError(e.to_string()))?;

        Ok(())
    }
}

impl AddWebApiCliCommand<(), (), (), n_framework_core_cli_cliclack::CliclackPromptService> {
    pub fn handle(
        command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        AddWebApiCliCommand::new(
            context.add_webapi_command_handler.clone(),
            n_framework_core_cli_cliclack::CliclackPromptService::new(),
        )
        .execute(AddWebApiRequest {
            mode: if command.option("no-input").is_some() {
                InteractivityMode::NonInteractive
            } else {
                InteractivityMode::Interactive
            },
            service_name: command.option("service"),
        })
        .map_err(|error| error.to_string())
    }
}

#[cfg(test)]
#[path = "handler.tests.rs"]
mod tests;
