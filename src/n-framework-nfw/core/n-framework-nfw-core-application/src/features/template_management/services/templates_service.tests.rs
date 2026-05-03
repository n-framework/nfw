use super::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::features::cli::configuration::abstractions::config_store::ConfigStore;
use crate::features::template_management::services::abstractions::git_repository::GitRepository;
use crate::features::template_management::services::abstractions::template_catalog_source::TemplateCatalogSource;
use crate::features::template_management::services::abstractions::template_source_synchronizer::TemplateSourceSynchronizer;
use crate::features::template_management::services::abstractions::validator::Validator;
use crate::features::template_management::services::abstractions::yaml_parser::YamlParser;
use crate::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use crate::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use crate::features::versioning::abstractions::version_comparator::VersionComparator;
use n_framework_nfw_core_domain::features::template_management::template_source::TemplateSource;

// --- Mocks ---

#[derive(Debug, Clone)]
struct TrackingConfigStore {
    sources: Arc<Mutex<Vec<TemplateSource>>>,
}

impl TrackingConfigStore {
    fn new(sources: Vec<TemplateSource>) -> Self {
        Self {
            sources: Arc::new(Mutex::new(sources)),
        }
    }
}

impl ConfigStore for TrackingConfigStore {
    fn load_sources(&self) -> Result<Vec<TemplateSource>, String> {
        self.sources
            .lock()
            .map(|sources| sources.clone())
            .map_err(|error| format!("failed to acquire sources lock: {error}"))
    }

    fn save_sources(&self, sources: &[TemplateSource]) -> Result<(), String> {
        let mut locked_sources = self
            .sources
            .lock()
            .map_err(|error| format!("failed to acquire sources lock: {error}"))?;
        *locked_sources = sources.to_vec();
        Ok(())
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

#[derive(Debug, Default, Clone)]
struct TrackingSourceSynchronizer {
    outcomes: HashMap<String, Result<(PathBuf, Option<String>), String>>,
    removed_sources: Arc<Mutex<Vec<String>>>,
}

impl TemplateSourceSynchronizer for TrackingSourceSynchronizer {
    fn sync_source(&self, source: &TemplateSource) -> Result<(PathBuf, Option<String>), String> {
        self.outcomes
            .get(&source.name)
            .cloned()
            .unwrap_or_else(|| Ok((PathBuf::from("/tmp/nfw/unused"), None)))
    }

    fn clear_source_cache(&self, source_name: &str) -> Result<(), String> {
        let mut removed_sources = self
            .removed_sources
            .lock()
            .map_err(|error| format!("failed to acquire removed sources lock: {error}"))?;
        removed_sources.push(source_name.to_owned());
        Ok(())
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

#[derive(Debug, Clone)]
struct MockGitRepository {
    valid_remote_urls: HashMap<String, bool>,
}

impl MockGitRepository {
    fn new(valid_remote_urls: HashMap<String, bool>) -> Self {
        Self { valid_remote_urls }
    }
}

impl GitRepository for MockGitRepository {
    fn clone(&self, _url: &str, _destination: &Path) -> Result<(), String> {
        Ok(())
    }

    fn fetch(&self, _repository_path: &Path) -> Result<(), String> {
        Ok(())
    }

    fn pull(&self, _repository_path: &Path) -> Result<(), String> {
        Ok(())
    }

    fn current_branch(&self, _repository_path: &Path) -> Result<String, String> {
        Ok("main".to_owned())
    }

    fn is_valid_repo(&self, _repository_path: &Path) -> Result<bool, String> {
        Ok(true)
    }

    fn is_valid_git_url_format(&self, url: &str) -> bool {
        url.starts_with("http://")
            || url.starts_with("https://")
            || url.starts_with("git@")
            || url.starts_with('/')
    }

    fn is_remote_url_reachable(&self, url: &str) -> bool {
        self.valid_remote_urls.get(url).copied().unwrap_or(true)
    }
}

// --- Tests ---

#[test]
fn lists_templates_from_single_source() {
    let root = PathBuf::from("/tmp/nfw/source-official");
    let template_path = root.join("web-api");
    let (service, _, _) = create_service(
        vec![TemplateSource::new(
            "official".to_owned(),
            "https://example.com/official.git".to_owned(),
        )],
        HashMap::from([("official".to_owned(), Ok((root.clone(), None)))]),
        HashMap::from([(root, vec![template_path.clone()])]),
        HashMap::from([(template_path, valid_template_yaml("web-api", "Web API"))]),
        HashMap::new(),
    );

    let (templates, warnings) = service.list_templates().expect("list should succeed");

    assert!(warnings.is_empty());
    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].id, "web-api");
    assert_eq!(templates[0].source_name, "official");
}

#[test]
fn adds_source_when_url_is_valid() {
    let (service, store, _) = create_service(
        vec![TemplateSource::new(
            "official".to_owned(),
            "https://github.com/n-framework/nfw-templates".to_owned(),
        )],
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::from([("https://example.com/my-team.git".to_owned(), true)]),
    );

    let result = service.add_source("my-team", "https://example.com/my-team.git");

    assert!(result.is_ok());
    let sources = store.load_sources().expect("sources should be loadable");
    assert_eq!(sources.len(), 2);
    assert!(sources.iter().any(|source| source.name == "my-team"));
}

#[test]
fn removes_source_and_clears_cache() {
    let (service, store, synchronizer) = create_service(
        vec![
            TemplateSource::new(
                "official".to_owned(),
                "https://github.com/n-framework/nfw-templates".to_owned(),
            ),
            TemplateSource::new(
                "my-team".to_owned(),
                "https://example.com/my-team.git".to_owned(),
            ),
        ],
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    );

    let result = service.remove_source("my-team");

    assert!(result.is_ok());
    let sources = store.load_sources().expect("sources should be loadable");
    assert_eq!(sources.len(), 1);
    assert!(sources.iter().all(|source| source.name != "my-team"));
    let removed_sources = synchronizer
        .removed_sources
        .lock()
        .expect("removed sources lock should be available");
    assert_eq!(removed_sources.as_slice(), ["my-team"]);
}

#[test]
fn initializes_default_official_source_once() {
    let (service, store, _) = create_service(
        Vec::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    );

    service
        .ensure_default_source_registered()
        .expect("default source should be initialized");
    service
        .ensure_default_source_registered()
        .expect("default source initialization should be idempotent");

    let sources = store.load_sources().expect("sources should be loadable");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].name, "official");
}

// --- Helpers ---

fn create_service(
    sources: Vec<TemplateSource>,
    sync_outcomes: HashMap<String, Result<(PathBuf, Option<String>), String>>,
    directories_by_root: HashMap<PathBuf, Vec<PathBuf>>,
    metadata_by_directory: HashMap<PathBuf, String>,
    valid_remote_urls: HashMap<String, bool>,
) -> (
    TemplatesService<
        TrackingSourceSynchronizer,
        MockCatalogSource,
        TestYamlParser,
        TestValidator,
        TestVersionComparator,
        TrackingConfigStore,
        MockGitRepository,
    >,
    TrackingConfigStore,
    TrackingSourceSynchronizer,
) {
    let store = TrackingConfigStore::new(sources);
    let synchronizer = TrackingSourceSynchronizer {
        outcomes: sync_outcomes,
        removed_sources: Arc::new(Mutex::new(Vec::new())),
    };
    let catalog_source = MockCatalogSource {
        directories_by_root,
        metadata_by_directory,
    };
    let catalog_parser =
        TemplateCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);
    let catalog_resolver = TemplateCatalogSourceResolver::new(catalog_source, catalog_parser);

    let service = TemplatesService::new(
        synchronizer.clone(),
        catalog_resolver,
        store.clone(),
        TestValidator,
        MockGitRepository::new(valid_remote_urls),
    );

    (service, store, synchronizer)
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
