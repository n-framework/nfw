use n_framework_nfw_core_domain::features::generator_management::generator_config::GeneratorConfig;
use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;
use std::path::Path;

use crate::features::generator_management::models::generator_error::GeneratorError;

/// Defines a high-level service for executing code generation from generators.
pub trait GenerateService: Send + Sync {
    /// Executes a generation request.
    fn generate(
        &self,
        config: &GeneratorConfig,
        generator_root: &Path,
        output_root: &Path,
        parameters: &GeneratorParameters,
    ) -> Result<(), GeneratorError>;
}
