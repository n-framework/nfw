//! Shared workspace metadata YAML document helpers.

use serde_yaml::{Mapping, Value};

pub const NFW_SCHEMA_URL: &str =
    "https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json";
pub const NFW_SCHEMA_DIRECTIVE_COMMENT: &str = "# yaml-language-server: $schema=https://raw.githubusercontent.com/n-framework/nfw/main/schemas/nfw.schema.json";
pub const NFW_YAML_BANNER_COMMENTS: &str = "\
#    _  ______                                   __
#   / |/ / __/______ ___ _  ___ _    _____  ____/ /__
#  /    / _// __/ _ `/  ' \\/ -_) |/|/ / _ \\/ __/  '_/
# /_/|_/_/ /_/  \\_,_/_/_/_/\\__/|__,__/\\___/_/ /_/\\_\\
";

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct PreservedComments {
    pub header: Vec<String>,
    pub before_workspace: Vec<String>,
    pub before_services: Vec<String>,
}

pub fn split_leading_comments_and_body(content: &str) -> (&str, &str) {
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

pub fn extract_preserved_comments(content: &str) -> PreservedComments {
    let banner_lines = NFW_YAML_BANNER_COMMENTS
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();
    let mut preserved = PreservedComments::default();
    let mut pending_comments = Vec::<String>::new();

    for line in content.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with('#') {
            if !banner_lines.contains(&trimmed) && trimmed != NFW_SCHEMA_DIRECTIVE_COMMENT {
                pending_comments.push(trimmed.to_owned());
            }
            continue;
        }

        if trimmed.is_empty() {
            continue;
        }

        if line.starts_with(' ') || line.starts_with('\t') {
            pending_comments.clear();
            continue;
        }

        if trimmed.starts_with("workspace:") {
            preserved.before_workspace.append(&mut pending_comments);
            continue;
        }

        if trimmed.starts_with("services:") {
            preserved.before_services.append(&mut pending_comments);
            continue;
        }

        if trimmed.starts_with("$schema:") {
            preserved.header.append(&mut pending_comments);
            continue;
        }

        pending_comments.clear();
    }

    if !pending_comments.is_empty() {
        preserved.header.append(&mut pending_comments);
    }

    preserved
}

pub fn ensure_schema_key(root_mapping: &mut Mapping) {
    root_mapping.insert(
        Value::String("$schema".to_owned()),
        Value::String(NFW_SCHEMA_URL.to_owned()),
    );
}

pub fn remove_workspace_project_guid(root_mapping: &mut Mapping) -> Result<(), String> {
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

pub fn reorder_root_keys(root_mapping: &mut Mapping) {
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

pub fn add_top_level_section_spacing(content: &str) -> String {
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

pub fn insert_preserved_section_comments(
    content: &str,
    preserved_comments: &PreservedComments,
) -> String {
    let mut formatted = String::new();
    let mut inserted_workspace = false;
    let mut inserted_services = false;

    for line in content.lines() {
        if line == "workspace:" && !inserted_workspace {
            for comment in &preserved_comments.before_workspace {
                formatted.push_str(comment);
                formatted.push('\n');
            }
            inserted_workspace = true;
        }

        if line == "services:" && !inserted_services {
            for comment in &preserved_comments.before_services {
                formatted.push_str(comment);
                formatted.push('\n');
            }
            inserted_services = true;
        }

        formatted.push_str(line);
        formatted.push('\n');
    }

    formatted
}

pub fn format_nfw_yaml_document(content: &str, preserved_comments: &PreservedComments) -> String {
    let formatted_body = add_top_level_section_spacing(content);
    let formatted_body_with_comments =
        insert_preserved_section_comments(&formatted_body, preserved_comments);
    let header_comment_block = preserved_comments.header.join("\n");

    if header_comment_block.is_empty() {
        return format!(
            "{NFW_YAML_BANNER_COMMENTS}\n{NFW_SCHEMA_DIRECTIVE_COMMENT}\n{formatted_body_with_comments}"
        );
    }

    format!(
        "{NFW_YAML_BANNER_COMMENTS}\n{header_comment_block}\n{NFW_SCHEMA_DIRECTIVE_COMMENT}\n{formatted_body_with_comments}"
    )
}

fn move_key_if_exists(source: &mut Mapping, destination: &mut Mapping, key: &str) {
    let key_value = Value::String(key.to_owned());
    if let Some(value) = source.remove(&key_value) {
        destination.insert(key_value, value);
    }
}
