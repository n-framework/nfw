use super::*;
use crate::features::generator_management::models::errors::GeneratorCatalogSourceResolverError;
use crate::features::versioning::abstractions::version_comparator::VersionComparator;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Clone)]
struct TestCatalogSource {
    discovered_generator_directories: Vec<PathBuf>,
    metadata_by_directory: HashMap<PathBuf, String>,
    discovery_error: Option<String>,
    metadata_read_error_by_directory: HashMap<PathBuf, String>,
}

impl TestCatalogSource {
    fn with_generators(
        discovered_generator_directories: Vec<PathBuf>,
        metadata_by_directory: HashMap<PathBuf, String>,
    ) -> Self {
        Self {
            discovered_generator_directories,
            metadata_by_directory,
            discovery_error: None,
            metadata_read_error_by_directory: HashMap::new(),
        }
    }
}

impl GeneratorCatalogSource for TestCatalogSource {
    fn discover_generator_directories(&self, _source_root: &Path) -> Result<Vec<PathBuf>, String> {
        if let Some(error) = self.discovery_error.as_deref() {
            return Err(error.to_owned());
        }

        Ok(self.discovered_generator_directories.clone())
    }

    fn read_generator_metadata(&self, generator_directory: &Path) -> Result<String, String> {
        if let Some(error) = self
            .metadata_read_error_by_directory
            .get(generator_directory)
        {
            return Err(error.clone());
        }

        self.metadata_by_directory
            .get(generator_directory)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "missing test metadata for generator directory '{}'",
                    generator_directory.display()
                )
            })
    }
}

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
fn resolves_generator_catalog_from_discovered_directories() {
    let source_root = PathBuf::from("/tmp/nfw/catalog");
    let worker_generator_path = source_root.join("worker-generator");
    let web_generator_path = source_root.join("web-generator");

    let source = TestCatalogSource::with_generators(
        vec![worker_generator_path.clone(), web_generator_path.clone()],
        HashMap::from([
            (
                worker_generator_path.clone(),
                valid_generator_yaml("worker-service", "Worker Service"),
            ),
            (
                web_generator_path.clone(),
                valid_generator_yaml("web-api", "Web API"),
            ),
        ]),
    );

    let metadata_parser =
        GeneratorCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);
    let resolver = GeneratorCatalogSourceResolver::new(source, metadata_parser);

    let catalog = resolver
        .resolve("official", &source_root)
        .expect("catalog should resolve");

    assert_eq!(catalog.source_name, "official");
    assert_eq!(catalog.len(), 2);
    assert_eq!(catalog.generators[0].metadata.id, "web-api");
    assert_eq!(catalog.generators[0].cache_path, web_generator_path);
    assert_eq!(catalog.generators[1].metadata.id, "worker-service");
    assert_eq!(catalog.generators[1].cache_path, worker_generator_path);
}

#[test]
fn returns_source_scan_error_when_discovery_fails() {
    let source_root = PathBuf::from("/tmp/nfw/catalog");
    let source = TestCatalogSource {
        discovery_error: Some("permission denied".to_owned()),
        ..TestCatalogSource::default()
    };

    let metadata_parser =
        GeneratorCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);
    let resolver = GeneratorCatalogSourceResolver::new(source, metadata_parser);
    let error = resolver
        .resolve("official", &source_root)
        .expect_err("resolver should fail");

    match error {
        GeneratorCatalogSourceResolverError::SourceScanFailed {
            source_name,
            reason,
        } => {
            assert_eq!(source_name, "official");
            assert_eq!(reason, "permission denied");
        }
        _ => panic!("unexpected error variant"),
    }
}

#[test]
fn skips_invalid_generators_without_failing_the_catalog() {
    let source_root = PathBuf::from("/tmp/nfw/catalog");
    let generator_path = source_root.join("broken-generator");

    let source = TestCatalogSource::with_generators(
        vec![generator_path.clone()],
        HashMap::from([(
            generator_path.clone(),
            r#"
id: broken-generator
name: Broken
description: Invalid because version is missing
language: rust
"#
            .to_owned(),
        )]),
    );

    let metadata_parser =
        GeneratorCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);
    let resolver = GeneratorCatalogSourceResolver::new(source, metadata_parser);
    let catalog = resolver
        .resolve("official", &source_root)
        .expect("resolver should succeed and skip the invalid generator");

    assert_eq!(catalog.len(), 0);
}

fn valid_generator_yaml(id: &str, name: &str) -> String {
    format!(
        r#"
id: {id}
name: {name}
description: Valid generator
version: 1.0.0
language: rust
"#
    )
}
