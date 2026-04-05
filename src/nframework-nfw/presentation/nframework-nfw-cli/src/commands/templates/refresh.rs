use nframework_nfw_core_application::features::template_management::commands::refresh_templates::refresh_templates_command::RefreshTemplatesCommand;
use nframework_nfw_core_application::features::template_management::commands::refresh_templates::refresh_templates_command_handler::RefreshTemplatesCommandHandler;
use nframework_nfw_core_application::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;

/// Thin CLI presentation layer for refreshing templates.
/// Delegates all business logic to the application layer command handler.
#[derive(Debug, Clone)]
pub struct RefreshTemplatesCliCommand<H> {
    handler: H,
}

impl<H> RefreshTemplatesCliCommand<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

impl<S> RefreshTemplatesCliCommand<RefreshTemplatesCommandHandler<S>>
where
    S: TemplateCatalogDiscoveryService,
{
    pub fn execute(&self) -> Result<(), String> {
        let command = RefreshTemplatesCommand;
        let result = self
            .handler
            .handle(&command)
            .map_err(|error| error.to_string())?;

        for warning in result.warnings {
            eprintln!("warning: {warning}");
        }

        println!(
            "Refreshed {} source(s), {} template(s).",
            result.source_count, result.template_count
        );

        Ok(())
    }
}
