use crate::features::template_management::models::errors::add_artifact_error::AddArtifactError;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::artifact_generation_service::{
    AddArtifactContext, ArtifactGenerationService, ServiceInfo, WorkspaceContext,
};
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;

use super::gen_repository_command::GenRepositoryCommand;

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
    pub fn new(working_dir_provider: W, root_resolver: R, engine: E) -> Self {
        Self {
            service: ArtifactGenerationService::new(working_dir_provider, root_resolver, engine),
        }
    }

    pub fn handle(&self, command: &GenRepositoryCommand) -> Result<(), AddArtifactError> {
        let service_name = &command.context.service_name;
        let features_dir = command
            .context
            .service_path
            .join(format!("src/core/{}.Core.Domain/Features", service_name));

        // Auto-detect feature if not explicitly provided
        let target_feature = if let Some(f) = &command.feature {
            f.to_string()
        } else {
            let found_features = self.auto_detect_features(&features_dir, &command.entity_name);

            if found_features.is_empty() {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "Entity '{}' not found in any feature in {:?}",
                    command.entity_name, features_dir
                )));
            } else if found_features.len() > 1 {
                return Err(AddArtifactError::InvalidIdentifier(format!(
                    "Entity '{}' found in multiple features ({:?}). Please specify --feature.",
                    command.entity_name, found_features
                )));
            }

            found_features[0].clone()
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
        if let Ok(entries) = std::fs::read_dir(&entities_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str()
                    && file_name.starts_with(&command.entity_name)
                    && file_name.ends_with(".cs")
                {
                    entity_found = true;
                    break;
                }
            }
        }

        if !entity_found {
            return Err(AddArtifactError::InvalidIdentifier(format!(
                "Entity '{}' not found in feature '{}' at {:?}",
                command.entity_name, target_feature, entities_dir
            )));
        }

        // 2. Validate Persistence is Configured
        let nfw_yaml = &command.context.nfw_yaml;

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
            serde_json::Value::String(command.entity_name.clone()),
        );

        let params = Some(serde_json::Value::Object(params_map));

        // 4. Execute generation
        self.service.execute_generation(
            &command.entity_name,
            Some(&target_feature),
            &params,
            &command.context,
        )
    }

    fn auto_detect_features(
        &self,
        features_dir: &std::path::Path,
        entity_name: &str,
    ) -> Vec<String> {
        let mut found_features = vec![];
        if !features_dir.exists() {
            return found_features;
        }

        if let Ok(entries) = std::fs::read_dir(features_dir) {
            for entry in entries.flatten() {
                if !entry.path().is_dir() {
                    continue;
                }

                if let Some(feature_name) = entry.file_name().to_str() {
                    let entities_dir = entry.path().join("Entities");
                    if !entities_dir.exists() {
                        continue;
                    }

                    if let Ok(entity_entries) = std::fs::read_dir(&entities_dir) {
                        for entity_entry in entity_entries.flatten() {
                            if let Some(file_name) = entity_entry.file_name().to_str()
                                && file_name.starts_with(entity_name)
                                && file_name.ends_with(".cs")
                            {
                                found_features.push(feature_name.to_string());
                            }
                        }
                    }
                }
            }
        }

        found_features
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
