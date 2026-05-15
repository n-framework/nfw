use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use n_framework_nfw_core_application::features::service_management::models::service_generator_resolution::ServiceGeneratorResolution;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_generator_prompt::ServiceGeneratorPrompt;

#[derive(Debug, Clone)]
pub struct InteractiveServiceGeneratorPrompt<P>
where
    P: InteractivePrompt + Logger + Clone,
{
    prompt_service: P,
}

impl<P> InteractiveServiceGeneratorPrompt<P>
where
    P: InteractivePrompt + Logger + Clone,
{
    pub fn new(prompt_service: P) -> Self {
        Self { prompt_service }
    }
}

impl<P> ServiceGeneratorPrompt for InteractiveServiceGeneratorPrompt<P>
where
    P: InteractivePrompt + Logger + Clone,
{
    fn select_generator(
        &self,
        generators: &[ServiceGeneratorResolution],
    ) -> Result<String, String> {
        let options = generators
            .iter()
            .map(|generator| {
                SelectOption::new(
                    generator.qualified_generator_id(),
                    generator.qualified_generator_id(),
                )
                .with_description(&generator.description)
            })
            .collect::<Vec<_>>();

        let selected_index = self
            .prompt_service
            .select_index("Select a service generator:", &options, Some(0))
            .map_err(|error| error.to_string())?;

        generators
            .get(selected_index)
            .map(ServiceGeneratorResolution::qualified_generator_id)
            .ok_or_else(|| "selected generator index is out of bounds".to_owned())
    }
}
