use crate::features::service_management::models::service_generator_resolution::ServiceGeneratorResolution;

pub trait ServiceGeneratorPrompt {
    fn select_generator(&self, generators: &[ServiceGeneratorResolution])
    -> Result<String, String>;
}
