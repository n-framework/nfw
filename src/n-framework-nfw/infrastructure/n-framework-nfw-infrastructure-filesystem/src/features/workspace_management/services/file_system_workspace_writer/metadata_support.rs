use n_framework_nfw_core_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use n_framework_nfw_infrastructure_workspace_metadata::{
    NFW_SCHEMA_URL, ensure_schema_key, extract_preserved_comments, format_nfw_yaml_document,
    remove_workspace_project_guid, reorder_root_keys, split_leading_comments_and_body,
};
use serde_yaml::Value;
use std::fs;
use std::path::Path;

pub fn ensure_workspace_metadata_file(
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

pub fn normalize_workspace_metadata_file(output_root: &Path) -> Result<(), String> {
    let workspace_metadata_path = output_root.join("nfw.yaml");
    let content = fs::read_to_string(&workspace_metadata_path).map_err(|error| {
        format!(
            "failed to read workspace metadata file '{}': {error}",
            workspace_metadata_path.display()
        )
    })?;
    let preserved_comments = extract_preserved_comments(&content);

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
    reorder_root_keys(root_mapping);

    let serialized = serde_yaml::to_string(&root).map_err(|error| {
        format!(
            "failed to serialize workspace metadata file '{}': {error}",
            workspace_metadata_path.display()
        )
    })?;
    let formatted_document = format_nfw_yaml_document(&serialized, &preserved_comments);

    fs::write(&workspace_metadata_path, formatted_document).map_err(|error| {
        format!(
            "failed to write workspace metadata file '{}': {error}",
            workspace_metadata_path.display()
        )
    })
}

pub fn ensure_workspace_metadata_banner_comments(output_root: &Path) -> Result<(), String> {
    let workspace_metadata_path = output_root.join("nfw.yaml");
    let content = fs::read_to_string(&workspace_metadata_path).map_err(|error| {
        format!(
            "failed to read workspace metadata file '{}': {error}",
            workspace_metadata_path.display()
        )
    })?;

    let (_, yaml_body) = split_leading_comments_and_body(&content);
    let preserved_comments = extract_preserved_comments(&content);
    let formatted_document = format_nfw_yaml_document(yaml_body, &preserved_comments);

    fs::write(&workspace_metadata_path, formatted_document).map_err(|error| {
        format!(
            "failed to write workspace metadata file '{}': {error}",
            workspace_metadata_path.display()
        )
    })
}
