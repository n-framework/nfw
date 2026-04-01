use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde::de::DeserializeOwned;

use nframework_nfw_application::features::cli::configuration::abstraction::config_store::ConfigStore;
use nframework_nfw_application::features::template_management::services::abstraction::template_catalog_source::TemplateCatalogSource;
use nframework_nfw_application::features::template_management::services::abstraction::template_source_synchronizer::TemplateSourceSynchronizer;
use nframework_nfw_application::features::template_management::services::abstraction::validator::Validator;
use nframework_nfw_application::features::template_management::services::abstraction::yaml_parser::YamlParser;
use nframework_nfw_application::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use nframework_nfw_application::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use nframework_nfw_application::features::template_management::services::templates_service::TemplatesService;
use nframework_nfw_application::features::versioning::abstraction::version_comparator::VersionComparator;
use nframework_nfw_domain::features::template_management::template_source::TemplateSource;

#[derive(Debug, Default, Clone)]
struct MockConfigStore {
    sources: Vec<TemplateSource>,
}

impl ConfigStore for MockConfigStore {
    fn load_sources(&self) -> Result<Vec<TemplateSource>, String> {
        Ok(self.sources.clone())
    }

    fn save_sources(&self, _sources: &[TemplateSource]) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
struct MockSourceSynchronizer {
    outcomes: HashMap<String, Result<(PathBuf, Option<String>), String>>,
}

impl TemplateSourceSynchronizer for MockSourceSynchronizer {
    fn sync_source(&self, source: &TemplateSource) -> Result<(PathBuf, Option<String>), String> {
        self.outcomes
            .get(&source.name)
            .cloned()
            .unwrap_or_else(|| Err("source not configured in test synchronizer".to_owned()))
    }
}

#[derive(Debug, Default, Clone)]
struct MockCatalogSource {
    directories_by_root: HashMap<PathBuf, Vec<PathBuf>>,
    metadata_by_directory: HashMap<PathBuf, String>,
}

impl TemplateCatalogSource for MockCatalogSource {
    fn discover_template_directories(&self, source_root: &Path) -> Result<Vec<PathBuf>, String> {
        Ok(self
            .directories_by_root
            .get(source_root)
            .cloned()
            .unwrap_or_default())
    }

    fn read_template_metadata(&self, template_directory: &Path) -> Result<String, String> {
        self.metadata_by_directory
            .get(template_directory)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "template metadata is not configured for '{}'",
                    template_directory.display()
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
fn lists_templates_from_single_source() {
    let root = PathBuf::from("/tmp/nfw/source-official");
    let template_path = root.join("web-api");
    let service = create_service(
        vec![TemplateSource::new(
            "official".to_owned(),
            "https://example.com/official.git".to_owned(),
            true,
        )],
        HashMap::from([("official".to_owned(), Ok((root.clone(), None)))]),
        HashMap::from([(root, vec![template_path.clone()])]),
        HashMap::from([(template_path, valid_template_yaml("web-api", "Web API"))]),
    );

    let (templates, warnings) = service.list_templates().expect("list should succeed");

    assert!(warnings.is_empty());
    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].id, "web-api");
    assert_eq!(templates[0].source_name, "official");
}

#[test]
fn lists_templates_from_multiple_sources() {
    let official_root = PathBuf::from("/tmp/nfw/source-official");
    let community_root = PathBuf::from("/tmp/nfw/source-community");
    let official_template_path = official_root.join("web-api");
    let community_template_path = community_root.join("worker-service");

    let service = create_service(
        vec![
            TemplateSource::new(
                "official".to_owned(),
                "https://example.com/official.git".to_owned(),
                true,
            ),
            TemplateSource::new(
                "community".to_owned(),
                "https://example.com/community.git".to_owned(),
                true,
            ),
        ],
        HashMap::from([
            ("official".to_owned(), Ok((official_root.clone(), None))),
            ("community".to_owned(), Ok((community_root.clone(), None))),
        ]),
        HashMap::from([
            (official_root, vec![official_template_path.clone()]),
            (community_root, vec![community_template_path.clone()]),
        ]),
        HashMap::from([
            (
                official_template_path,
                valid_template_yaml("web-api", "Web API"),
            ),
            (
                community_template_path,
                valid_template_yaml("worker-service", "Worker Service"),
            ),
        ]),
    );

    let (templates, warnings) = service.list_templates().expect("list should succeed");

    assert!(warnings.is_empty());
    assert_eq!(templates.len(), 2);
    assert_eq!(templates[0].id, "web-api");
    assert_eq!(templates[1].id, "worker-service");
}

#[test]
fn returns_warning_for_empty_source() {
    let empty_root = PathBuf::from("/tmp/nfw/source-empty");
    let service = create_service(
        vec![TemplateSource::new(
            "empty".to_owned(),
            "https://example.com/empty.git".to_owned(),
            true,
        )],
        HashMap::from([("empty".to_owned(), Ok((empty_root.clone(), None)))]),
        HashMap::from([(empty_root, Vec::new())]),
        HashMap::new(),
    );

    let (templates, warnings) = service.list_templates().expect("list should succeed");

    assert!(templates.is_empty());
    assert_eq!(warnings.len(), 1);
    assert!(warnings[0].contains("contains no valid templates"));
}

#[test]
fn falls_back_when_a_source_is_unreachable() {
    let official_root = PathBuf::from("/tmp/nfw/source-official");
    let official_template_path = official_root.join("web-api");

    let service = create_service(
        vec![
            TemplateSource::new(
                "official".to_owned(),
                "https://example.com/official.git".to_owned(),
                true,
            ),
            TemplateSource::new(
                "broken".to_owned(),
                "https://example.com/broken.git".to_owned(),
                true,
            ),
        ],
        HashMap::from([
            ("official".to_owned(), Ok((official_root.clone(), None))),
            ("broken".to_owned(), Err("network offline".to_owned())),
        ]),
        HashMap::from([(official_root, vec![official_template_path.clone()])]),
        HashMap::from([(
            official_template_path,
            valid_template_yaml("web-api", "Web API"),
        )]),
    );

    let (templates, warnings) = service.list_templates().expect("list should succeed");

    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].id, "web-api");
    assert!(
        warnings
            .iter()
            .any(|warning| warning.contains("unreachable"))
    );
}

fn create_service(
    sources: Vec<TemplateSource>,
    sync_outcomes: HashMap<String, Result<(PathBuf, Option<String>), String>>,
    directories_by_root: HashMap<PathBuf, Vec<PathBuf>>,
    metadata_by_directory: HashMap<PathBuf, String>,
) -> TemplatesService<
    MockSourceSynchronizer,
    MockCatalogSource,
    TestYamlParser,
    TestValidator,
    TestVersionComparator,
    MockConfigStore,
> {
    let source_synchronizer = MockSourceSynchronizer {
        outcomes: sync_outcomes,
    };
    let catalog_source = MockCatalogSource {
        directories_by_root,
        metadata_by_directory,
    };
    let catalog_parser =
        TemplateCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);
    let catalog_resolver = TemplateCatalogSourceResolver::new(catalog_source, catalog_parser);
    let config_store = MockConfigStore { sources };

    TemplatesService::new(source_synchronizer, catalog_resolver, config_store)
}

fn valid_template_yaml(id: &str, name: &str) -> String {
    format!(
        r#"
id: {id}
name: {name}
description: Valid template
version: 1.0.0
language: rust
"#
    )
}
