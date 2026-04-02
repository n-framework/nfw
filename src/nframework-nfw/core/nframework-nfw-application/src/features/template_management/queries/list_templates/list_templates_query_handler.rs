use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::queries::list_templates::list_templates_query::ListTemplatesQuery;
use crate::features::template_management::queries::list_templates::list_templates_query_result::ListTemplatesQueryResult;
use crate::features::template_management::services::abstraction::template_listing_service::TemplateListingService;

#[derive(Debug, Clone)]
pub struct ListTemplatesQueryHandler<S>
where
    S: TemplateListingService,
{
    templates_service: S,
}

impl<S> ListTemplatesQueryHandler<S>
where
    S: TemplateListingService,
{
    pub fn new(templates_service: S) -> Self {
        Self { templates_service }
    }

    pub fn handle(
        &self,
        _query: ListTemplatesQuery,
    ) -> Result<ListTemplatesQueryResult, TemplatesServiceError> {
        let (templates, warnings) = self.templates_service.list_templates()?;
        Ok(ListTemplatesQueryResult {
            templates,
            warnings,
        })
    }
}
