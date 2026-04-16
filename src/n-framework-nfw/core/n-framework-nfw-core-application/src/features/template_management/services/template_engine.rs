use std::collections::BTreeMap;
use std::path::Path;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use crate::features::template_management::models::template_error::TemplateError;

pub trait TemplateEngine {
    fn execute(
        &self,
        config: &TemplateConfig,
        template_root: &Path,
        output_root: &Path,
        placeholders: &BTreeMap<String, String>,
    ) -> Result<(), TemplateError>;
}
