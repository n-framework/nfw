use std::cmp::Ordering;

use serde::Serialize;
use serde::de::DeserializeOwned;

use nframework_nfw_core_application::features::template_management::services::abstractions::validator::Validator;
use nframework_nfw_core_application::features::template_management::services::abstractions::yaml_parser::YamlParser;
use nframework_nfw_core_application::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use nframework_nfw_core_application::features::versioning::abstractions::version_comparator::VersionComparator;
use nframework_nfw_core_domain::features::template_management::language::Language;

#[derive(Debug, Default, Clone, Copy)]
struct TestYamlParser;

impl YamlParser for TestYamlParser {
    fn parse<T>(&self, content: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        serde_yaml::from_str(content).map_err(|error| error.to_string())
    }

    fn serialize<T>(&self, value: &T) -> Result<String, String>
    where
        T: Serialize,
    {
        serde_yaml::to_string(value).map_err(|error| error.to_string())
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct TestValidator;

impl Validator for TestValidator {
    fn is_kebab_case(&self, value: &str) -> bool {
        if value.starts_with('-') || value.ends_with('-') || value.contains("--") {
            return false;
        }

        value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    }

    fn is_git_url(&self, value: &str) -> bool {
        value.starts_with("http://") || value.starts_with("https://") || value.starts_with("git@")
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct TestVersionComparator;

impl VersionComparator for TestVersionComparator {
    fn parse(&self, version: &str) -> Result<(), String> {
        semver::Version::parse(version)
            .map(|_| ())
            .map_err(|error| error.to_string())
    }

    fn compare(&self, left: &str, right: &str) -> Result<Ordering, String> {
        let left = semver::Version::parse(left).map_err(|error| error.to_string())?;
        let right = semver::Version::parse(right).map_err(|error| error.to_string())?;
        Ok(left.cmp(&right))
    }

    fn is_stable(&self, version: &str) -> Result<bool, String> {
        let parsed = semver::Version::parse(version).map_err(|error| error.to_string())?;
        Ok(parsed.pre.is_empty())
    }

    fn satisfies(&self, version: &str, requirement: &str) -> Result<bool, String> {
        let version = semver::Version::parse(version).map_err(|error| error.to_string())?;
        let requirement =
            semver::VersionReq::parse(requirement).map_err(|error| error.to_string())?;
        Ok(requirement.matches(&version))
    }
}

#[test]
fn parses_valid_yaml() {
    let parser = TemplateCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);

    let metadata = parser
        .parse_template_metadata(
            r#"
id: web-api
name: Web API
description: Production-ready web API template
version: 1.2.3
language: rust
tags:
  - api
  - service
author: N Framework
min_cli_version: 1.0.0
source_url: https://github.com/n-framework/templates.git
"#,
        )
        .expect("metadata should parse");

    assert_eq!(metadata.id, "web-api");
    assert_eq!(metadata.language, Language::Rust);
}

#[test]
fn rejects_invalid_yaml() {
    let parser = TemplateCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);

    let result = parser.parse_template_metadata("id: [");

    assert!(result.is_err());
}

#[test]
fn rejects_missing_required_fields() {
    let parser = TemplateCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);

    let result = parser.parse_template_metadata(
        r#"
id: web-api
name: ""
description: ""
version: 1.0.0
language: rust
"#,
    );

    assert!(result.is_err());
}

#[test]
fn accepts_missing_language_as_neutral() {
    let parser = TemplateCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);

    let metadata = parser
        .parse_template_metadata(
            r#"
id: blank-workspace
name: Blank Workspace
description: Minimal starter workspace template
version: 1.0.0
"#,
        )
        .expect("metadata should parse");

    assert_eq!(metadata.language, Language::Neutral);
}
