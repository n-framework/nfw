use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use std::path::Path;

use crate::features::template_management::models::template_error::TemplateError;

/// Defines a high-level service for executing code generation from templates.
pub trait GenerateService: Send + Sync {
    /// Executes a generation request.
    fn generate(
        &self,
        config: &TemplateConfig,
        template_root: &Path,
        output_root: &Path,
        parameters: &TemplateParameters,
    ) -> Result<(), TemplateError>;
}
