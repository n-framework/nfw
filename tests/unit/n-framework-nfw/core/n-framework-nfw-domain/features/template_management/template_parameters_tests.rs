macro_rules! assert_json_eq {
    ($left:expr, $right:expr) => {
        assert_eq!(
            serde_json::to_value($left).unwrap(),
            serde_json::to_value($right).unwrap()
        );
    };
}

use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;

#[test]
fn new_creates_empty_parameters() {
    let params = TemplateParameters::new();
    assert!(params.as_map().is_empty());
}

#[test]
fn with_name_sets_name() {
    let params = TemplateParameters::new().with_name("MyService").unwrap();
    assert_eq!(params.name(), Some("MyService"));
    assert_eq!(params.get("Name"), Some("MyService"));
}

#[test]
fn with_feature_sets_feature() {
    let params = TemplateParameters::new().with_feature("Auth").unwrap();
    assert_eq!(params.feature(), Some("Auth"));
    assert_eq!(params.get("Feature"), Some("Auth"));
}

#[test]
fn with_namespace_sets_namespace() {
    let params = TemplateParameters::new()
        .with_namespace("Com.Example")
        .unwrap();
    assert_eq!(params.namespace(), Some("Com.Example"));
    assert_eq!(params.get("Namespace"), Some("Com.Example"));
}

#[test]
fn insert_validates_key_format() {
    let mut params = TemplateParameters::new();

    // Valid keys
    assert!(params.insert("HelloWorld123", "val").is_ok());
    assert!(params.insert("key.with.dots", "val").is_ok());
    assert!(params.insert("key-with-dashes", "val").is_ok());
    assert!(params.insert("key_with_underscores", "val").is_ok());
    assert!(params.insert("{{TOKEN}}", "val").is_ok());
    assert!(params.insert("__TOKEN__", "val").is_ok());

    // Invalid keys
    assert!(params.insert("", "val").is_err());
    assert!(params.insert("  ", "val").is_err());
    assert!(params.insert("key with spaces", "val").is_err());
    assert!(params.insert("key!@#", "val").is_err());
}

#[test]
fn insert_overwrites_existing_key() {
    let mut params = TemplateParameters::new();
    assert!(params.insert("Key", "Value1").is_ok());
    assert_eq!(params.get("Key"), Some("Value1"));

    assert!(params.insert("Key", "Value2").is_ok());
    assert_eq!(params.get("Key"), Some("Value2"));
}

#[test]
fn with_methods_fail_on_empty_values() {
    let params = TemplateParameters::new();

    assert!(params.clone().with_name("  ").is_err());
    assert!(params.clone().with_feature("").is_err());
    assert!(params.clone().with_namespace("\t").is_err());
}
#[test]
fn try_from_btreemap_validates_keys() {
    use std::collections::BTreeMap;
    let mut map = BTreeMap::new();
    map.insert("ValidKey".to_string(), "val".to_string());
    map.insert("Invalid key!".to_string(), "val".to_string());

    let result = TemplateParameters::try_from(map);
    assert!(result.is_err());
    assert!(result.unwrap_err().contains("invalid parameter key"));
}

#[test]
fn try_from_json_object_works() {
    use serde_json::json;
    let val = json!({
        "Name": "Service",
        "Feature": "Auth",
        "Nested": { "key": "val" }
    });

    let params = TemplateParameters::try_from(val).unwrap();
    assert_eq!(params.name(), Some("Service"));
    assert_eq!(params.feature(), Some("Auth"));
    assert_json_eq!(params.get_value("Nested").unwrap(), json!({ "key": "val" }));
}

#[test]
fn try_from_json_non_object_fails() {
    use serde_json::json;
    assert!(TemplateParameters::try_from(json!("string")).is_err());
    assert!(TemplateParameters::try_from(json!([1, 2, 3])).is_err());
}
