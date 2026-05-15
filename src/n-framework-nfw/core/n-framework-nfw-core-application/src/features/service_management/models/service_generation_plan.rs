use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;
use n_framework_nfw_core_domain::features::versioning::version::Version;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct ServiceGenerationPlan {
    pub service_name: String,
    pub output_root: PathBuf,
    pub generator_cache_path: PathBuf,
    pub generator_id: String,
    pub generator_version: Version,
    pub namespace: String,
    pub placeholder_values: GeneratorParameters,
}
