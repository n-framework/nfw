use crate::features::workspace_management::services::file_system_workspace_writer::render_support::{
    render_bytes, render_path,
};
use nframework_nfw_core_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use std::fs;
use std::path::Path;

pub fn copy_template_content(
    template_root: &Path,
    output_root: &Path,
    resolution: &NewCommandResolution,
) -> Result<(), String> {
    let content_root = template_root.join("content");
    if !content_root.is_dir() {
        return Err(format!(
            "template directory '{}' is missing required 'content/' directory",
            template_root.display()
        ));
    }

    copy_directory_recursive(&content_root, &content_root, output_root, resolution)
}

fn copy_directory_recursive(
    content_root: &Path,
    current_path: &Path,
    output_root: &Path,
    resolution: &NewCommandResolution,
) -> Result<(), String> {
    for entry in fs::read_dir(current_path).map_err(|error| {
        format!(
            "failed to read template content '{}': {error}",
            current_path.display()
        )
    })? {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read a template content entry in '{}': {error}",
                current_path.display()
            )
        })?;

        let source_path = entry.path();
        let relative_path = source_path.strip_prefix(content_root).map_err(|error| {
            format!(
                "failed to compute template-relative path for '{}': {error}",
                source_path.display()
            )
        })?;
        let rendered_relative_path = render_path(relative_path, resolution);
        let destination_path = output_root.join(rendered_relative_path);

        if source_path.is_dir() {
            fs::create_dir_all(&destination_path).map_err(|error| {
                format!(
                    "failed to create workspace directory '{}': {error}",
                    destination_path.display()
                )
            })?;
            copy_directory_recursive(content_root, &source_path, output_root, resolution)?;
            continue;
        }

        if let Some(parent) = destination_path.parent() {
            fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create workspace directory '{}': {error}",
                    parent.display()
                )
            })?;
        }

        let bytes = fs::read(&source_path).map_err(|error| {
            format!(
                "failed to read template file '{}': {error}",
                source_path.display()
            )
        })?;
        let rendered_bytes = render_bytes(&bytes, resolution);
        fs::write(&destination_path, rendered_bytes).map_err(|error| {
            format!(
                "failed to write workspace file '{}': {error}",
                destination_path.display()
            )
        })?;
    }

    Ok(())
}
