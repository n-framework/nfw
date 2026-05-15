use std::path::PathBuf;

use n_framework_nfw_core_domain::features::generator_management::generator_source::GeneratorSource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NfwConfiguration {
    pub generator_sources: Vec<GeneratorSource>,
    pub cache_directory: PathBuf,
    pub config_directory: PathBuf,
}

impl NfwConfiguration {
    pub fn new(
        generator_sources: Vec<GeneratorSource>,
        cache_directory: PathBuf,
        config_directory: PathBuf,
    ) -> Self {
        Self {
            generator_sources,
            cache_directory,
            config_directory,
        }
    }
}
