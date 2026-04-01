use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::models::listed_template::ListedTemplate;

pub trait TemplateListingService {
    fn list_templates(&self) -> Result<(Vec<ListedTemplate>, Vec<String>), TemplatesServiceError>;
}
