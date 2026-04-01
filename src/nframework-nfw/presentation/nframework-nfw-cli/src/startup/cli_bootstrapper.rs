use crate::startup::cli_service_collection_factory::{
    CliServiceCollection, CliServiceCollectionFactory,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct CliBootstrapper;

impl CliBootstrapper {
    pub fn bootstrap() -> Result<CliServiceCollection, String> {
        let service_collection = CliServiceCollectionFactory::create();

        service_collection
            .templates_service
            .ensure_default_source_registered()
            .map_err(|error| error.to_string())?;

        Ok(service_collection)
    }
}
