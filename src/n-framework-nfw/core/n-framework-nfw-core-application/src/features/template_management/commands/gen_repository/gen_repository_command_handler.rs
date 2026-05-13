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

        let re = Regex::new(r"^[a-zA-Z][a-zA-Z0-9]*$").unwrap();
        if !re.is_match(entity_name) {
            return Err(AddArtifactError::InvalidIdentifier(format!(
                "Invalid entity name '{}'. Entity names must start with a letter and contain only alphanumeric characters.",
                entity_name
            )));
        }

        let context = command.context();
        let service_name = &context.service_name;

        // Build the WorkspaceContext so we can use template-driven path resolution.
        let workspace = WorkspaceContext {
            workspace_root: context.workspace_root.clone(),
            nfw_yaml: context.nfw_yaml.clone(),
            preserved_comments: Default::default(),
        };
        let service = ServiceInfo {
            name: service_name.clone(),
            path: context.service_path.to_string_lossy().to_string(),
            template_id: context
                .nfw_yaml
                .get("services")
                .and_then(|s| s.get(service_name))
                .and_then(|d| d.get("template"))
                .and_then(|t| t.as_mapping())
                .and_then(|t| t.get("id"))
                .and_then(|id| id.as_str())
                .unwrap_or_default()
                .to_string(),
        };

        // Auto-detect or validate feature using the entity template's step destinations — no
        // hardcoded path candidates.
        let target_feature = if let Some(f) = command.feature() {
            f.to_string()
        } else {
            let matching = self.service.find_artifact_in_features(
                &workspace,
                &service,
                "entity",
                entity_name,
            )?;

            if matching.is_empty() {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "Entity '{}' not found in any feature. Generate the entity first.",
                    entity_name
                )));
            } else if matching.len() > 1 {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "Entity '{}' found in multiple features ({:?}). Please specify --feature.",
                    entity_name, matching
                )));
            }
            matching.into_iter().next().unwrap()
        };

        // Validate entity exists in the specified feature using the entity template's paths.
        let entity_subdir = self.service.resolve_artifact_subdir(
            &workspace,
            &service,
            "entity",
            &target_feature,
        )?;
        if let Some(dir) = &entity_subdir {
            if !dir.is_dir() {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "Feature '{}' does not contain an entity artifacts directory at {}",
                    target_feature,
                    dir.display()
                )));
            }
            // Check that a file starting with entity_name exists in that directory.
            let entity_exists = std::fs::read_dir(dir)
                .ok()
                .map(|entries| {
                    entries.flatten().any(|e| {
                        e.file_name()
                            .to_str()
                            .map(|n| n.starts_with(entity_name))
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false);
            if !entity_exists {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "Entity '{}' not found in feature '{}'",
                    entity_name, target_feature
                )));
            }
        }

        // Check if repository already exists by probing the path derived from the repository
        // template's step destination — no hardcoded `Repositories/` or `.cs` suffix.
        let repo_subdir = self.service.resolve_artifact_subdir(
            &workspace,
            &service,
            "repository",
            &target_feature,
        )?;
        if let Some(dir) = repo_subdir
            && dir.is_dir()
        {
            let repo_exists = std::fs::read_dir(&dir)
                .ok()
                .map(|entries| {
                    entries.flatten().any(|e| {
                        e.file_name()
                            .to_str()
                            .map(|n| {
                                n.starts_with(entity_name)
                                    || n.starts_with(&format!("I{}", entity_name))
                            })
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false);
            if repo_exists {
                return Err(AddArtifactError::ArtifactAlreadyExists(format!(
                    "Repository for entity '{}' already exists in feature '{}'",
                    entity_name, target_feature
                )));
            }
        }

        // Validate persistence module is configured.
        let has_persistence = context
            .nfw_yaml
            .get("services")
            .and_then(|s| s.get(service_name))
            .and_then(|d| d.get("modules"))
            .and_then(|m| m.as_sequence())
            .map(|seq| seq.iter().any(|m| m.as_str() == Some("persistence")))
            .unwrap_or(false);

        if !has_persistence {
            return Err(AddArtifactError::MissingRequiredModule(format!(
                "Service '{}' does not have 'persistence' module configured. Run 'nfw add persistence' first.",
                service_name
            )));
        }

        let mut params_map = serde_json::Map::new();
        params_map.insert(
            "Entity".to_string(),
            serde_json::Value::String(entity_name.to_string()),
        );
        let params = Some(serde_json::Value::Object(params_map));

        self.service
            .execute_generation(entity_name, Some(&target_feature), &params, context)
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
