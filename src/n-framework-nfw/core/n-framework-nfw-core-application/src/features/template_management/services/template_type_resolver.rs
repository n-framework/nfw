use std::fs;
use std::path::Path;

use serde_yaml::Value;

pub fn read_template_type(template_cache_path: &Path) -> Result<String, String> {
    let metadata_path = template_cache_path.join("template.yaml");
    let content = fs::read_to_string(&metadata_path)
        .map_err(|error| format!("failed to read '{}': {error}", metadata_path.display()))?;
    let value = serde_yaml::from_str::<Value>(&content).map_err(|error| {
        format!(
            "invalid template metadata '{}': {error}",
            metadata_path.display()
        )
    })?;

    Ok(infer_template_type(&value))
}

fn infer_template_type(value: &Value) -> String {
    let explicit_type = value
        .get("type")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|candidate| !candidate.is_empty())
        .map(ToOwned::to_owned);
    if let Some(explicit_type) = explicit_type {
        return explicit_type;
    }

    let inferred_from_tags = value
        .get("tags")
        .and_then(Value::as_sequence)
        .and_then(|tags| {
            let has_service_tag = tags
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .any(|tag| tag.eq_ignore_ascii_case("service"));
            if has_service_tag {
                return Some("service".to_owned());
            }

            let has_workspace_tag = tags
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .any(|tag| tag.eq_ignore_ascii_case("workspace"));
            if has_workspace_tag {
                return Some("workspace".to_owned());
            }

            None
        });

    inferred_from_tags.unwrap_or_else(|| "unknown".to_owned())
}
