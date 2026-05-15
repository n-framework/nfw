mod generator_copy;
mod metadata_support;
mod render_support;

use n_framework_nfw_core_application::features::generator_management::constants::generator;

use n_framework_nfw_core_application::features::generator_management::services::generator_engine::GeneratorEngine;
use n_framework_nfw_core_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::workspace_writer::WorkspaceWriter;
use n_framework_nfw_core_domain::features::generator_management::generator_config::GeneratorConfig;
use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;
use n_framework_nfw_core_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;
use std::fs;
use std::path::Path;

use crate::features::workspace_management::services::file_system_workspace_writer::metadata_support::{
    ensure_workspace_metadata_banner_comments, ensure_workspace_metadata_file,
    normalize_workspace_metadata_file,
};
use crate::features::workspace_management::services::file_system_workspace_writer::render_support::stable_project_guid;
use crate::features::workspace_management::services::file_system_workspace_writer::generator_copy::copy_generator_content;

#[derive(Debug, Clone)]
pub struct FileSystemWorkspaceWriter<E: GeneratorEngine> {
    engine: E,
}

impl<E: GeneratorEngine> FileSystemWorkspaceWriter<E> {
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

impl<E: GeneratorEngine> WorkspaceWriter for FileSystemWorkspaceWriter<E> {
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

        // Check for tiered generator structure
        let tiered_root = resolution.generator_cache_path.join("new");
        let generator_config_path = tiered_root.join(generator::WORKFLOW_FILE);

        if generator_config_path.is_file() {
            let config_content = fs::read_to_string(&generator_config_path).map_err(|e| {
                format!(
                    "failed to read tiered generator config at {}: {e}",
                    generator_config_path.display()
                )
            })?;
            let config: GeneratorConfig = serde_yaml::from_str(&config_content)
                .map_err(|e| format!("failed to parse tiered generator config: {e}"))?;

            let parameters = GeneratorParameters::new()
                .with_name(&resolution.workspace_name)
                .map_err(|e| e.to_string())?
                .with_namespace(&resolution.namespace_base)
                .map_err(|e| e.to_string())?;

            let mut parameters = parameters;
            let _ = parameters.insert("WorkspaceName", &resolution.workspace_name);

            // Note: ProjectGuid is typically used in C# generators, providing it for compatibility
            let project_guid =
                stable_project_guid(&resolution.workspace_name, &resolution.generator_id);
            let _ = parameters.insert("ProjectGuid", project_guid);

            self.engine
                .execute(&config, &tiered_root, &resolution.output_path, &parameters)
                .map_err(|e| e.to_string())?;
        } else {
            copy_generator_content(
                &resolution.generator_cache_path,
                &resolution.output_path,
                resolution,
            )?;
        }

        ensure_workspace_metadata_file(&resolution.output_path, resolution)?;
        normalize_workspace_metadata_file(&resolution.output_path)?;
        ensure_workspace_metadata_banner_comments(&resolution.output_path)
    }
}
