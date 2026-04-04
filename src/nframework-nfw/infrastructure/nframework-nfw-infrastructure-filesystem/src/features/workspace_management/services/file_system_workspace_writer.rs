use std::fs;
use std::path::{Path, PathBuf};

use nframework_nfw_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use nframework_nfw_application::features::workspace_management::services::abstraction::workspace_writer::WorkspaceWriter;
use nframework_nfw_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;
use serde_yaml::{Mapping, Value};

const NFW_SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json";
const NFW_SCHEMA_DIRECTIVE_COMMENT: &str = "# yaml-language-server: $schema=https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json";
const NFW_YAML_BANNER_COMMENTS: &str = "\
#    _  ______                                   __
#   / |/ / __/______ ___ _  ___ _    _____  ____/ /__
#  /    / _// __/ _ `/  ' \\/ -_) |/|/ / _ \\/ __/  '_/
# /_/|_/_/ /_/  \\_,_/_/_/_/\\__/|__,__/\\___/_/ /_/\\_\\
";

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

    fn ensure_workspace_metadata_file(
        &self,
        output_root: &Path,
        resolution: &NewCommandResolution,
    ) -> Result<(), String> {
        let workspace_metadata_path = output_root.join("nfw.yaml");
        if workspace_metadata_path.is_file() {
            return Ok(());
        }

        if workspace_metadata_path.exists() {
            return Err(format!(
                "workspace metadata path '{}' exists but is not a file",
                workspace_metadata_path.display()
            ));
        }
        let content = format!(
            "$schema: {NFW_SCHEMA_URL}\nworkspace:\n  name: {}\n  template: {}\n  namespace: {}\n",
            resolution.workspace_name, resolution.template_id, resolution.namespace_base,
        );

        fs::write(&workspace_metadata_path, content).map_err(|error| {
            format!(
                "failed to write workspace metadata file '{}': {error}",
                workspace_metadata_path.display()
            )
        })
    }

    fn normalize_workspace_metadata_file(&self, output_root: &Path) -> Result<(), String> {
        let workspace_metadata_path = output_root.join("nfw.yaml");
        let content = fs::read_to_string(&workspace_metadata_path).map_err(|error| {
            format!(
                "failed to read workspace metadata file '{}': {error}",
                workspace_metadata_path.display()
            )
        })?;
        let preserved_comments = extract_preserved_comment_block(&content);

        let mut root = serde_yaml::from_str::<Value>(&content).map_err(|error| {
            format!(
                "failed to parse workspace metadata file '{}': {error}",
                workspace_metadata_path.display()
            )
        })?;
        let root_mapping = root
            .as_mapping_mut()
            .ok_or_else(|| "workspace metadata root must be a YAML mapping".to_owned())?;

        ensure_schema_key(root_mapping);
        remove_workspace_project_guid(root_mapping)?;

        let serialized = serde_yaml::to_string(&root).map_err(|error| {
            format!(
                "failed to serialize workspace metadata file '{}': {error}",
                workspace_metadata_path.display()
            )
        })?;

        let formatted_document = format_nfw_yaml_document(&serialized, &preserved_comments)?;

        fs::write(&workspace_metadata_path, formatted_document).map_err(|error| {
            format!(
                "failed to write workspace metadata file '{}': {error}",
                workspace_metadata_path.display()
            )
        })
    }

    fn ensure_workspace_metadata_banner_comments(&self, output_root: &Path) -> Result<(), String> {
        let workspace_metadata_path = output_root.join("nfw.yaml");
        let content = fs::read_to_string(&workspace_metadata_path).map_err(|error| {
            format!(
                "failed to read workspace metadata file '{}': {error}",
                workspace_metadata_path.display()
            )
        })?;

        let (_, yaml_body) = split_leading_comments_and_body(&content);
        let preserved_comments = extract_preserved_comment_block(&content);
        let formatted_document = format_nfw_yaml_document(yaml_body, &preserved_comments)?;

        fs::write(&workspace_metadata_path, formatted_document).map_err(|error| {
            format!(
                "failed to write workspace metadata file '{}': {error}",
                workspace_metadata_path.display()
            )
        })
    }

    fn copy_directory_recursive(
        &self,
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

        self.copy_template_content(
            &resolution.template_cache_path,
            &resolution.output_path,
            resolution,
        )?;

        self.ensure_workspace_metadata_file(&resolution.output_path, resolution)?;
        self.normalize_workspace_metadata_file(&resolution.output_path)?;
        self.ensure_workspace_metadata_banner_comments(&resolution.output_path)
    }
}

fn render_path(relative_path: &Path, resolution: &NewCommandResolution) -> PathBuf {
    let mut rendered_path = PathBuf::new();

    for component in relative_path.components() {
        let rendered_component =
            render_text(component.as_os_str().to_string_lossy().as_ref(), resolution);
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

fn ensure_schema_key(root_mapping: &mut Mapping) {
    let schema_key = Value::String("$schema".to_owned());
    root_mapping.insert(schema_key, Value::String(NFW_SCHEMA_URL.to_owned()));
}

fn remove_workspace_project_guid(root_mapping: &mut Mapping) -> Result<(), String> {
    let workspace_key = Value::String("workspace".to_owned());
    let Some(workspace_value) = root_mapping.get_mut(&workspace_key) else {
        return Ok(());
    };

    let workspace_mapping = workspace_value
        .as_mapping_mut()
        .ok_or_else(|| "'workspace' must be a YAML mapping".to_owned())?;
    workspace_mapping.remove(Value::String("projectGuid".to_owned()));
    Ok(())
}

fn split_leading_comments_and_body(content: &str) -> (&str, &str) {
    let mut body_start = 0usize;

    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_start();
        let is_comment = trimmed.starts_with('#');
        let is_empty = trimmed.trim().is_empty();

        if is_comment || is_empty {
            body_start += line.len();
            continue;
        }
        break;
    }

    (&content[..body_start], &content[body_start..])
}

fn format_nfw_yaml_document(
    yaml_body: &str,
    preserved_comment_block: &str,
) -> Result<String, String> {
    let mut root = serde_yaml::from_str::<Value>(yaml_body)
        .map_err(|error| format!("failed to parse workspace metadata YAML body: {error}"))?;
    let root_mapping = root
        .as_mapping_mut()
        .ok_or_else(|| "workspace metadata root must be a YAML mapping".to_owned())?;
    reorder_root_keys(root_mapping);

    let serialized = serde_yaml::to_string(&root)
        .map_err(|error| format!("failed to serialize workspace metadata YAML body: {error}"))?;
    let formatted_body = add_top_level_section_spacing(&serialized);

    if preserved_comment_block.is_empty() {
        return Ok(format!(
            "{NFW_YAML_BANNER_COMMENTS}\n{NFW_SCHEMA_DIRECTIVE_COMMENT}\n{formatted_body}"
        ));
    }

    Ok(format!(
        "{NFW_YAML_BANNER_COMMENTS}\n{preserved_comment_block}\n{NFW_SCHEMA_DIRECTIVE_COMMENT}\n{formatted_body}"
    ))
}

fn reorder_root_keys(root_mapping: &mut Mapping) {
    let mut reordered = Mapping::new();
    move_key_if_exists(root_mapping, &mut reordered, "$schema");
    move_key_if_exists(root_mapping, &mut reordered, "workspace");
    move_key_if_exists(root_mapping, &mut reordered, "services");

    let remaining = std::mem::take(root_mapping);
    for (key, value) in remaining {
        reordered.insert(key, value);
    }

    *root_mapping = reordered;
}

fn move_key_if_exists(source: &mut Mapping, destination: &mut Mapping, key: &str) {
    let key_value = Value::String(key.to_owned());
    if let Some(value) = source.remove(&key_value) {
        destination.insert(key_value, value);
    }
}

fn add_top_level_section_spacing(content: &str) -> String {
    let mut formatted = String::new();
    let mut previous_was_empty = false;

    for line in content.lines() {
        let requires_leading_empty_line = line == "workspace:" || line == "services:";
        if requires_leading_empty_line && !formatted.is_empty() && !previous_was_empty {
            formatted.push('\n');
        }

        formatted.push_str(line);
        formatted.push('\n');
        previous_was_empty = line.trim().is_empty();
    }

    formatted
}

fn extract_preserved_comment_block(content: &str) -> String {
    let banner_lines = NFW_YAML_BANNER_COMMENTS
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let preserved_lines = content
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('#'))
        .filter(|line| !banner_lines.contains(line))
        .filter(|line| *line != NFW_SCHEMA_DIRECTIVE_COMMENT)
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    preserved_lines.join("\n")
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
