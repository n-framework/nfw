use crate::features::service_management::models::add_service_command_request::AddServiceCommandRequest;
use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_generator_resolution::ServiceGeneratorResolution;
use crate::features::service_management::services::abstractions::service_generator_prompt::ServiceGeneratorPrompt;
use crate::features::service_management::services::abstractions::service_generator_selector::{
    ServiceGeneratorSelectionContext, ServiceGeneratorSelector,
};
use n_framework_core_cli_abstractions::{InteractiveError, InteractivePrompt, Logger};
use serde_yaml::Value as YamlValue;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct AddServiceInputResolutionService<S, P, Q>
where
    S: ServiceGeneratorSelector,
    P: ServiceGeneratorPrompt,
    Q: InteractivePrompt + Logger,
{
    generator_selector: S,
    generator_prompt: P,
    prompt_service: Q,
}

impl<S, P, Q> AddServiceInputResolutionService<S, P, Q>
where
    S: ServiceGeneratorSelector,
    P: ServiceGeneratorPrompt,
    Q: InteractivePrompt + Logger,
{
    pub fn new(generator_selector: S, generator_prompt: P, prompt_service: Q) -> Self {
        Self {
            generator_selector,
            generator_prompt,
            prompt_service,
        }
    }

    pub fn resolve_service_name(
        &self,
        request: &AddServiceCommandRequest,
    ) -> Result<String, AddServiceError> {
        if let Some(service_name) = request.service_name.clone() {
            return Ok(service_name);
        }

        if request.is_non_interactive() || !self.prompt_service.is_interactive() {
            return Err(AddServiceError::MissingRequiredInput("name".to_owned()));
        }

        self.prompt_service
            .text("Service name", None)
            .map_err(map_prompt_error)
    }

    pub fn resolve_generator_selection(
        &self,
        request: &AddServiceCommandRequest,
        workspace_root: &Path,
        nfw_yaml: &YamlValue,
    ) -> Result<ServiceGeneratorResolution, AddServiceError> {
        if let Some(generator_id) = request.generator_id.as_deref() {
            return self.generator_selector.resolve_service_generator(
                generator_id,
                ServiceGeneratorSelectionContext {
                    workspace_root,
                    nfw_yaml,
                },
            );
        }

        if request.is_non_interactive() {
            return Err(AddServiceError::MissingRequiredInput(
                "generator".to_owned(),
            ));
        }

        let spinner = if !request.is_non_interactive() && self.prompt_service.is_interactive() {
            Some(
                self.prompt_service
                    .spinner("Discovering generators...")
                    .map_err(|e| AddServiceError::PromptFailed(e.to_string()))?,
            )
        } else {
            None
        };

        let generators_result = self.generator_selector.list_service_generators();

        if let Some(spinner) = &spinner {
            if generators_result.is_ok() {
                spinner.stop("Generators discovered");
            } else {
                spinner.error("Failed to discover generators");
            }
        }

        let generators = generators_result?;

        if generators.is_empty() {
            return Err(AddServiceError::GeneratorNotFound(
                "<service generator>".to_owned(),
            ));
        }

        let selected_generator_id = self
            .generator_prompt
            .select_generator(&generators)
            .map_err(AddServiceError::PromptFailed)?;

        self.generator_selector.resolve_service_generator(
            &selected_generator_id,
            ServiceGeneratorSelectionContext {
                workspace_root,
                nfw_yaml,
            },
        )
    }
}

fn map_prompt_error(error: InteractiveError) -> AddServiceError {
    if error.is_cancelled() {
        return AddServiceError::Interrupted;
    }

    AddServiceError::PromptFailed(error.to_string())
}
