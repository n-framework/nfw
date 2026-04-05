//! Tests for offline/fallback behavior when template sources are unreachable.
//!
//! These tests verify that when all sources are unreachable (network failure),
//! the system can fall back to cached templates and provide appropriate warnings.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use nframework_nfw_core_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use nframework_nfw_core_application::features::cli::configuration::abstractions::path_resolver::PathResolver;
use nframework_nfw_core_application::features::template_management::services::abstractions::git_repository::GitRepository;
use nframework_nfw_core_application::features::template_management::services::abstractions::template_source_synchronizer::TemplateSourceSynchronizer;
use nframework_nfw_core_application::features::template_management::services::abstractions::validator::Validator;
use nframework_nfw_core_application::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use nframework_nfw_core_application::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use nframework_nfw_core_application::features::template_management::services::templates_service::TemplatesService;
use nframework_nfw_core_domain::features::template_management::template_source::TemplateSource;
use nframework_nfw_infrastructure_filesystem::features::template_management::services::local_templates_catalog_source::LocalTemplatesCatalogSource;
use nframework_nfw_infrastructure_filesystem::features::template_management::services::placeholder_detector::PlaceholderDetector;
use nframework_nfw_infrastructure_git::features::template_management::services::cli_git_repository::CliGitRepository;
use nframework_nfw_infrastructure_git::features::template_management::services::git_template_catalog_source::GitTemplateCatalogSource;
use nframework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use nframework_nfw_infrastructure_yaml::features::template_management::services::serde_yaml_parser::SerdeYamlParser;

/// Creates a temporary sandbox directory for testing
fn create_sandbox_directory() -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let sandbox = std::env::temp_dir().join(format!("nfw-test-offline-{}", timestamp));
    fs::create_dir_all(&sandbox).expect("failed to create sandbox directory");
    sandbox
}

/// Creates a bare git repository
fn create_bare_repository(path: &Path) {
    fs::create_dir_all(path).expect("failed to create repository directory");
    Command::new("git")
        .arg("init")
        .arg("--bare")
        .arg(path)
        .output()
        .expect("failed to initialize bare repository");
}

/// Creates a template repository with a sample template
fn create_template_repository(path: &Path, template_id: &str) {
    fs::create_dir_all(path).expect("failed to create template directory");

    // Initialize git repository
    Command::new("git")
        .arg("init")
        .current_dir(path)
        .output()
        .expect("failed to initialize git repository");

    let content_dir = path.join("content");
    fs::create_dir_all(&content_dir).expect("failed to create content directory");

    let metadata_content = format!(
        r#"
id: {}
name: {} Template
description: A test template
version: 1.0.0
language: rust
tags:
  - test
"#,
        template_id, template_id
    );

    fs::write(path.join("template.yaml"), metadata_content).expect("failed to write template.yaml");

    fs::write(
        content_dir.join("main.rs"),
        format!("fn main() {{ println!(\"Hello from {}!\"); }}", template_id),
    )
    .expect("failed to write main.rs");

    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(path)
        .output()
        .expect("failed to add files");

    Command::new("git")
        .arg("config")
        .arg("user.email")
        .arg("test@example.com")
        .current_dir(path)
        .output()
        .expect("failed to set git user email");

    Command::new("git")
        .arg("config")
        .arg("user.name")
        .arg("Test User")
        .current_dir(path)
        .output()
        .expect("failed to set git user name");

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("Initial commit")
        .current_dir(path)
        .output()
        .expect("failed to commit");
}

#[derive(Debug, Clone)]
struct TestConfigStore {
    sources: Vec<TemplateSource>,
}

impl ConfigStore for TestConfigStore {
    fn load_sources(&self) -> Result<Vec<TemplateSource>, String> {
        Ok(self.sources.clone())
    }

    fn save_sources(&self, _sources: &[TemplateSource]) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct TestPathResolver {
    cache_directory: PathBuf,
    config_directory: PathBuf,
}

impl PathResolver for TestPathResolver {
    fn cache_dir(&self) -> Result<PathBuf, String> {
        Ok(self.cache_directory.clone())
    }

    fn config_dir(&self) -> Result<PathBuf, String> {
        Ok(self.config_directory.clone())
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
        value.starts_with("http://")
            || value.starts_with("https://")
            || value.starts_with("git@")
            || Path::new(value).exists()
    }
}

/// Mock synchronizer that returns a cache path with a stale-data warning.
#[derive(Debug, Clone)]
struct CachedWarningSynchronizer {
    cache_path: PathBuf,
}

impl TemplateSourceSynchronizer for CachedWarningSynchronizer {
    fn sync_source(&self, source: &TemplateSource) -> Result<(PathBuf, Option<String>), String> {
        Ok((
            self.cache_path.clone(),
            Some(format!(
                "could not refresh remote '{}'; using existing cache (simulated offline)",
                source.url
            )),
        ))
    }

    fn clear_source_cache(&self, _source_name: &str) -> Result<(), String> {
        Ok(())
    }
}

#[test]
fn uses_cached_templates_when_all_sources_unreachable() {
    let sandbox = create_sandbox_directory();
    let remote_repository = sandbox.join("remote-catalog.git");
    let seed_repository = sandbox.join("seed-repository");
    let cache_directory = sandbox.join("cache");
    let config_directory = sandbox.join("config");

    fs::create_dir_all(&cache_directory).expect("failed to create cache directory");
    fs::create_dir_all(&config_directory).expect("failed to create config directory");

    create_bare_repository(&remote_repository);
    create_template_repository(&seed_repository, "offline-test");

    Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(remote_repository.as_os_str())
        .current_dir(&seed_repository)
        .output()
        .expect("failed to add remote");

    Command::new("git")
        .arg("push")
        .arg("-u")
        .arg("origin")
        .arg("main")
        .current_dir(&seed_repository)
        .output()
        .expect("failed to push to remote");

    let path_resolver = TestPathResolver {
        cache_directory: cache_directory.clone(),
        config_directory: config_directory.clone(),
    };

    let git_repository = CliGitRepository::new();
    let real_synchronizer = GitTemplateCatalogSource::new(git_repository, path_resolver);
    let catalog_source = LocalTemplatesCatalogSource::new(PlaceholderDetector::new());
    let catalog_parser = TemplateCatalogParser::new(
        SerdeYamlParser::new(),
        TestValidator,
        SemverVersionComparator::new(),
    );
    let catalog_resolver = TemplateCatalogSourceResolver::new(catalog_source, catalog_parser);

    let config_store = TestConfigStore {
        sources: vec![TemplateSource::new(
            "test-source".to_owned(),
            remote_repository.to_str().unwrap().to_owned(),
        )],
    };

    let templates_service = TemplatesService::new(
        real_synchronizer,
        catalog_resolver,
        config_store,
        TestValidator,
        CliGitRepository::new(),
    );

    let (templates, warnings) = templates_service
        .list_templates()
        .expect("first sync should succeed");

    assert!(
        !templates.is_empty(),
        "should discover templates on first sync"
    );
    assert!(
        warnings.is_empty(),
        "first sync should not produce warnings"
    );

    let cache_path = cache_directory.join("templates/test-source");
    Command::new("git")
        .arg("remote")
        .arg("set-url")
        .arg("origin")
        .arg(cache_directory.join("missing-remote.git").as_os_str())
        .current_dir(&cache_path)
        .output()
        .expect("failed to repoint remote to missing path");

    let (offline_templates, offline_warnings) = templates_service
        .list_templates()
        .expect("cached templates should be used when remote is unreachable");

    assert!(
        !offline_templates.is_empty(),
        "should still discover templates from cache"
    );
    assert!(
        offline_warnings
            .iter()
            .any(|warning| warning.contains("using cached data")),
        "should warn about stale cache usage"
    );

    let _ = fs::remove_dir_all(sandbox);
}

#[test]
fn warns_about_stale_data_when_using_cached_templates() {
    let sandbox = create_sandbox_directory();
    let cache_directory = sandbox.join("cache");

    fs::create_dir_all(&cache_directory).expect("failed to create cache directory");

    let cache_path = cache_directory.join("templates/test-source");
    fs::create_dir_all(&cache_path).expect("failed to create cache directory");

    let git_dir = cache_path.join(".git");
    fs::create_dir_all(&git_dir.join("objects")).expect("failed to create .git/objects");
    fs::create_dir_all(&git_dir.join("refs")).expect("failed to create .git/refs");

    fs::write(git_dir.join("HEAD"), "ref: refs/heads/main\n").expect("failed to write HEAD");
    let metadata_content = r#"
id: cached-template
name: Cached Template
description: A cached test template
version: 1.0.0
language: rust
tags:
  - cached
"#;
    fs::write(cache_path.join("template.yaml"), metadata_content)
        .expect("failed to write template.yaml");

    let content_dir = cache_path.join("content");
    fs::create_dir_all(&content_dir).expect("failed to create content directory");
    fs::write(content_dir.join("main.rs"), "fn main() {}").expect("failed to write main.rs");

    let _path_resolver = TestPathResolver {
        cache_directory: cache_directory.clone(),
        config_directory: sandbox.join("config"),
    };

    let git_repository = CliGitRepository::new();

    let is_valid = git_repository.is_valid_repo(&cache_path);
    assert!(is_valid.is_ok(), "should be able to check if repo is valid");
    assert!(is_valid.unwrap(), "cached repo should be valid");

    let catalog_source = LocalTemplatesCatalogSource::new(PlaceholderDetector::new());
    let catalog_parser = TemplateCatalogParser::new(
        SerdeYamlParser::new(),
        TestValidator,
        SemverVersionComparator::new(),
    );
    let catalog_resolver = TemplateCatalogSourceResolver::new(catalog_source, catalog_parser);

    let templates_service = TemplatesService::new(
        CachedWarningSynchronizer {
            cache_path: cache_path.clone(),
        },
        catalog_resolver,
        TestConfigStore {
            sources: vec![TemplateSource::new(
                "test-source".to_owned(),
                "https://example.com/offline.git".to_owned(),
            )],
        },
        TestValidator,
        git_repository,
    );

    let (templates, warnings) = templates_service
        .list_templates()
        .expect("cached templates should be listed with warnings");
    assert_eq!(templates.len(), 1, "should list cached template");
    assert!(
        warnings
            .iter()
            .any(|warning| warning.contains("using cached data")),
        "should include stale-data warning"
    );
    let _ = fs::remove_dir_all(sandbox);
}
