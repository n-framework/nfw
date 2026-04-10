use n_framework_nfw_core_application::features::template_management::commands::add_template_source::add_template_source_command::AddTemplateSourceCommand;
use n_framework_nfw_core_application::features::template_management::commands::add_template_source::add_template_source_command_handler::AddTemplateSourceCommandHandler;
use n_framework_nfw_core_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use n_framework_nfw_core_application::features::template_management::services::abstractions::git_repository::GitRepository;
use n_framework_nfw_core_application::features::template_management::services::abstractions::validator::Validator;

/// Thin CLI presentation layer for adding a template source.
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

impl<CS, V, G> AddSourceCliCommand<AddTemplateSourceCommandHandler<CS, V, G>>
where
    CS: ConfigStore,
    V: Validator,
    G: GitRepository,
{
    pub fn execute(&self, name: &str, url: &str) -> Result<(), String> {
        let command = AddTemplateSourceCommand::new(name.to_owned(), url.to_owned());
        self.handler
            .handle(&command)
            .map_err(|error| error.to_string())?;
        println!("Template source '{name}' added.");
        Ok(())
    }
}
