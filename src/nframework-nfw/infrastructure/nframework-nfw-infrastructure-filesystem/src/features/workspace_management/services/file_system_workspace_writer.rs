use std::fs;
use std::path::{Path, PathBuf};

use nframework_nfw_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use nframework_nfw_application::features::workspace_management::services::abstraction::workspace_writer::WorkspaceWriter;
use nframework_nfw_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;

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

    fn copy_template_content(
        &self,
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

        self.copy_directory_recursive(&content_root, &content_root, output_root, resolution)
    }

    fn copy_directory_recursive(
        &self,
        content_root: &Path,
        current_path: &Path,
        output_root: &Path,
        resolution: &NewCommandResolution,
    ) -> Result<(), String> {
        for entry in fs::read_dir(current_path).map_err(|error| {
            format!("failed to read template content '{}': {error}", current_path.display())
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
                self.copy_directory_recursive(content_root, &source_path, output_root, resolution)?;
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
                format!("failed to read template file '{}': {error}", source_path.display())
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
}

impl WorkspaceWriter for FileSystemWorkspaceWriter {
    fn write_workspace(
        &self,
        blueprint: &WorkspaceBlueprint,
        resolution: &NewCommandResolution,
    ) -> Result<(), String> {
        Self::assert_target_is_empty_or_missing(&resolution.output_path)?;

        fs::create_dir_all(&resolution.output_path).map_err(|error| {
            format!(
                "failed to create workspace directory '{}': {error}",
                resolution.output_path.display()
            )
        })?;

        for root_directory in &blueprint.root_directories {
            fs::create_dir_all(resolution.output_path.join(root_directory)).map_err(|error| {
                format!("failed to create directory '{root_directory}': {error}")
            })?;
        }

        self.copy_template_content(
            &resolution.template_cache_path,
            &resolution.output_path,
            resolution,
        )
    }
}

fn render_path(relative_path: &Path, resolution: &NewCommandResolution) -> PathBuf {
    let mut rendered_path = PathBuf::new();

    for component in relative_path.components() {
        let rendered_component = render_text(component.as_os_str().to_string_lossy().as_ref(), resolution);
        rendered_path.push(rendered_component);
    }

    rendered_path
}

fn render_bytes(bytes: &[u8], resolution: &NewCommandResolution) -> Vec<u8> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(renderable_text) => render_text(&renderable_text, resolution).into_bytes(),
        Err(_) => {
            // Binary file or non-UTF-8 encoding detected - return bytes unmodified.
            // This is expected for images, compiled binaries, etc.
            bytes.to_vec()
        }
    }
}

fn render_text(text: &str, resolution: &NewCommandResolution) -> String {
    let project_guid = stable_project_guid(&resolution.workspace_name, &resolution.template_id);
    text.replace("__WorkspaceName__", &resolution.workspace_name)
        .replace("__ServiceName__", &resolution.workspace_name)
        .replace("__Namespace__", &resolution.namespace_base)
        .replace("__ProjectGuid__", &project_guid)
}

pub fn stable_project_guid(workspace_name: &str, template_id: &str) -> String {
    let mut state_a: u64 = 0xcbf29ce484222325;
    let mut state_b: u64 = 0x8422_2325_cbf2_9ce4;
    for byte in workspace_name.bytes().chain(template_id.bytes()) {
        state_a ^= byte as u64;
        state_a = state_a.wrapping_mul(0x100000001b3);

        state_b ^= (byte as u64) << 1;
        state_b = state_b.wrapping_mul(0x100000001b3);
    }

    let part1 = (state_a >> 32) as u32;
    let part2 = ((state_a >> 16) & 0xffff) as u16;
    let part3 = (state_a & 0xffff) as u16;
    let part4 = ((state_b >> 48) & 0xffff) as u16;
    let part5 = state_b & 0xffff_ffff_ffff;

    format!("{part1:08x}-{part2:04x}-{part3:04x}-{part4:04x}-{part5:012x}")
}
