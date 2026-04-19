use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use n_framework_nfw_core_application::features::service_management::models::service_template_resolution::ServiceTemplateResolution;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_prompt::ServiceTemplatePrompt;

#[derive(Debug, Clone)]
pub struct InteractiveServiceTemplatePrompt<P>
where
    P: InteractivePrompt + Logger + Clone,
{
    prompt_service: P,
}

impl<P> InteractiveServiceTemplatePrompt<P>
where
    P: InteractivePrompt + Logger + Clone,
{
    pub fn new(prompt_service: P) -> Self {
        Self { prompt_service }
    }
}

impl<P> ServiceTemplatePrompt for InteractiveServiceTemplatePrompt<P>
where
    P: InteractivePrompt + Logger + Clone,
{
    fn select_template(&self, templates: &[ServiceTemplateResolution]) -> Result<String, String> {
        let options = templates
            .iter()
            .map(|template| {
                SelectOption::new(
                    template.qualified_template_id(),
                    template.qualified_template_id(),
                )
                .with_description(&template.description)
            })
            .collect::<Vec<_>>();

        let selected_index = self
            .prompt_service
            .select_index("Select a service template:", &options, Some(0))
            .map_err(|error| error.to_string())?;

        templates
            .get(selected_index)
            .map(ServiceTemplateResolution::qualified_template_id)
            .ok_or_else(|| "selected template index is out of bounds".to_owned())
    }
}
