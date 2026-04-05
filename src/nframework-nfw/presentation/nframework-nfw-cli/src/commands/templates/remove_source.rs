use nframework_nfw_application::features::template_management::commands::remove_template_source::remove_template_source_command::RemoveTemplateSourceCommand;
use nframework_nfw_application::features::template_management::commands::remove_template_source::remove_template_source_command_handler::RemoveTemplateSourceCommandHandler;
use nframework_nfw_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use nframework_nfw_application::features::template_management::services::abstractions::template_source_synchronizer::TemplateSourceSynchronizer;

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
