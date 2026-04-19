use crate::features::service_management::commands::add_service::add_service_command::{
    AddServiceCommand, AddServiceCommandResult,
};
use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_generation_plan::ServiceGenerationPlan;
use crate::features::service_management::services::abstractions::service_provenance_store::ServiceProvenanceStore;
use crate::features::service_management::services::abstractions::service_template_prompt::ServiceTemplatePrompt;
use crate::features::service_management::services::abstractions::service_template_renderer::ServiceTemplateRenderer;
use crate::features::service_management::services::abstractions::service_template_selector::ServiceTemplateSelector;
use crate::features::service_management::services::add_service_input_resolution_service::AddServiceInputResolutionService;
use crate::features::service_management::services::add_service_request_validator::AddServiceRequestValidator;
use crate::features::service_management::services::add_service_workspace_context_guard::AddServiceWorkspaceContextGuard;
use crate::features::service_management::services::service_generation_plan_builder::ServiceGenerationPlanBuilder;
use crate::features::service_management::services::service_template_provenance_service::ServiceTemplateProvenanceService;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_core_cli_abstractions::{InteractivePrompt, Logger};

#[derive(Debug, Clone)]
pub struct AddServiceCommandHandler<D, S, P, Q, R, PS>
where
    D: WorkingDirectoryProvider,
    S: ServiceTemplateSelector,
    P: ServiceTemplatePrompt,
    Q: InteractivePrompt + Logger,
    R: ServiceTemplateRenderer,
    PS: ServiceProvenanceStore,
{
    request_validator: AddServiceRequestValidator,
    workspace_context_guard: AddServiceWorkspaceContextGuard,
    input_resolution_service: AddServiceInputResolutionService<S, P, Q>,
    plan_builder: ServiceGenerationPlanBuilder,
    renderer: R,
    provenance_service: ServiceTemplateProvenanceService<PS>,
    working_directory_provider: D,
}

impl<D, S, P, Q, R, PS> AddServiceCommandHandler<D, S, P, Q, R, PS>
where
    D: WorkingDirectoryProvider,
    S: ServiceTemplateSelector,
    P: ServiceTemplatePrompt,
    Q: InteractivePrompt + Logger,
    R: ServiceTemplateRenderer,
    PS: ServiceProvenanceStore,
{
    pub fn new(
        working_directory_provider: D,
        input_resolution_service: AddServiceInputResolutionService<S, P, Q>,
        renderer: R,
        provenance_service: ServiceTemplateProvenanceService<PS>,
    ) -> Self {
        Self {
            request_validator: AddServiceRequestValidator::new(),
            workspace_context_guard: AddServiceWorkspaceContextGuard::new(),
            input_resolution_service,
            plan_builder: ServiceGenerationPlanBuilder::new(),
            renderer,
            provenance_service,
            working_directory_provider,
        }
    }

    pub fn handle(
        &self,
        command: &AddServiceCommand,
    ) -> Result<AddServiceCommandResult, AddServiceError> {
        let plan = self.prepare_generation_plan(command)?;
        self.execute_generation_plan(&plan)
    }

    pub fn prepare_generation_plan(
        &self,
        command: &AddServiceCommand,
    ) -> Result<ServiceGenerationPlan, AddServiceError> {
        let request = command.to_request();
        self.request_validator.validate_request(&request)?;

        let service_name = self
            .input_resolution_service
            .resolve_service_name(&request)?;
        self.request_validator
            .validate_service_name(&service_name)?;

        let current_directory = self
            .working_directory_provider
            .current_dir()
            .map_err(AddServiceError::Internal)?;
        let workspace_root = self
            .workspace_context_guard
            .ensure_workspace_root(&current_directory)?;

        let template_resolution = self
            .input_resolution_service
            .resolve_template_selection(&request)?;

        self.plan_builder
            .build(&service_name, &workspace_root, &template_resolution)
    }

    pub fn execute_generation_plan(
        &self,
        plan: &ServiceGenerationPlan,
    ) -> Result<AddServiceCommandResult, AddServiceError> {
        let current_directory = self
            .working_directory_provider
            .current_dir()
            .map_err(AddServiceError::Internal)?;
        let workspace_root = self
            .workspace_context_guard
            .ensure_workspace_root(&current_directory)?;

        if let Err(error) = self.renderer.render_service(plan) {
            return Err(self.cleanup_and_wrap(&plan.output_root, error));
        }

        if let Err(error) = self.provenance_service.persist(
            &workspace_root,
            &plan.service_name,
            &plan.template_id,
            &plan.template_version.to_string(),
        ) {
            return Err(self.cleanup_and_wrap(&plan.output_root, error));
        }

        Ok(AddServiceCommandResult {
            service_name: plan.service_name.clone(),
            output_path: plan.output_root.clone(),
            template_id: plan.template_id.clone(),
            template_version: plan.template_version.to_string(),
        })
    }

    pub fn execute(
        &self,
        command: &AddServiceCommand,
    ) -> Result<AddServiceCommandResult, AddServiceError> {
        self.handle(command)
    }

    fn cleanup_and_wrap(
        &self,
        output_root: &std::path::Path,
        error: AddServiceError,
    ) -> AddServiceError {
        match self.renderer.cleanup_partial_output(output_root) {
            Ok(()) => error,
            Err(cleanup_error) => AddServiceError::CleanupFailed(format!(
                "{}; original error: {error}",
                cleanup_error
            )),
        }
    }
}
