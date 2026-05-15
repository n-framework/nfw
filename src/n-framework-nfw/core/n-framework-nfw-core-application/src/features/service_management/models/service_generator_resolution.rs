use std::path::PathBuf;

use n_framework_nfw_core_domain::features::versioning::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceGeneratorResolution {
    pub source_name: String,
    pub generator_name: String,
    pub generator_id: String,
    pub resolved_version: Version,
    pub generator_type: String,
    pub generator_cache_path: PathBuf,
    pub description: String,
}

impl ServiceGeneratorResolution {
    pub fn qualified_generator_id(&self) -> String {
        format!("{}/{}", self.source_name, self.generator_id)
    }
}
