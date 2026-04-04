use nframework_nfw_domain::features::service_management::layer_dependency_matrix::LayerDependencyMatrix;

use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::services::abstraction::generated_project_dependency_inspector::GeneratedProjectDependencyInspector;

#[derive(Debug, Clone)]
pub struct ServiceLayerDependencyValidator<I>
where
    I: GeneratedProjectDependencyInspector,
{
    dependency_inspector: I,
    matrix: LayerDependencyMatrix,
}

impl<I> ServiceLayerDependencyValidator<I>
where
    I: GeneratedProjectDependencyInspector,
{
    pub fn new(dependency_inspector: I) -> Self {
        Self {
            dependency_inspector,
            matrix: LayerDependencyMatrix::standard(),
        }
    }

    pub fn validate(&self, service_root: &std::path::Path) -> Result<(), AddServiceError> {
        let edges = self
            .dependency_inspector
            .inspect_dependencies(service_root)
            .map_err(AddServiceError::Internal)?;

        let violations = self.matrix.validate_edges(&edges);
        if violations.is_empty() {
            return Ok(());
        }

        Err(AddServiceError::DependencyRuleViolation(
            violations.join("; "),
        ))
    }
}
