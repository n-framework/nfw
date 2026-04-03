use crate::features::template_management::commands::refresh_templates::refresh_templates_command::{
    RefreshTemplatesCommand, RefreshTemplatesCommandResult,
};
use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::services::abstraction::template_catalog_discovery_service::TemplateCatalogDiscoveryService;

/// Command handler for refreshing template catalogs.
///
/// This handler orchestrates the template refresh process:
/// 1. Discovers all catalogs from registered sources
/// 2. Counts templates across all sources
/// 3. Returns summary with warnings
#[derive(Debug, Clone)]
pub struct RefreshTemplatesCommandHandler<S>
where
    S: TemplateCatalogDiscoveryService,
{
    discovery_service: S,
}

impl<S> RefreshTemplatesCommandHandler<S>
where
    S: TemplateCatalogDiscoveryService,
{
    pub fn new(discovery_service: S) -> Self {
        Self { discovery_service }
    }

    pub fn handle(
        &self,
        _command: &RefreshTemplatesCommand,
    ) -> Result<RefreshTemplatesCommandResult, TemplatesServiceError> {
        let (catalogs, warnings) = self.discovery_service.discover_catalogs()?;

        let template_count: usize = catalogs.iter().map(|catalog| catalog.templates.len()).sum();

        Ok(RefreshTemplatesCommandResult {
            source_count: catalogs.len(),
            template_count,
            warnings,
        })
    }
}
