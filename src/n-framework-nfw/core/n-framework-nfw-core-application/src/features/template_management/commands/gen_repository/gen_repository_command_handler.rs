use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::artifact_generation_service::{
    AddArtifactContext, ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use regex::Regex;

use super::gen_repository_command::GenRepositoryCommand;

/// Handler for the `GenRepositoryCommand`.
///
/// This handler manages the orchestration of repository generation, including
/// multi-step feature discovery and artifact construction.
#[derive(Debug, Clone)]
pub struct GenRepositoryCommandHandler<W, R, E> {
    service: ArtifactGenerationService<W, R, E>,
}

impl<W, R, E> GenRepositoryCommandHandler<W, R, E>
where
    W: WorkingDirectoryProvider,
    R: TemplateRootResolver,
    E: TemplateEngine,
{
    /// Creates a new `GenRepositoryCommandHandler`.
    pub fn new(working_dir_provider: W, root_resolver: R, engine: E) -> Self {
        Self {
            service: ArtifactGenerationService::new(working_dir_provider, root_resolver, engine),
        }
    }

    /// Handles the `GenRepositoryCommand` to scaffold a repository.
    pub fn handle(&self, command: &GenRepositoryCommand) -> Result<(), AddArtifactError> {
        let entity_name = command.entity_name();

        // Validate entity name
        let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
        if !re.is_match(entity_name) {
            return Err(AddArtifactError::InvalidIdentifier(format!(
                "Invalid entity name '{}'. Entity names must start with a letter and contain only alphanumeric characters.",
                entity_name
            )));
        }

        let context = command.context();
        let service_name = &context.service_name;
        let namespace = self.service.extract_namespace(&context.nfw_yaml)?;
        let service_root = context.workspace_root.join(&context.service_path);

        let possible_roots = vec![
            service_root
                .join("src")
                .join("core")
                .join(format!("{}.Core.Domain", namespace))
                .join("Features"),
            service_root.join("src").join("Domain").join("Features"),
            service_root.join("src").join("Features"),
            service_root.join("Features"),
        ];

        let mut features_dir_opt = None;
        for root in possible_roots {
            if root.is_dir() {
                features_dir_opt = Some(root);
                break;
            }
        }

        let features_dir = match features_dir_opt {
            Some(dir) => dir,
            None => {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "Could not find Features directory in domain layer for service '{}'",
                    service_name
                )));
            }
        };

        // Auto-detect feature if not explicitly provided
        let target_feature = if let Some(f) = command.feature() {
            f.to_string()
        } else {
            let features_containing_entity =
                self.auto_detect_features(&features_dir, entity_name)?;

            if features_containing_entity.is_empty() {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "Entity '{}' not found in any feature in {:?}",
                    entity_name, features_dir
                )));
            } else if features_containing_entity.len() > 1 {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "Entity '{}' found in multiple features ({:?}). Please specify --feature.",
                    entity_name, features_containing_entity
                )));
            }

            features_containing_entity[0].clone()
        };

        // 1. Verify Entity Exists
        let entities_dir = features_dir.join(&target_feature).join("Entities");

        if !entities_dir.exists() {
            return Err(AddArtifactError::InvalidIdentifier(format!(
                "Feature '{}' does not contain an Entities folder at {:?}",
                target_feature, entities_dir
            )));
        }

        let mut entity_found = false;
        let entries = std::fs::read_dir(&entities_dir).map_err(|e| {
            AddArtifactError::FileReadError(format!(
                "Failed to read entities directory {:?}: {}",
                entities_dir, e
            ))
        })?;

        for entry_result in entries {
            let entry = entry_result.map_err(|e| {
                AddArtifactError::FileReadError(format!(
                    "Error reading directory entry in {:?}: {}",
                    entities_dir, e
                ))
            })?;

            if let Some(file_name) = entry.file_name().to_str() {
                if file_name.starts_with(entity_name) && file_name.ends_with(".cs") {
                    entity_found = true;
                    break;
                }
            } else {
                tracing::warn!(
                    "Failed to convert file name to string in {:?}",
                    entities_dir
                );
            }
        }

        if !entity_found {
            return Err(AddArtifactError::InvalidIdentifier(format!(
                "Entity '{}' not found in feature '{}' at {:?}",
                entity_name, target_feature, entities_dir
            )));
        }

        // Check if repository already exists
        let possible_repo_roots = vec![
            service_root
                .join("src")
                .join("infrastructure")
                .join(format!("{}.Infrastructure.Persistence", namespace))
                .join("Features")
                .join(&target_feature)
                .join("Repositories"),
            service_root
                .join("src")
                .join("Persistence")
                .join("Features")
                .join(&target_feature)
                .join("Repositories"),
            service_root
                .join("src")
                .join("Features")
                .join(&target_feature)
                .join("Persistence")
                .join("Repositories"),
            features_dir
                .join(&target_feature)
                .join("Persistence")
                .join("Repositories"),
        ];

        let mut repo_exists = false;
        let mut existing_repo_path = None;
        for root in possible_repo_roots {
            let repo_file = root.join(format!("{}Repository.cs", entity_name));
            if repo_file.exists() {
                repo_exists = true;
                existing_repo_path = Some(repo_file);
                break;
            }

            let i_repo_file = root.join(format!("I{}Repository.cs", entity_name));
            if i_repo_file.exists() {
                repo_exists = true;
                existing_repo_path = Some(i_repo_file);
                break;
            }
        }

        if repo_exists {
            return Err(AddArtifactError::ArtifactAlreadyExists(format!(
                "Repository for entity '{}' already exists at {:?}",
                entity_name,
                existing_repo_path.unwrap()
            )));
        }

        // 2. Validate Persistence is Configured
        let nfw_yaml = &context.nfw_yaml;

        let mut has_persistence = false;

        if let Some(services) = nfw_yaml.get("services")
            && let Some(service_info) = services.get(service_name)
            && let Some(modules) = service_info.get("modules")
            && let Some(modules_seq) = modules.as_sequence()
        {
            has_persistence = modules_seq
                .iter()
                .any(|m| m.as_str() == Some("persistence"));
        }

        if !has_persistence {
            return Err(AddArtifactError::MissingRequiredModule(format!(
                "Service '{}' does not have 'persistence' module configured. Run 'nfw add persistence' first.",
                service_name
            )));
        }

        // 3. Add Entity to Params
        let mut params_map = serde_json::Map::new();
        params_map.insert(
            "Entity".to_string(),
            serde_json::Value::String(entity_name.to_string()),
        );

        let params = Some(serde_json::Value::Object(params_map));

        // 4. Execute generation
        self.service
            .execute_generation(entity_name, Some(&target_feature), &params, context)
    }

    /// Auto-detects the feature containing the given entity.
    ///
    /// This method performs a recursive filesystem search within the features directory
    /// to locate the `Entities` folder and find the `[Entity].cs` file.
    ///
    /// The algorithm follows these steps:
    /// 1. Iterates through all subdirectories in the `features_dir`.
    /// 2. For each directory, it looks for an `Entities` child folder.
    /// 3. If found, it reads all files in that `Entities` folder.
    /// 4. Matches any file that starts with the `entity_name` and has a `.cs` extension.
    /// 5. Collects all matching feature names to return.
    fn auto_detect_features(
        &self,
        features_dir: &std::path::Path,
        entity_name: &str,
    ) -> Result<Vec<String>, AddArtifactError> {
        let mut features_containing_entity = vec![];
        if !features_dir.exists() {
            tracing::warn!(
                "Features directory does not exist: {}",
                features_dir.display()
            );
            return Ok(features_containing_entity);
        }

        let entries = std::fs::read_dir(features_dir).map_err(|e| {
            AddArtifactError::FileReadError(format!(
                "Failed to read features directory {}: {}",
                features_dir.display(),
                e
            ))
        })?;

        for entry_result in entries {
            let entry = entry_result.map_err(|e| {
                AddArtifactError::FileReadError(format!(
                    "Failed to read features directory entry in {}: {}",
                    features_dir.display(),
                    e
                ))
            })?;

            if !entry.path().is_dir() {
                continue;
            }

            if let Some(feature_name) = entry.file_name().to_str() {
                let entities_dir = entry.path().join("Entities");
                if !entities_dir.exists() {
                    continue;
                }

                let entity_entries = std::fs::read_dir(&entities_dir).map_err(|e| {
                    AddArtifactError::FileReadError(format!(
                        "Failed to read entities directory {}: {}",
                        entities_dir.display(),
                        e
                    ))
                })?;

                for entity_entry_result in entity_entries {
                    let entity_entry = entity_entry_result.map_err(|e| {
                        AddArtifactError::FileReadError(format!(
                            "Failed to read entities entry in {}: {}",
                            entities_dir.display(),
                            e
                        ))
                    })?;

                    if let Some(file_name) = entity_entry.file_name().to_str()
                        && file_name.starts_with(entity_name)
                        && file_name.ends_with(".cs")
                    {
                        features_containing_entity.push(feature_name.to_string());
                    }
                }
            }
        }

        Ok(features_containing_entity)
    }

    pub fn get_workspace_context(&self) -> Result<WorkspaceContext, AddArtifactError> {
        self.service.get_workspace_context()
    }

    pub fn extract_services(
        &self,
        workspace: &WorkspaceContext,
    ) -> Result<Vec<ServiceInfo>, AddArtifactError> {
        self.service.extract_services(workspace)
    }

    pub fn list_features(
        &self,
        workspace: &WorkspaceContext,
        service: &ServiceInfo,
    ) -> Result<Vec<String>, AddArtifactError> {
        self.service.list_features(workspace, service)
    }

    pub fn load_template_context(
        &self,
        workspace: WorkspaceContext,
        service: &ServiceInfo,
        generator_type: &str,
    ) -> Result<AddArtifactContext, AddArtifactError> {
        self.service
            .load_template_context(workspace, service, generator_type)
    }
}

#[cfg(test)]
#[path = "gen_repository_command_handler.tests.rs"]
mod gen_repository_command_handler_tests;
