use std::path::PathBuf;

use nframework_nfw_core_domain::features::versioning::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ServiceTemplateResolution {
    pub source_name: String,
    pub template_name: String,
    pub template_id: String,
    pub resolved_version: Version,
    pub template_type: String,
    pub template_cache_path: PathBuf,
    pub description: String,
}

impl ServiceTemplateResolution {
    pub fn qualified_template_id(&self) -> String {
        format!("{}/{}", self.source_name, self.template_id)
    }
}
