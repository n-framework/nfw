use crate::features::service_management::models::add_service_command_request::AddServiceCommandRequest;
use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_template_resolution::ServiceTemplateResolution;
use crate::features::service_management::services::abstractions::service_template_prompt::ServiceTemplatePrompt;
use crate::features::service_management::services::abstractions::service_template_selector::ServiceTemplateSelector;
use n_framework_core_cli_abstractions::{InteractiveError, InteractivePrompt, Logger};

#[derive(Debug, Clone)]
pub struct AddServiceInputResolutionService<S, P, Q>
where
    S: ServiceTemplateSelector,
    P: ServiceTemplatePrompt,
    Q: InteractivePrompt + Logger,
{
    template_selector: S,
    template_prompt: P,
    prompt_service: Q,
}

impl<S, P, Q> AddServiceInputResolutionService<S, P, Q>
where
    S: ServiceTemplateSelector,
    P: ServiceTemplatePrompt,
    Q: InteractivePrompt + Logger,
{
    pub fn new(template_selector: S, template_prompt: P, prompt_service: Q) -> Self {
        Self {
            template_selector,
            template_prompt,
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

    pub fn resolve_template_selection(
        &self,
        request: &AddServiceCommandRequest,
    ) -> Result<ServiceTemplateResolution, AddServiceError> {
        if let Some(template_id) = request.template_id.as_deref() {
            return self.template_selector.resolve_service_template(template_id);
        }

        if request.is_non_interactive() {
            return Err(AddServiceError::MissingRequiredInput("template".to_owned()));
        }

        let spinner = if !request.is_non_interactive() && self.prompt_service.is_interactive() {
            Some(
                self.prompt_service
                    .spinner("Discovering templates...")
                    .map_err(|e| AddServiceError::PromptFailed(e.to_string()))?,
            )
        } else {
            None
        };

        let templates_result = self.template_selector.list_service_templates();

        if let Some(spinner) = &spinner {
            if templates_result.is_ok() {
                spinner.stop("Templates discovered");
            } else {
                spinner.error("Failed to discover templates");
            }
        }

        let templates = templates_result?;

        if templates.is_empty() {
            return Err(AddServiceError::TemplateNotFound(
                "<service template>".to_owned(),
            ));
        }

        let selected_template_id = self
            .template_prompt
            .select_template(&templates)
            .map_err(AddServiceError::PromptFailed)?;

        self.template_selector
            .resolve_service_template(&selected_template_id)
    }
}

fn map_prompt_error(error: InteractiveError) -> AddServiceError {
    if error.is_cancelled() {
        return AddServiceError::Interrupted;
    }

    AddServiceError::PromptFailed(error.to_string())
}
