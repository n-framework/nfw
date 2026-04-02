use nframework_nfw_application::features::template_management::services::abstraction::template_catalog_discovery_service::TemplateCatalogDiscoveryService;

#[derive(Debug, Clone)]
pub struct RefreshTemplatesCliCommand<S>
where
    S: TemplateCatalogDiscoveryService,
{
    discovery_service: S,
}

impl<S> RefreshTemplatesCliCommand<S>
where
    S: TemplateCatalogDiscoveryService,
{
    pub fn new(discovery_service: S) -> Self {
        Self { discovery_service }
    }

    pub fn execute(&self) -> Result<(), String> {
        let (catalogs, warnings) = self
            .discovery_service
            .discover_catalogs()
            .map_err(|error| error.to_string())?;

        for warning in warnings {
            eprintln!("warning: {warning}");
        }

        let template_count: usize = catalogs.iter().map(|catalog| catalog.templates.len()).sum();
        println!(
            "Refreshed {} source(s), {} template(s).",
            catalogs.len(),
            template_count
        );

        Ok(())
    }
}
