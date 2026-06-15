use std::path::Path;

use n_framework_nfw_core_application::features::entity_generation::abstractions::entity_schema_store::EntitySchemaStore;
use n_framework_nfw_core_application::features::generator_management::commands::gen_crud::gen_crud_command::GenCrudCommand;
use n_framework_nfw_core_application::features::generator_management::commands::gen_crud::gen_crud_command_handler::GenCrudCommandHandler;
use n_framework_nfw_core_application::features::generator_management::models::errors::add_artifact_error::AddArtifactError;
use n_framework_nfw_infrastructure_filesystem::features::entity_generation::adapters::file_system_entity_schema_store::FileSystemEntitySchemaStore;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::generator_engine::FileSystemGeneratorEngine;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::file_system_generator_root_resolver::FileSystemGeneratorRootResolver;
use n_framework_nfw_infrastructure_filesystem::features::workspace_management::services::standard_working_directory_provider::StandardWorkingDirectoryProvider;

pub fn execute_non_interactive_gen_crud(
    _workspace_root: &Path,
    entity_name: &str,
    feature: &str,
    params: Option<serde_json::Value>,
) -> Result<(), AddArtifactError> {
    let handler = GenCrudCommandHandler::new(
        StandardWorkingDirectoryProvider::new(),
        FileSystemGeneratorRootResolver::new(),
        FileSystemGeneratorEngine::new(),
        FileSystemEntitySchemaStore::new(),
    );

    let workspace_context = handler.get_workspace_context()?;
    let services = handler.extract_services(&workspace_context)?;

    if services.is_empty() {
        return Err(AddArtifactError::ConfigError(
            "No services found".to_string(),
        ));
    }

    let service = services[0].clone();
    let context = handler.load_generator_context(workspace_context, &service, "crud")?;

    let feature_opt = if feature.is_empty() {
        None
    } else {
        Some(feature.to_string())
    };

    let command = GenCrudCommand::new(entity_name, feature_opt, params, context);
    handler.handle(&command)
}
