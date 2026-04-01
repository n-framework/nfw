use nframework_nfw_application::features::template_management::queries::list_templates::list_templates_query::ListTemplatesQuery;
use nframework_nfw_application::features::template_management::queries::list_templates::list_templates_query_handler::ListTemplatesQueryHandler;
use nframework_nfw_application::features::template_management::services::abstraction::template_listing_service::TemplateListingService;

#[derive(Debug, Clone)]
pub struct TemplatesCliCommand<S>
where
    S: TemplateListingService,
{
    query_handler: ListTemplatesQueryHandler<S>,
}

impl<S> TemplatesCliCommand<S>
where
    S: TemplateListingService,
{
    pub fn new(query_handler: ListTemplatesQueryHandler<S>) -> Self {
        Self { query_handler }
    }

    pub fn execute(&self) -> Result<(), String> {
        let result = self
            .query_handler
            .handle(ListTemplatesQuery)
            .map_err(|error| error.to_string())?;

        for warning in result.warnings {
            eprintln!("warning: {warning}");
        }

        if result.templates.is_empty() {
            println!("No templates found.");
            return Ok(());
        }

        for template in result.templates {
            println!(
                "{}/{} {} ({})",
                template.source_name, template.id, template.name, template.version
            );
            println!("  {}", template.description);
        }

        Ok(())
    }
}
