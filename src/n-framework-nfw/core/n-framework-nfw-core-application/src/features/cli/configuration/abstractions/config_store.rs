use n_framework_nfw_core_domain::features::template_management::template_source::TemplateSource;

pub trait ConfigStore {
    fn load_sources(&self) -> Result<Vec<TemplateSource>, String>;
    fn save_sources(&self, sources: &[TemplateSource]) -> Result<(), String>;
}
