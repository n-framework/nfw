use std::fs;
use std::path::Path;

use nframework_nfw_application::features::service_management::models::service_template_provenance_record::ServiceTemplateProvenanceRecord;
use nframework_nfw_application::features::service_management::services::abstraction::service_provenance_store::ServiceProvenanceStore;
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

#[derive(Debug, Default, Clone, Copy)]
pub struct WorkspaceMetadataWriter;

impl WorkspaceMetadataWriter {
    pub fn new() -> Self {
        Self
    }
}

impl ServiceProvenanceStore for WorkspaceMetadataWriter {
    fn persist_service_provenance(
        &self,
        workspace_root: &Path,
        record: &ServiceTemplateProvenanceRecord,
    ) -> Result<(), String> {
        let workspace_file = workspace_root.join("nfw.yaml");
        if !workspace_file.is_file() {
            return Err(format!(
                "workspace metadata file '{}' does not exist",
                workspace_file.display()
            ));
        }

        let content = fs::read_to_string(&workspace_file).map_err(|error| {
            format!(
                "failed to read workspace metadata file '{}': {error}",
                workspace_file.display()
            )
        })?;
        let mut root = serde_yaml::from_str::<Value>(&content).map_err(|error| {
            format!(
                "failed to parse workspace metadata file '{}': {error}",
                workspace_file.display()
            )
        })?;

        let root_mapping = root
            .as_mapping_mut()
            .ok_or_else(|| "workspace metadata root must be a YAML mapping".to_owned())?;
        ensure_schema_key(root_mapping);
        remove_workspace_project_guid(root_mapping)?;

        let services_key = Value::String("services".to_owned());
        if !root_mapping.contains_key(&services_key) {
            root_mapping.insert(services_key.clone(), Value::Mapping(Mapping::new()));
        }

        let services_mapping = root_mapping
            .get_mut(&services_key)
            .and_then(Value::as_mapping_mut)
            .ok_or_else(|| "'services' must be a YAML mapping".to_owned())?;

        let mut template_mapping = Mapping::new();
        template_mapping.insert(
            Value::String("id".to_owned()),
            Value::String(record.template_id.clone()),
        );
        template_mapping.insert(
            Value::String("version".to_owned()),
            Value::String(record.template_version.clone()),
        );

        let mut service_entry = Mapping::new();
        service_entry.insert(
            Value::String("path".to_owned()),
            Value::String(format!("src/{}", record.service_name)),
        );
        service_entry.insert(
            Value::String("template".to_owned()),
            Value::Mapping(template_mapping),
        );
        service_entry.insert(
            Value::String("generatedAtUtc".to_owned()),
            Value::String(record.generated_at_utc.clone()),
        );

        services_mapping.insert(
            Value::String(record.service_name.clone()),
            Value::Mapping(service_entry),
        );
        reorder_root_keys(root_mapping);

        let serialized = serde_yaml::to_string(&root).map_err(|error| {
            format!(
                "failed to serialize workspace metadata file '{}': {error}",
                workspace_file.display()
            )
        })?;
        let rewritten = format_nfw_yaml_document(&serialized);

        fs::write(&workspace_file, rewritten).map_err(|error| {
            format!(
                "failed to write workspace metadata file '{}': {error}",
                workspace_file.display()
            )
        })
    }
}

fn ensure_schema_key(root_mapping: &mut Mapping) {
    root_mapping.insert(
        Value::String("$schema".to_owned()),
        Value::String(NFW_SCHEMA_URL.to_owned()),
    );
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

fn format_nfw_yaml_document(serialized_yaml_body: &str) -> String {
    let formatted_body = add_top_level_section_spacing(serialized_yaml_body);
    format!("{NFW_YAML_BANNER_COMMENTS}\n{NFW_SCHEMA_DIRECTIVE_COMMENT}\n{formatted_body}")
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
