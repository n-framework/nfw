use crate::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use std::path::Path;

/// Orchestrates the transformation of templates into output files.
///
/// The engine is responsible for iterating over the steps defined in a `TemplateConfig`,
/// resolving paths, rendering content via a renderer, and performing injections.
/// 
/// **Security Note:** Implementations MUST ensure path traversal protection. Output paths
/// must be constrained to the `output_root`, preventing templates from writing outside
/// the intended boundaries via malicious placeholder values or template source paths.
pub trait TemplateEngine {
    /// Executes the template configuration.
    ///
    /// # Arguments
    /// * `config` - The validated template configuration to execute.
    /// * `template_root` - The root directory where template sources are located.
    /// * `output_root` - The root directory where files should be generated.
    /// * `parameters` - The parameters to use during rendering (e.g. Name, Namespace).
    fn execute(
        &self,
        config: &TemplateConfig,
        template_root: &Path,
        output_root: &Path,
        parameters: &TemplateParameters,
    ) -> Result<(), TemplateError>;
}
