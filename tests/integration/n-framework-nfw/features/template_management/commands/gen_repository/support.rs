use std::path::Path;

use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::file_system_template_root_resolver::FileSystemTemplateRootResolver;
use n_framework_nfw_infrastructure_filesystem::features::template_management::template_engine::FileSystemTemplateEngine;

use n_framework_nfw_core_application::features::template_management::commands::gen_repository::gen_repository_command::GenRepositoryCommand;
use n_framework_nfw_core_application::features::template_management::commands::gen_repository::gen_repository_command_handler::GenRepositoryCommandHandler;

pub fn execute_non_interactive_gen_repository(
    _workspace_root: &Path,
    entity_name: &str,
    feature: &str,
) -> Result<(), n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError>{
    let working_dir_provider = StandardWorkingDirectoryProvider::new();
    let template_root_resolver = FileSystemTemplateRootResolver::new();
    let engine = FileSystemTemplateEngine::new();

    let handler =
        GenRepositoryCommandHandler::new(working_dir_provider, template_root_resolver, engine);

    let workspace_context = handler.get_workspace_context()?;
    let services = handler.extract_services(&workspace_context)?;

    if services.is_empty() {
        return Err(n_framework_nfw_core_application::features::template_management::models::errors::add_artifact_error::AddArtifactError::ConfigError("No services found".to_string()));
    }

    let service = services[0].clone();

    let template_context =
        handler.load_template_context(workspace_context, &service, "repository")?;

    let feature_opt = if feature.is_empty() {
        None
    } else {
        Some(feature.to_string())
    };

    let command = GenRepositoryCommand::new(entity_name.to_string(), feature_opt, template_context);

    handler.handle(&command)
}
