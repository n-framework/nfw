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
