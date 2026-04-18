use n_framework_nfw_core_application::features::template_management::commands::generate::generate_command::GenerateCommand;
use n_framework_nfw_core_application::features::template_management::commands::generate::generate_command_handler::GenerateCommandHandler;
pub use n_framework_nfw_core_application::features::template_management::models::errors::generate_error::GenerateError;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;

/// CLI command implementation for the `generate` subcommand.
#[derive(Debug, Clone)]
pub struct GenerateCliCommand<W, R, E> {
    handler: GenerateCommandHandler<W, R, E>,
}

/// Request parameters for a generation operation.
#[derive(Debug, Clone)]
pub struct GenerateRequest<'a> {
    /// The type of component to generate (e.g. 'command', 'feature').
    pub generator_type: &'a str,
    /// The name of the new component.
    pub name: &'a str,
    /// Optional feature name to associate the component with.
    pub feature: Option<&'a str>,
    /// Optional arbitrary parameters as 'Key=Value' pairs.
    pub params: Option<&'a str>,
}

impl<W, R, E> GenerateCliCommand<W, R, E>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
{
    pub fn new(handler: GenerateCommandHandler<W, R, E>) -> Self {
        Self { handler }
    }

    pub fn execute(&self, request: GenerateRequest) -> Result<(), GenerateError> {
        let command = GenerateCommand::new(
            request.generator_type,
            request.name,
            request.feature.map(ToOwned::to_owned),
            request.params.map(ToOwned::to_owned),
        );

        self.handler.handle(&command)?;

        println!(
            "Generated '{}' '{}' successfully.",
            request.generator_type, request.name
        );
        
        Ok(())
    }
}
