use crate::features::generator_management::commands::refresh_generators::refresh_generators_command::{
    RefreshGeneratorsCommand, RefreshGeneratorsCommandResult,
};
use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use crate::features::generator_management::services::abstractions::generator_catalog_discovery_service::GeneratorCatalogDiscoveryService;

/// Command handler for refreshing generator catalogs.
///
/// This handler orchestrates the generator refresh process:
/// 1. Discovers all catalogs from registered sources
/// 2. Counts generators across all sources
/// 3. Returns summary with warnings
#[derive(Debug, Clone)]
pub struct RefreshGeneratorsCommandHandler<S>
where
    S: GeneratorCatalogDiscoveryService,
{
    discovery_service: S,
}

impl<S> RefreshGeneratorsCommandHandler<S>
where
    S: GeneratorCatalogDiscoveryService,
{
    pub fn new(discovery_service: S) -> Self {
        Self { discovery_service }
    }

    pub fn handle(
        &self,
        _command: &RefreshGeneratorsCommand,
    ) -> Result<RefreshGeneratorsCommandResult, GeneratorsServiceError> {
        let (catalogs, warnings) = self.discovery_service.discover_catalogs()?;

        let generator_count: usize = catalogs
            .iter()
            .map(|catalog| catalog.generators.len())
            .sum();

        Ok(RefreshGeneratorsCommandResult {
            source_count: catalogs.len(),
            generator_count,
            warnings,
        })
    }
}
