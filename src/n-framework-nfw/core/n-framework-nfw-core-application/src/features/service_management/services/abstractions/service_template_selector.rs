use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_template_resolution::ServiceTemplateResolution;
use serde_yaml::Value as YamlValue;
use std::path::Path;

/// Provides context required for querying the service template catalog.
pub struct ServiceTemplateSelectionContext<'a> {
    /// The root path of the NFramework workspace.
    pub workspace_root: &'a Path,
    /// The parsed workspace configuration (nfw.yaml).
    pub nfw_yaml: &'a YamlValue,
}

impl<'a> ServiceTemplateSelectionContext<'a> {
    pub fn new(workspace_root: &'a Path, nfw_yaml: &'a YamlValue) -> Self {
        Self {
            workspace_root,
            nfw_yaml,
        }
    }
}

pub trait ServiceTemplateSelector {
    /// Resolves a template based on its identifier within the given template selection context.
    fn resolve_service_template(
        &self,
        template_identifier: &str,
        context: ServiceTemplateSelectionContext<'_>,
    ) -> Result<ServiceTemplateResolution, AddServiceError>;

    fn list_service_templates(&self) -> Result<Vec<ServiceTemplateResolution>, AddServiceError>;
}
