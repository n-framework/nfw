use std::path::PathBuf;

use nframework_nfw_domain::features::template_management::template_source::TemplateSource;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NfwConfiguration {
    pub template_sources: Vec<TemplateSource>,
    pub cache_directory: PathBuf,
    pub config_directory: PathBuf,
}

impl NfwConfiguration {
    pub fn new(
        template_sources: Vec<TemplateSource>,
        cache_directory: PathBuf,
        config_directory: PathBuf,
    ) -> Self {
        Self {
            template_sources,
            cache_directory,
            config_directory,
        }
    }
}
