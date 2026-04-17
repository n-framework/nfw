use crate::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use std::collections::BTreeMap;
use std::path::Path;

pub trait TemplateEngine {
    fn execute(
        &self,
        config: &TemplateConfig,
        template_root: &Path,
        output_root: &Path,
        placeholders: &BTreeMap<String, String>,
    ) -> Result<(), TemplateError>;
}
