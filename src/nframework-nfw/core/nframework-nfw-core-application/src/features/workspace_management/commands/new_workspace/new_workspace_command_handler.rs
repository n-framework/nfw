use nframework_core_cli_abstractions::PromptService;

use crate::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::workspace_management::commands::new_workspace::new_workspace_command::{
    NewWorkspaceCommand, NewWorkspaceCommandResult,
};
use crate::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use crate::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use crate::features::workspace_management::services::abstractions::workspace_name_validator::WorkspaceNameValidator;
use crate::features::workspace_management::services::abstractions::workspace_writer::WorkspaceWriter;
use crate::features::workspace_management::services::input_resolution_service::InputResolutionService;
use crate::features::workspace_management::services::namespace_resolver::NamespaceResolver;
use crate::features::workspace_management::services::new_command_validator::NewCommandValidator;
use crate::features::workspace_management::services::template_selection_for_new_service::TemplateSelectionForNewService;
use crate::features::workspace_management::services::workspace_blueprint_builder::WorkspaceBlueprintBuilder;

/// Command handler for creating a new workspace.
///
/// This handler orchestrates the workspace creation process:
/// 1. Validates the command input
/// 2. Resolves workspace name (from input or prompt)
/// 3. Resolves template selection (from input or prompt)
/// 4. Resolves namespace and output path
/// 5. Builds and writes the workspace blueprint
#[derive(Debug, Clone)]
pub struct NewWorkspaceCommandHandler<P, V, T, W, D, PS>
where
    P: PromptService,
    V: WorkspaceNameValidator + Clone,
    T: TemplateCatalogDiscoveryService + Clone,
    W: WorkspaceWriter,
    D: WorkingDirectoryProvider,
    PS: PromptService + Clone,
{
    input_resolution_service: InputResolutionService<P, V>,
    template_selection_service: TemplateSelectionForNewService<T, PS>,
    workspace_blueprint_builder: WorkspaceBlueprintBuilder,
    namespace_resolver: NamespaceResolver,
    new_command_validator: NewCommandValidator<V>,
    workspace_writer: W,
    working_directory_provider: D,
}

impl<P, V, T, W, D, PS> NewWorkspaceCommandHandler<P, V, T, W, D, PS>
where
    P: PromptService,
    V: WorkspaceNameValidator + Clone,
    T: TemplateCatalogDiscoveryService + Clone,
    W: WorkspaceWriter,
    D: WorkingDirectoryProvider,
    PS: PromptService + Clone,
{
    pub fn new(
        prompt_service: P,
        workspace_name_validator: V,
        template_selection_service: TemplateSelectionForNewService<T, PS>,
        workspace_writer: W,
        working_directory_provider: D,
    ) -> Self {
        let input_resolution_service =
            InputResolutionService::new(prompt_service, workspace_name_validator.clone());
        let new_command_validator = NewCommandValidator::new(workspace_name_validator);

        Self {
            input_resolution_service,
            template_selection_service,
            workspace_blueprint_builder: WorkspaceBlueprintBuilder::new(),
            namespace_resolver: NamespaceResolver::new(),
            new_command_validator,
            workspace_writer,
            working_directory_provider,
        }
    }

    pub fn handle(
        &self,
        command: &NewWorkspaceCommand,
    ) -> Result<NewWorkspaceCommandResult, WorkspaceNewError> {
        let request = command.to_request();
        self.new_command_validator.validate_request(&request)?;

        let workspace_name = self
            .input_resolution_service
            .resolve_workspace_name(&request)?;
        let template_selection = self.template_selection_service.resolve_template_selection(
            command.template_id.as_deref(),
            !command.no_input && command.is_interactive_terminal,
        )?;

        let namespace_base = self
            .namespace_resolver
            .resolve_workspace_base_namespace(&workspace_name);
        let output_path = self
            .working_directory_provider
            .current_dir()
            .map_err(|error| WorkspaceNewError::Internal(error.to_string()))?
            .join(&workspace_name);

        let resolution = NewCommandResolution {
            workspace_name: workspace_name.clone(),
            template_id: format!(
                "{}/{}",
                template_selection.source_name, template_selection.template.metadata.id
            ),
            template_cache_path: template_selection.template.cache_path.clone(),
            namespace_base: namespace_base.clone(),
            output_path: output_path.clone(),
        };

        let blueprint = self.workspace_blueprint_builder.build(&workspace_name);
        self.workspace_writer
            .write_workspace(&blueprint, &resolution)
            .map_err(WorkspaceNewError::WriteFailed)?;

        Ok(NewWorkspaceCommandResult {
            workspace_name,
            output_path,
            template_id: resolution.template_id,
            namespace_base,
        })
    }
}
