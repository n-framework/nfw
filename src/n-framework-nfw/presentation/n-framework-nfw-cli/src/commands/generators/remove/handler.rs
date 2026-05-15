use n_framework_nfw_core_application::features::generator_management::commands::remove_generator_source::remove_generator_source_command::RemoveGeneratorSourceCommand;
use n_framework_nfw_core_application::features::generator_management::commands::remove_generator_source::remove_generator_source_command_handler::RemoveGeneratorSourceCommandHandler;
use n_framework_nfw_core_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::generator_source_synchronizer::GeneratorSourceSynchronizer;
use crate::startup::cli_service_collection_factory::CliServiceCollection;

/// Thin CLI presentation layer for removing a generator source.
/// Delegates all business logic to the application layer command handler.
#[derive(Debug, Clone)]
pub struct RemoveSourceCliCommand<H> {
    handler: H,
}

impl<H> RemoveSourceCliCommand<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

impl<CS, S> RemoveSourceCliCommand<RemoveGeneratorSourceCommandHandler<CS, S>>
where
    CS: ConfigStore,
    S: GeneratorSourceSynchronizer,
{
    pub fn execute(&self, name: &str) -> Result<(), String> {
        let command = RemoveGeneratorSourceCommand::new(name.to_owned());
        self.handler
            .handle(&command)
            .map_err(|error| error.to_string())?;
        println!("Generator source '{name}' removed.");
        Ok(())
    }
}

impl RemoveSourceCliCommand<()> {
    pub fn handle(
        _command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        RemoveSourceCliCommand::new(context.remove_generator_source_command_handler.clone())
            .execute(
                _command
                    .option("name")
                    .ok_or_else(|| "[exit:1] Generator source name is required".to_string())?,
            )
    }
}
