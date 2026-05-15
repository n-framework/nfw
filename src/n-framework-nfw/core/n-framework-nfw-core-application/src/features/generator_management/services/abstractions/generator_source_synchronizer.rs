use std::path::PathBuf;

use n_framework_nfw_core_domain::features::generator_management::generator_source::GeneratorSource;

pub trait GeneratorSourceSynchronizer {
    fn sync_source(&self, source: &GeneratorSource) -> Result<(PathBuf, Option<String>), String>;
    fn clear_source_cache(&self, source_name: &str) -> Result<(), String>;
}
