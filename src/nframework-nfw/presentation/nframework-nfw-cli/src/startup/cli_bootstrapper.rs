use crate::startup::cli_service_collection_factory::{
    CliServiceCollection, CliServiceCollectionFactory,
};
use nframework_nfw_application::features::template_management::commands::ensure_default_source::ensure_default_source_command::EnsureDefaultSourceCommand;

#[derive(Debug, Default, Clone, Copy)]
pub struct CliBootstrapper;

impl CliBootstrapper {
    pub fn bootstrap() -> Result<CliServiceCollection, String> {
        let service_collection = CliServiceCollectionFactory::create();

        // Bootstrap: ensure default template source is registered.
        // Done after construction so I/O failures return a Result
        // instead of panicking inside the factory.
        service_collection
            .ensure_default_source_command_handler
            .handle(&EnsureDefaultSourceCommand)
            .map_err(|error| error.to_string())?;

        Ok(service_collection)
    }
}
