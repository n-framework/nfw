use std::path::Path;

use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_generation_plan::ServiceGenerationPlan;

pub trait ServiceTemplateRenderer {
    fn render_service(&self, plan: &ServiceGenerationPlan) -> Result<(), AddServiceError>;

    fn cleanup_partial_output(&self, output_root: &Path) -> Result<(), AddServiceError>;
}
