use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_generator_resolution::ServiceGeneratorResolution;
use serde_yaml::Value as YamlValue;
use std::path::Path;

/// Provides context required for querying the service generator catalog.
pub struct ServiceGeneratorSelectionContext<'a> {
    /// The root path of the NFramework workspace.
    pub workspace_root: &'a Path,
    /// The parsed workspace configuration (nfw.yaml).
    pub nfw_yaml: &'a YamlValue,
}

impl<'a> ServiceGeneratorSelectionContext<'a> {
    pub fn new(workspace_root: &'a Path, nfw_yaml: &'a YamlValue) -> Self {
        Self {
            workspace_root,
            nfw_yaml,
        }
    }
}

pub trait ServiceGeneratorSelector {
    /// Resolves a generator based on its identifier within the given generator selection context.
    fn resolve_service_generator(
        &self,
        generator_identifier: &str,
        context: ServiceGeneratorSelectionContext<'_>,
    ) -> Result<ServiceGeneratorResolution, AddServiceError>;

    fn list_service_generators(&self) -> Result<Vec<ServiceGeneratorResolution>, AddServiceError>;
}
