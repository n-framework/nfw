use std::str::FromStr;

use nframework_nfw_core_domain::features::template_management::language::Language;
use nframework_nfw_core_domain::features::template_management::template_metadata::TemplateMetadata;
use nframework_nfw_core_domain::features::versioning::version::Version;

#[test]
fn validates_valid_metadata() {
    let metadata = TemplateMetadata {
        id: "web-api".to_owned(),
        name: "Web API".to_owned(),
        description: "A service template".to_owned(),
        version: Version::from_str("1.0.0").expect("version should parse"),
        language: Language::Rust,
        tags: vec!["api".to_owned(), "service".to_owned()],
        author: Some("NFW".to_owned()),
        min_cli_version: Some(Version::from_str("1.0.0").expect("version should parse")),
        source_url: Some("https://github.com/n-framework/templates.git".to_owned()),
    };

    assert!(metadata.validate().is_ok());
}

#[test]
fn rejects_missing_required_fields() {
    let metadata = TemplateMetadata {
        id: "".to_owned(),
        name: "".to_owned(),
        description: "".to_owned(),
        version: Version::from_str("1.0.0").expect("version should parse"),
        language: Language::Rust,
        tags: vec![],
        author: None,
        min_cli_version: None,
        source_url: None,
    };

    assert!(metadata.validate().is_err());
}

#[test]
fn rejects_invalid_values() {
    let metadata = TemplateMetadata {
        id: "Not-Kebab".to_owned(),
        name: "Template".to_owned(),
        description: "Description".to_owned(),
        version: Version::from_str("1.0.0").expect("version should parse"),
        language: Language::Dotnet,
        tags: vec!["".to_owned()],
        author: None,
        min_cli_version: None,
        source_url: None,
    };

    assert!(metadata.validate().is_err());
}

#[test]
fn accepts_optional_fields_as_none() {
    let metadata = TemplateMetadata {
        id: "worker-service".to_owned(),
        name: "Worker".to_owned(),
        description: "Worker template".to_owned(),
        version: Version::from_str("2.1.0").expect("version should parse"),
        language: Language::Go,
        tags: vec![],
        author: None,
        min_cli_version: None,
        source_url: None,
    };

    assert!(metadata.validate().is_ok());
}
