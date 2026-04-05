use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_template_resolution::ServiceTemplateResolution;

pub trait ServiceTemplateSelector {
    fn resolve_service_template(
        &self,
        template_identifier: &str,
    ) -> Result<ServiceTemplateResolution, AddServiceError>;

    fn list_service_templates(&self) -> Result<Vec<ServiceTemplateResolution>, AddServiceError>;
}
