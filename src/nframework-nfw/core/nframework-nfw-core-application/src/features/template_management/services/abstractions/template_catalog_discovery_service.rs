use nframework_nfw_core_domain::features::template_management::template_catalog::TemplateCatalog;

use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;

pub trait TemplateCatalogDiscoveryService {
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<TemplateCatalog>, Vec<String>), TemplatesServiceError>;
}
