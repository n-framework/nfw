use std::path::Path;

use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::file_system_template_root_resolver::FileSystemTemplateRootResolver;
use n_framework_nfw_infrastructure_filesystem::features::template_management::template_engine::FileSystemTemplateEngine;

use n_framework_nfw_core_application::features::template_management::commands::gen_endpoint::gen_endpoint_command::GenEndpointCommand;
use n_framework_nfw_core_application::features::template_management::commands::gen_endpoint::gen_endpoint_command_handler::GenEndpointCommandHandler;

pub fn execute_non_interactive_gen_endpoint(
    _workspace_root: &Path,
    name: &str,
    feature: &str,
    operation_type: &str,
) -> Result<(), n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError>{
    let working_dir_provider = StandardWorkingDirectoryProvider::new();
    let template_root_resolver = FileSystemTemplateRootResolver::new();
    let engine = FileSystemTemplateEngine::new();

    let handler =
        GenEndpointCommandHandler::new(working_dir_provider, template_root_resolver, engine);

    let workspace_context = handler.get_workspace_context()?;
    let services = handler.extract_services(&workspace_context)?;

    if services.is_empty() {
        return Err(n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError::ConfigError("No services found".to_string()));
    }

    let service = services[0].clone();

    let template_context =
        handler.load_template_context("endpoint", &service, &workspace_context)?;

    let feature_opt = if feature.is_empty() {
        None
    } else {
        Some(feature.to_string())
    };

    let command = GenEndpointCommand {
        name: name.to_string(),
        feature: feature_opt,
        operation_type: operation_type.to_string(),
        context: template_context,
        params: None,
        attach_to_mediator: false,
    };

    handler.handle(command).map(|_| ())
}
