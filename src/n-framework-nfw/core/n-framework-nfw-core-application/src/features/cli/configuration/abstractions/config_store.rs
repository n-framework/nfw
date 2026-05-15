use n_framework_nfw_core_domain::features::generator_management::generator_source::GeneratorSource;

pub trait ConfigStore {
    fn load_sources(&self) -> Result<Vec<GeneratorSource>, String>;
    fn save_sources(&self, sources: &[GeneratorSource]) -> Result<(), String>;
}
