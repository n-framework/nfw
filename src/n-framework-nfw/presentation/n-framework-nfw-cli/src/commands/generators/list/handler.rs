use n_framework_nfw_core_application::features::generator_management::queries::list_generators::list_generators_query::ListGeneratorsQuery;
use n_framework_nfw_core_application::features::generator_management::queries::list_generators::list_generators_query_handler::ListGeneratorsQueryHandler;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::generator_listing_service::GeneratorListingService;
use crate::startup::cli_service_collection_factory::CliServiceCollection;
use n_framework_nfw_core_application::features::generator_management::queries::list_generators::list_generators_query_result::ListGeneratorsQueryResult;
use n_framework_nfw_core_application::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;

/// Thin CLI presentation layer for listing generators.
/// Delegates all business logic to the application layer query handler.
#[derive(Debug, Clone)]
pub struct GeneratorsCliCommand<H> {
    query_handler: H,
}

impl<H> GeneratorsCliCommand<H>
where
    H: GeneratorListingQueryHandler,
{
    pub fn new(query_handler: H) -> Self {
        Self { query_handler }
    }

    pub fn execute(&self) -> Result<(), String> {
        let result = self
            .query_handler
            .handle_list_generators()
            .map_err(|error| error.to_string())?;

        for warning in result.warnings {
            eprintln!("warning: {warning}");
        }

        if result.generators.is_empty() {
            println!("No generators found.");
            return Ok(());
        }

        for generator in result.generators {
            println!(
                "{}/{} {} ({})",
                generator.source_name, generator.id, generator.name, generator.version
            );
            println!("  {}", generator.description);
        }

        Ok(())
    }
}

impl GeneratorsCliCommand<()> {
    pub fn handle(
        _command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        GeneratorsCliCommand::new(context.list_generators_query_handler.clone()).execute()
    }
}

/// Abstraction for the query handler to avoid generic type explosion in CLI.
pub trait GeneratorListingQueryHandler {
    fn handle_list_generators(&self) -> Result<ListGeneratorsQueryResult, GeneratorsServiceError>;
}

impl<S> GeneratorListingQueryHandler for ListGeneratorsQueryHandler<S>
where
    S: GeneratorListingService,
{
    fn handle_list_generators(&self) -> Result<ListGeneratorsQueryResult, GeneratorsServiceError> {
        self.handle(ListGeneratorsQuery)
    }
}
