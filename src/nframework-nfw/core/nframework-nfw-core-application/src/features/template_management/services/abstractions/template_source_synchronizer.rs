use std::path::PathBuf;

use nframework_nfw_core_domain::features::template_management::template_source::TemplateSource;

pub trait TemplateSourceSynchronizer {
    fn sync_source(&self, source: &TemplateSource) -> Result<(PathBuf, Option<String>), String>;
    fn clear_source_cache(&self, source_name: &str) -> Result<(), String>;
}
