use n_framework_nfw_core_application::features::template_management::queries::list_templates::list_templates_query::ListTemplatesQuery;
use n_framework_nfw_core_application::features::template_management::queries::list_templates::list_templates_query_handler::ListTemplatesQueryHandler;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_listing_service::TemplateListingService;
use crate::startup::cli_service_collection_factory::CliServiceCollection;
use n_framework_nfw_core_application::features::template_management::queries::list_templates::list_templates_query_result::ListTemplatesQueryResult;
use n_framework_nfw_core_application::features::template_management::models::errors::templates_service_error::TemplatesServiceError;

/// Thin CLI presentation layer for listing templates.
/// Delegates all business logic to the application layer query handler.
#[derive(Debug, Clone)]
pub struct TemplatesCliCommand<H> {
    query_handler: H,
}

impl<H> TemplatesCliCommand<H>
where
    H: TemplateListingQueryHandler,
{
    pub fn new(query_handler: H) -> Self {
        Self { query_handler }
    }

    pub fn execute(&self) -> Result<(), String> {
        let result = self
            .query_handler
            .handle_list_templates()
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

impl TemplatesCliCommand<()> {
    pub fn handle(
        _command: &dyn n_framework_core_cli_abstractions::Command,
        context: &CliServiceCollection,
    ) -> Result<(), String> {
        TemplatesCliCommand::new(context.list_templates_query_handler.clone()).execute()
    }
}

/// Abstraction for the query handler to avoid generic type explosion in CLI.
pub trait TemplateListingQueryHandler {
    fn handle_list_templates(&self) -> Result<ListTemplatesQueryResult, TemplatesServiceError>;
}

impl<S> TemplateListingQueryHandler for ListTemplatesQueryHandler<S>
where
    S: TemplateListingService,
{
    fn handle_list_templates(&self) -> Result<ListTemplatesQueryResult, TemplatesServiceError> {
        self.handle(ListTemplatesQuery)
    }
}
