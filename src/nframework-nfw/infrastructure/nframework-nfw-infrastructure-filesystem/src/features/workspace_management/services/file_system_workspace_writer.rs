mod metadata_support;
mod render_support;
mod template_copy;

use nframework_nfw_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use nframework_nfw_application::features::workspace_management::services::abstractions::workspace_writer::WorkspaceWriter;
use nframework_nfw_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;
use std::fs;
use std::path::Path;

use crate::features::workspace_management::services::file_system_workspace_writer::metadata_support::{
    ensure_workspace_metadata_banner_comments, ensure_workspace_metadata_file,
    normalize_workspace_metadata_file,
};
use crate::features::workspace_management::services::file_system_workspace_writer::template_copy::copy_template_content;

#[derive(Debug, Clone)]
pub struct FileSystemWorkspaceWriter;

impl Default for FileSystemWorkspaceWriter {
    fn default() -> Self {
        Self::new()
    }
}

impl FileSystemWorkspaceWriter {
    pub fn new() -> Self {
        Self
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

impl WorkspaceWriter for FileSystemWorkspaceWriter {
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

        copy_template_content(
            &resolution.template_cache_path,
            &resolution.output_path,
            resolution,
        )?;

        ensure_workspace_metadata_file(&resolution.output_path, resolution)?;
        normalize_workspace_metadata_file(&resolution.output_path)?;
        ensure_workspace_metadata_banner_comments(&resolution.output_path)
    }
}
