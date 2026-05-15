use n_framework_nfw_core_application::features::generator_management::commands::add_generator_source::add_generator_source_command::AddGeneratorSourceCommand;
use n_framework_nfw_core_application::features::generator_management::commands::add_generator_source::add_generator_source_command_handler::AddGeneratorSourceCommandHandler;
use n_framework_nfw_core_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::git_repository::GitRepository;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::validator::Validator;
use crate::startup::cli_service_collection_factory::CliServiceCollection;

/// Thin CLI presentation layer for adding a generator source.
/// Delegates all business logic to the application layer command handler.
#[derive(Debug, Clone)]
pub struct AddSourceCliCommand<H> {
    handler: H,
}

impl<H> AddSourceCliCommand<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

impl<CS, V, G> AddSourceCliCommand<AddGeneratorSourceCommandHandler<CS, V, G>>
where
    CS: ConfigStore,
    V: Validator,
    G: GitRepository,
{
    pub fn execute(&self, name: &str, url: &str) -> Result<(), String> {
        let command = AddGeneratorSourceCommand::new(name.to_owned(), url.to_owned());
        self.handler
            .handle(&command)
            .map_err(|error| error.to_string())?;
        println!("Generator source '{name}' added.");
        Ok(())
    }
}

impl AddSourceCliCommand<()> {
    pub fn handle(
        command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        AddSourceCliCommand::new(context.add_generator_source_command_handler.clone()).execute(
            command
                .option("name")
                .ok_or_else(|| "[exit:1] Generator source name is required".to_string())?,
            command
                .option("url")
                .ok_or_else(|| "[exit:1] Generator source URL is required".to_string())?,
        )
    }
}
