//! Shared workspace metadata YAML document helpers.

use std::collections::HashMap;

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
    pub footer: Vec<String>,
    pub path_comments: HashMap<Vec<String>, Vec<String>>,
    pub inline_comments: HashMap<Vec<String>, String>,
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
    let mut path_stack: Vec<(usize, String)> = Vec::new();

    for line in content.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.starts_with('#') {
            if !banner_lines.contains(&trimmed_line) && trimmed_line != NFW_SCHEMA_DIRECTIVE_COMMENT
            {
                pending_comments.push(trimmed_line.to_owned());
            }
            continue;
        }

        if trimmed_line.is_empty() {
            continue;
        }

        if let Some(colon_idx) = trimmed_line.find(':') {
            let key_part = &trimmed_line[..colon_idx];
            let key = key_part.trim();
            let indent = line.chars().take_while(|c| c.is_whitespace()).count();

            while let Some((last_indent, _)) = path_stack.last() {
                if *last_indent >= indent {
                    path_stack.pop();
                } else {
                    break;
                }
            }

            let mut full_path: Vec<String> = path_stack.iter().map(|(_, k)| k.clone()).collect();
            full_path.push(key.to_string());

            if !pending_comments.is_empty() {
                preserved
                    .path_comments
                    .insert(full_path.clone(), std::mem::take(&mut pending_comments));
            }

            // Check for inline comment
            if let Some(comment_idx) = trimmed_line.find('#').filter(|&idx| idx > colon_idx) {
                let inline_comment = &trimmed_line[comment_idx..];
                preserved
                    .inline_comments
                    .insert(full_path.clone(), inline_comment.to_string());
            }

            path_stack.push((indent, key.to_string()));
        } else {
            // Not a key line, might be a value line. If it has comments, they are already handled.
            // If it's just data, clear pending comments as they were likely not documentation for the next key.
            pending_comments.clear();
        }
    }

    if !pending_comments.is_empty() {
        preserved.footer = pending_comments;
    }

    preserved
}

pub fn ensure_schema_key(root_mapping: &mut serde_yaml::Mapping) {
    root_mapping.insert(
        serde_yaml::Value::String("$schema".to_owned()),
        serde_yaml::Value::String(NFW_SCHEMA_URL.to_owned()),
    );
}

pub fn remove_workspace_project_guid(root_mapping: &mut serde_yaml::Mapping) -> Result<(), String> {
    let workspace_key = serde_yaml::Value::String("workspace".to_owned());
    let Some(workspace_value) = root_mapping.get_mut(&workspace_key) else {
        return Ok(());
    };

    let workspace_mapping = workspace_value
        .as_mapping_mut()
        .ok_or_else(|| "'workspace' must be a YAML mapping".to_owned())?;
    workspace_mapping.remove(serde_yaml::Value::String("projectGuid".to_owned()));
    Ok(())
}

pub fn reorder_root_keys(root_mapping: &mut serde_yaml::Mapping) {
    let mut reordered = serde_yaml::Mapping::new();
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

pub fn insert_preserved_comments(content: &str, preserved_comments: &PreservedComments) -> String {
    let mut formatted = String::new();
    let mut path_stack: Vec<(usize, String)> = Vec::new();

    for line in content.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() {
            formatted.push_str(line);
            formatted.push('\n');
            continue;
        }

        if let Some(colon_idx) = trimmed_line.find(':') {
            let key = &trimmed_line[..colon_idx].trim();
            let indent = line.chars().take_while(|c| c.is_whitespace()).count();

            while let Some((last_indent, _)) = path_stack.last() {
                if *last_indent >= indent {
                    path_stack.pop();
                } else {
                    break;
                }
            }

            let mut full_path: Vec<String> = path_stack.iter().map(|(_, k)| k.clone()).collect();
            full_path.push(key.to_string());

            if let Some(comments) = preserved_comments.path_comments.get(&full_path) {
                for comment in comments {
                    formatted.push_str(&" ".repeat(indent));
                    formatted.push_str(comment);
                    formatted.push('\n');
                }
            }

            path_stack.push((indent, key.to_string()));

            formatted.push_str(line);
            if let Some(inline_comment) = preserved_comments.inline_comments.get(&full_path) {
                formatted.push(' ');
                formatted.push_str(inline_comment);
            }
            formatted.push('\n');
            continue;
        }

        formatted.push_str(line);
        formatted.push('\n');
    }

    for comment in &preserved_comments.footer {
        formatted.push_str(comment);
        formatted.push('\n');
    }

    formatted
}

pub fn format_nfw_yaml_document(content: &str, preserved_comments: &PreservedComments) -> String {
    let formatted_body = add_top_level_section_spacing(content);
    let formatted_body_with_comments =
        insert_preserved_comments(&formatted_body, preserved_comments);
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

fn move_key_if_exists(
    source: &mut serde_yaml::Mapping,
    destination: &mut serde_yaml::Mapping,
    key: &str,
) {
    let key_value = serde_yaml::Value::String(key.to_owned());
    if let Some(value) = source.remove(&key_value) {
        destination.insert(key_value, value);
    }
}
