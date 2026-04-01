use std::path::PathBuf;

use nframework_nfw_domain::features::template_management::template_source::TemplateSource;

pub trait TemplateSourceSynchronizer {
    fn sync_source(&self, source: &TemplateSource) -> Result<(PathBuf, Option<String>), String>;
}
