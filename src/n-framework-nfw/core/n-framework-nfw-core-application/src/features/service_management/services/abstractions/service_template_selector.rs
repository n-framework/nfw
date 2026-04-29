use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_template_resolution::ServiceTemplateResolution;
use serde_yaml::Value as YamlValue;
use std::path::Path;

pub trait ServiceTemplateSelector {
    fn resolve_service_template(
        &self,
        template_identifier: &str,
        workspace_root: &Path,
        nfw_yaml: &YamlValue,
    ) -> Result<ServiceTemplateResolution, AddServiceError>;

    fn list_service_templates(&self) -> Result<Vec<ServiceTemplateResolution>, AddServiceError>;
}
