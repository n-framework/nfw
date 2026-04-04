use crate::features::service_management::models::service_template_resolution::ServiceTemplateResolution;

pub trait ServiceTemplatePrompt {
    fn select_template(&self, templates: &[ServiceTemplateResolution]) -> Result<String, String>;
}
