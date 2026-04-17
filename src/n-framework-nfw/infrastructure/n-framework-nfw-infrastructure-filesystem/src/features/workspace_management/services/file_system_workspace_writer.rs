mod metadata_support;
mod render_support;
mod template_copy;

use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::workspace_writer::WorkspaceWriter;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::features::workspace_management::services::file_system_workspace_writer::metadata_support::{
    ensure_workspace_metadata_banner_comments, ensure_workspace_metadata_file,
    normalize_workspace_metadata_file,
};
use crate::features::workspace_management::services::file_system_workspace_writer::template_copy::copy_template_content;

#[derive(Debug, Clone)]
pub struct FileSystemWorkspaceWriter<E: TemplateEngine> {
    engine: E,
}

impl<E: TemplateEngine> FileSystemWorkspaceWriter<E> {
    pub fn new(engine: E) -> Self {
        Self { engine }
    }

    fn assert_target_is_empty_or_missing(path: &Path) -> Result<(), String> {
        if !path.exists() {
            return Ok(());
        }

        let mut entries = fs::read_dir(path).map_err(|error| {
            format!(
                "failed to inspect target directory '{}': {error}",
                path.display()
            )
        })?;

        if entries.next().is_some() {
            return Err(format!(
                "target directory '{}' already exists and is not empty",
                path.display()
            ));
        }

        Ok(())
    }
}

impl<E: TemplateEngine> WorkspaceWriter for FileSystemWorkspaceWriter<E> {
    fn write_workspace(
        &self,
        _blueprint: &WorkspaceBlueprint,
        resolution: &NewCommandResolution,
    ) -> Result<(), String> {
        Self::assert_target_is_empty_or_missing(&resolution.output_path)?;

        fs::create_dir_all(&resolution.output_path).map_err(|error| {
            format!(
                "failed to create workspace directory '{}': {error}",
                resolution.output_path.display()
            )
        })?;

        // Check for tiered template structure
        let tiered_root = resolution.template_cache_path.join("new");
        let template_config_path = tiered_root.join("template.yaml");

        if template_config_path.is_file() {
            let config_content = fs::read_to_string(&template_config_path).map_err(|e| {
                format!(
                    "failed to read tiered template config at {}: {e}",
                    template_config_path.display()
                )
            })?;
            let config: TemplateConfig = serde_yaml::from_str(&config_content)
                .map_err(|e| format!("failed to parse tiered template config: {e}"))?;

            let mut placeholders = BTreeMap::new();
            placeholders.insert("Name".to_string(), resolution.workspace_name.clone());
            placeholders.insert(
                "WorkspaceName".to_string(),
                resolution.workspace_name.clone(),
            );
            placeholders.insert("Namespace".to_string(), resolution.namespace_base.clone());

            // Note: ProjectGuid is typically used in C# templates, providing it for compatibility
            let project_guid = crate::features::workspace_management::services::file_system_workspace_writer::render_support::stable_project_guid(
                &resolution.workspace_name,
                &resolution.template_id
            );
            placeholders.insert("ProjectGuid".to_string(), project_guid);

            self.engine
                .execute(
                    &config,
                    &tiered_root,
                    &resolution.output_path,
                    &placeholders,
                )
                .map_err(|e| format!("{e}"))?;
        } else {
            // Fallback to legacy content copying
            copy_template_content(
                &resolution.template_cache_path,
                &resolution.output_path,
                resolution,
            )?;
        }

        ensure_workspace_metadata_file(&resolution.output_path, resolution)?;
        normalize_workspace_metadata_file(&resolution.output_path)?;
        ensure_workspace_metadata_banner_comments(&resolution.output_path)
    }
}
