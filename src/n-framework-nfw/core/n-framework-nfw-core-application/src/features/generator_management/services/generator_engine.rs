use crate::features::generator_management::models::generator_error::GeneratorError;
use n_framework_nfw_core_domain::features::generator_management::generator_config::GeneratorConfig;
use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;
use std::path::Path;

/// Orchestrates the transformation of generators into output files.
///
/// The engine is responsible for iterating over the steps defined in a `GeneratorConfig`,
/// resolving paths, rendering content via a renderer, and performing injections.
///
/// **Security Note:** Implementations MUST ensure path traversal protection. Output paths
/// must be constrained to the `output_root`, preventing generators from writing outside
/// the intended boundaries via malicious placeholder values or generator source paths.
pub trait GeneratorEngine {
    /// Executes the generator configuration.
    ///
    /// # Arguments
    /// * `config` - The validated generator configuration to execute.
    /// * `generator_root` - The root directory where generator sources are located.
    /// * `output_root` - The root directory where files should be generated.
    /// * `parameters` - The parameters to use during rendering (e.g. Name, Namespace).
    fn execute(
        &self,
        config: &GeneratorConfig,
        generator_root: &Path,
        output_root: &Path,
        parameters: &GeneratorParameters,
    ) -> Result<(), GeneratorError>;
}
