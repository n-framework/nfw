use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use serde::Serialize;
use serde::de::DeserializeOwned;

use nframework_nfw_application::features::cli::configuration::abstraction::config_store::ConfigStore;
use nframework_nfw_application::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use nframework_nfw_application::features::template_management::services::abstraction::git_repository::GitRepository;
use nframework_nfw_application::features::template_management::services::abstraction::template_catalog_source::TemplateCatalogSource;
use nframework_nfw_application::features::template_management::services::abstraction::template_source_synchronizer::TemplateSourceSynchronizer;
use nframework_nfw_application::features::template_management::services::abstraction::validator::Validator;
use nframework_nfw_application::features::template_management::services::abstraction::yaml_parser::YamlParser;
use nframework_nfw_application::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use nframework_nfw_application::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use nframework_nfw_application::features::template_management::services::templates_service::TemplatesService;
use nframework_nfw_application::features::versioning::abstraction::version_comparator::VersionComparator;
use nframework_nfw_domain::features::template_management::template_source::TemplateSource;

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
struct StubCatalogSource;

impl TemplateCatalogSource for StubCatalogSource {
    fn discover_template_directories(&self, _source_root: &Path) -> Result<Vec<PathBuf>, String> {
        Ok(Vec::new())
    }

    fn read_template_metadata(&self, _template_directory: &Path) -> Result<String, String> {
        Err("template metadata is not available in source management tests".to_owned())
    }
}

#[derive(Debug, Default, Clone)]
struct TrackingSourceSynchronizer {
    removed_sources: Arc<Mutex<Vec<String>>>,
}

impl TemplateSourceSynchronizer for TrackingSourceSynchronizer {
    fn sync_source(&self, _source: &TemplateSource) -> Result<(PathBuf, Option<String>), String> {
        Ok((PathBuf::from("/tmp/nfw/unused"), None))
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
        // Simple format validation for tests
        url.starts_with("http://")
            || url.starts_with("https://")
            || url.starts_with("git@")
            || url.starts_with('/')
    }

    fn is_remote_url_reachable(&self, url: &str) -> bool {
        self.valid_remote_urls.get(url).copied().unwrap_or(false)
    }
}

#[test]
fn adds_source_when_url_is_valid() {
    let (service, store, _) = create_service(
        vec![TemplateSource::new(
            "official".to_owned(),
            "https://github.com/n-framework/nfw-templates".to_owned(),
        )],
        HashMap::from([("https://example.com/my-team.git".to_owned(), true)]),
    );

    let result = service.add_source("my-team", "https://example.com/my-team.git");

    assert!(result.is_ok());
    let sources = store.load_sources().expect("sources should be loadable");
    assert_eq!(sources.len(), 2);
    assert!(sources.iter().any(|source| source.name == "my-team"));
}

#[test]
fn rejects_duplicate_source_name() {
    let (service, _, _) = create_service(
        vec![TemplateSource::new(
            "official".to_owned(),
            "https://github.com/n-framework/nfw-templates".to_owned(),
        )],
        HashMap::from([("https://example.com/my-team.git".to_owned(), true)]),
    );

    let result = service.add_source("official", "https://example.com/my-team.git");

    assert_eq!(
        result,
        Err(TemplatesServiceError::SourceAlreadyExists(
            "official".to_owned()
        ))
    );
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
    let (service, store, _) = create_service(Vec::new(), HashMap::new());

    service
        .ensure_default_source_registered()
        .expect("default source should be initialized");
    service
        .ensure_default_source_registered()
        .expect("default source initialization should be idempotent");

    let sources = store.load_sources().expect("sources should be loadable");
    assert_eq!(sources.len(), 1);
    assert_eq!(sources[0].name, "official");
    assert_eq!(
        sources[0].url,
        "https://github.com/n-framework/nfw-templates"
    );
}

type TestTemplatesService = TemplatesService<
    TrackingSourceSynchronizer,
    StubCatalogSource,
    TestYamlParser,
    TestValidator,
    TestVersionComparator,
    TrackingConfigStore,
    MockGitRepository,
>;

fn create_service(
    sources: Vec<TemplateSource>,
    valid_remote_urls: HashMap<String, bool>,
) -> (
    TestTemplatesService,
    TrackingConfigStore,
    TrackingSourceSynchronizer,
) {
    let store = TrackingConfigStore::new(sources);
    let synchronizer = TrackingSourceSynchronizer::default();
    let parser = TemplateCatalogParser::new(TestYamlParser, TestValidator, TestVersionComparator);
    let resolver = TemplateCatalogSourceResolver::new(StubCatalogSource, parser);
    let service = TemplatesService::new(
        synchronizer.clone(),
        resolver,
        store.clone(),
        TestValidator,
        MockGitRepository::new(valid_remote_urls),
    );

    (service, store, synchronizer)
}
