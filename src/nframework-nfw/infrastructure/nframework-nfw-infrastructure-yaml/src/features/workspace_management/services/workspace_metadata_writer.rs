use std::fs;
use std::path::Path;

use nframework_nfw_application::features::service_management::models::service_template_provenance_record::ServiceTemplateProvenanceRecord;
use nframework_nfw_application::features::service_management::services::abstraction::service_provenance_store::ServiceProvenanceStore;
use serde_yaml::{Mapping, Value};

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

        let serialized = serde_yaml::to_string(&root).map_err(|error| {
            format!(
                "failed to serialize workspace metadata file '{}': {error}",
                workspace_file.display()
            )
        })?;

        fs::write(&workspace_file, serialized).map_err(|error| {
            format!(
                "failed to write workspace metadata file '{}': {error}",
                workspace_file.display()
            )
        })
    }
}
