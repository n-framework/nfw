use n_framework_nfw_core_application::features::template_management::commands::remove_template_source::remove_template_source_command::RemoveTemplateSourceCommand;
use n_framework_nfw_core_application::features::template_management::commands::remove_template_source::remove_template_source_command_handler::RemoveTemplateSourceCommandHandler;
use n_framework_nfw_core_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_source_synchronizer::TemplateSourceSynchronizer;

/// Thin CLI presentation layer for removing a template source.
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

impl<CS, S> RemoveSourceCliCommand<RemoveTemplateSourceCommandHandler<CS, S>>
where
    CS: ConfigStore,
    S: TemplateSourceSynchronizer,
{
    pub fn execute(&self, name: &str) -> Result<(), String> {
        let command = RemoveTemplateSourceCommand::new(name.to_owned());
        self.handler
            .handle(&command)
            .map_err(|error| error.to_string())?;
        println!("Template source '{name}' removed.");
        Ok(())
    }
}

impl RemoveSourceCliCommand<()> {
    pub fn handle(
        command: &dyn n_framework_core_cli_abstractions::Command,
        context: &crate::startup::cli_service_collection_factory::CliServiceCollection,
    ) -> Result<(), String> {
        RemoveSourceCliCommand::new(context.remove_template_source_command_handler.clone()).execute(
            command
                .option("name")
                .ok_or_else(|| "[exit:1] Option 'name' is required".to_string())?,
        )
    }
}
