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

            let has_feature_tag = tags
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .any(|tag| tag.eq_ignore_ascii_case("feature"));
            if has_feature_tag {
                return Some("feature".to_owned());
            }

            None
        });

    inferred_from_tags.unwrap_or_else(|| "unknown".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infer_template_type_uses_explicit_type_when_available() {
        let yaml = "type: explicit_type";
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        assert_eq!(infer_template_type(&value), "explicit_type");
    }

    #[test]
    fn infer_template_type_trims_explicit_type() {
        let yaml = "type: '  my_type  '";
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        assert_eq!(infer_template_type(&value), "my_type");
    }

    #[test]
    fn infer_template_type_falls_back_to_tags_when_explicit_type_is_empty() {
        let yaml = "type: ''\ntags: ['service']";
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        assert_eq!(infer_template_type(&value), "service");
    }

    #[test]
    fn infer_template_type_infers_service_from_tags() {
        let yaml = "tags: ['other', 'SERVICE']";
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        assert_eq!(infer_template_type(&value), "service");
    }

    #[test]
    fn infer_template_type_infers_workspace_from_tags() {
        let yaml = "tags: ['Workspace']";
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        assert_eq!(infer_template_type(&value), "workspace");
    }

    #[test]
    fn infer_template_type_infers_feature_from_tags() {
        let yaml = "tags: ['FEATURE']";
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        assert_eq!(infer_template_type(&value), "feature");
    }

    #[test]
    fn infer_template_type_returns_unknown_when_no_tags_match() {
        let yaml = "tags: ['other']";
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        assert_eq!(infer_template_type(&value), "unknown");
    }

    #[test]
    fn infer_template_type_returns_unknown_when_missing_type_and_tags() {
        let yaml = "id: some_template";
        let value = serde_yaml::from_str::<Value>(yaml).unwrap();
        assert_eq!(infer_template_type(&value), "unknown");
    }

    #[test]
    fn read_template_type_parses_yaml_and_infers_type() {
        let sandbox = tempfile::tempdir().unwrap();
        let yaml_path = sandbox.path().join("template.yaml");
        fs::write(&yaml_path, "type: read_type").unwrap();

        let ty = read_template_type(sandbox.path()).unwrap();
        assert_eq!(ty, "read_type");
    }

    #[test]
    fn read_template_type_returns_error_if_file_missing() {
        let sandbox = tempfile::tempdir().unwrap();
        let result = read_template_type(sandbox.path());
        assert!(result.is_err());
    }

    #[test]
    fn read_template_type_returns_error_if_invalid_yaml() {
        let sandbox = tempfile::tempdir().unwrap();
        let yaml_path = sandbox.path().join("template.yaml");
        fs::write(&yaml_path, "invalid: yaml: {}").unwrap();
        let result = read_template_type(sandbox.path());
        assert!(result.is_err());
    }
}
