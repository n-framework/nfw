use n_framework_nfw_core_application::features::generator_management::commands::refresh_generators::refresh_generators_command::RefreshGeneratorsCommand;
use n_framework_nfw_core_application::features::generator_management::commands::refresh_generators::refresh_generators_command_handler::RefreshGeneratorsCommandHandler;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::generator_catalog_discovery_service::GeneratorCatalogDiscoveryService;
use crate::startup::cli_service_collection_factory::CliServiceCollection;

/// Thin CLI presentation layer for refreshing generators.
/// Delegates all business logic to the application layer command handler.
#[derive(Debug, Clone)]
pub struct RefreshGeneratorsCliCommand<H> {
    handler: H,
}

impl<H> RefreshGeneratorsCliCommand<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

impl<S> RefreshGeneratorsCliCommand<RefreshGeneratorsCommandHandler<S>>
where
    S: GeneratorCatalogDiscoveryService,
{
    pub fn execute(&self) -> Result<(), String> {
        let command = RefreshGeneratorsCommand;
        let result = self
            .handler
            .handle(&command)
            .map_err(|error| error.to_string())?;

        for warning in result.warnings {
            eprintln!("warning: {warning}");
        }

        println!(
            "Refreshed {} source(s), {} generator(s).",
            result.source_count, result.generator_count
        );

        Ok(())
    }
}

impl RefreshGeneratorsCliCommand<()> {
    pub fn handle(
        _command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        RefreshGeneratorsCliCommand::new(context.refresh_generators_command_handler.clone())
            .execute()
    }
}
