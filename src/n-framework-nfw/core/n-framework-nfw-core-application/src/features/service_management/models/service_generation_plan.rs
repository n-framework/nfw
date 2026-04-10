use std::collections::BTreeMap;
use std::path::PathBuf;

use n_framework_nfw_core_domain::features::versioning::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceGenerationPlan {
    pub service_name: String,
    pub output_root: PathBuf,
    pub template_cache_path: PathBuf,
    pub template_id: String,
    pub template_version: Version,
    pub namespace: String,
    pub placeholder_values: BTreeMap<String, String>,
}
