//! Tests for corrupted cache recovery in generator synchronization.
//!
//! These tests verify that the generator system can properly detect and recover
//! from various cache corruption scenarios, including:
//! - Partial git clones (interrupted operations)
//! - Missing .git directories
//! - Corrupted git objects
//! - Invalid repository state

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use n_framework_nfw_core_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use n_framework_nfw_core_application::features::cli::configuration::abstractions::path_resolver::PathResolver;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::validator::Validator;
use n_framework_nfw_core_application::features::generator_management::services::generator_catalog_parser::GeneratorCatalogParser;
use n_framework_nfw_core_application::features::generator_management::services::generator_catalog_source_resolver::GeneratorCatalogSourceResolver;
use n_framework_nfw_core_application::features::generator_management::services::generators_service::GeneratorsService;
use n_framework_nfw_core_domain::features::generator_management::generator_source::GeneratorSource;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::local_generators_catalog_source::LocalGeneratorsCatalogSource;
use n_framework_nfw_infrastructure_git::features::generator_management::services::cli_git_repository::CliGitRepository;
use n_framework_nfw_infrastructure_git::features::generator_management::services::git_generator_catalog_source::GitGeneratorCatalogSource;
use n_framework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use n_framework_nfw_infrastructure_yaml::features::generator_management::services::serde_yaml_parser::SerdeYamlParser;

/// Creates a temporary sandbox directory for testing
fn create_sandbox_directory() -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let sandbox = std::env::temp_dir().join(format!("nfw-test-corrupted-cache-{}", timestamp));
    fs::create_dir_all(&sandbox).expect("failed to create sandbox directory");
    sandbox
}

/// Creates a bare git repository that can be used as a remote
fn create_bare_repository(path: &Path) {
    fs::create_dir_all(path).expect("failed to create repository directory");
    Command::new("git")
        .arg("init")
        .arg("--bare")
        .arg(path)
        .output()
        .expect("failed to initialize bare repository");
}

/// Creates a generator repository with a sample generator
fn create_generator_repository(path: &Path, generator_id: &str) {
    fs::create_dir_all(path).expect("failed to create generator directory");

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
name: {} Generator
description: A test generator
version: 1.0.0
language: rust
tags:
  - test
"#,
        generator_id, generator_id
    );

    fs::write(path.join("nfw.generator.yaml"), metadata_content)
        .expect("failed to write generator.yaml");

    fs::write(
        content_dir.join("main.rs"),
        format!(
            "fn main() {{ println!(\"Hello from {}!\"); }}",
            generator_id
        ),
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
    sources: Vec<GeneratorSource>,
}

impl ConfigStore for TestConfigStore {
    fn load_sources(&self) -> Result<Vec<GeneratorSource>, String> {
        Ok(self.sources.clone())
    }

    fn save_sources(&self, _sources: &[GeneratorSource]) -> Result<(), String> {
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

#[test]
fn detects_and_recovers_from_corrupted_git_cache() {
    let sandbox = create_sandbox_directory();
    let remote_repository = sandbox.join("remote-catalog.git");
    let seed_repository = sandbox.join("seed-repository");
    let cache_directory = sandbox.join("cache");
    let config_directory = sandbox.join("config");

    fs::create_dir_all(&cache_directory).expect("failed to create cache directory");
    fs::create_dir_all(&config_directory).expect("failed to create config directory");

    create_bare_repository(&remote_repository);
    create_generator_repository(&seed_repository, "test-generator");

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
    let source_synchronizer = GitGeneratorCatalogSource::new(git_repository, path_resolver);
    let catalog_source = LocalGeneratorsCatalogSource::new();
    let catalog_parser = GeneratorCatalogParser::new(
        SerdeYamlParser::new(),
        TestValidator,
        SemverVersionComparator::new(),
    );
    let catalog_resolver = GeneratorCatalogSourceResolver::new(catalog_source, catalog_parser);

    let config_store = TestConfigStore {
        sources: vec![GeneratorSource::new(
            "test-source".to_owned(),
            remote_repository.to_str().unwrap().to_owned(),
        )],
    };

    let generators_service = GeneratorsService::new(
        source_synchronizer,
        catalog_resolver,
        config_store,
        TestValidator,
        CliGitRepository::new(),
    );

    let (generators, warnings) = generators_service
        .list_generators()
        .expect("first sync should succeed");

    assert!(
        !generators.is_empty(),
        "should discover generators on first sync"
    );
    assert!(
        warnings.is_empty(),
        "first sync should not produce warnings"
    );

    let cache_path = cache_directory.join("generators/test-source");
    let git_dir = cache_path.join(".git");
    assert!(git_dir.exists(), ".git directory should exist after sync");

    fs::remove_dir_all(&git_dir).expect("failed to corrupt cache");
    let (generators_after_recovery, _warnings_after_recovery) = generators_service
        .list_generators()
        .expect("sync after corruption should succeed");

    assert!(
        !generators_after_recovery.is_empty(),
        "should discover generators after recovery from corruption"
    );
    let _ = fs::remove_dir_all(sandbox);
}

#[test]
fn recovers_from_partial_clone_failure() {
    let sandbox = create_sandbox_directory();
    let remote_repository = sandbox.join("remote-catalog.git");
    let seed_repository = sandbox.join("seed-repository");
    let cache_directory = sandbox.join("cache");
    let config_directory = sandbox.join("config");

    fs::create_dir_all(&cache_directory).expect("failed to create cache directory");
    fs::create_dir_all(&config_directory).expect("failed to create config directory");

    create_bare_repository(&remote_repository);
    create_generator_repository(&seed_repository, "partial-clone-test");

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
    let source_synchronizer = GitGeneratorCatalogSource::new(git_repository, path_resolver);
    let catalog_source = LocalGeneratorsCatalogSource::new();
    let catalog_parser = GeneratorCatalogParser::new(
        SerdeYamlParser::new(),
        TestValidator,
        SemverVersionComparator::new(),
    );
    let catalog_resolver = GeneratorCatalogSourceResolver::new(catalog_source, catalog_parser);

    let config_store = TestConfigStore {
        sources: vec![GeneratorSource::new(
            "test-source".to_owned(),
            remote_repository.to_str().unwrap().to_owned(),
        )],
    };

    let generators_service = GeneratorsService::new(
        source_synchronizer,
        catalog_resolver,
        config_store,
        TestValidator,
        CliGitRepository::new(),
    );

    // Create a partial clone by manually creating the cache directory
    let cache_path = cache_directory.join("generators/test-source");
    fs::create_dir_all(&cache_path).expect("failed to create cache directory");

    assert!(!cache_path.join(".git").exists(), ".git should not exist");

    let (generators, _warnings) = generators_service
        .list_generators()
        .expect("sync should recover from partial clone");

    assert!(
        !generators.is_empty(),
        "should discover generators after recovery"
    );
    let _ = fs::remove_dir_all(sandbox);
}

#[test]
fn handles_missing_cache_directory_gracefully() {
    let sandbox = create_sandbox_directory();
    let remote_repository = sandbox.join("remote-catalog.git");
    let seed_repository = sandbox.join("seed-repository");
    let cache_directory = sandbox.join("cache");
    let config_directory = sandbox.join("config");

    fs::create_dir_all(&config_directory).expect("failed to create config directory");
    create_bare_repository(&remote_repository);
    create_generator_repository(&seed_repository, "missing-dir-test");

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
    let source_synchronizer = GitGeneratorCatalogSource::new(git_repository, path_resolver);
    let catalog_source = LocalGeneratorsCatalogSource::new();
    let catalog_parser = GeneratorCatalogParser::new(
        SerdeYamlParser::new(),
        TestValidator,
        SemverVersionComparator::new(),
    );
    let catalog_resolver = GeneratorCatalogSourceResolver::new(catalog_source, catalog_parser);

    let config_store = TestConfigStore {
        sources: vec![GeneratorSource::new(
            "test-source".to_owned(),
            remote_repository.to_str().unwrap().to_owned(),
        )],
    };

    let generators_service = GeneratorsService::new(
        source_synchronizer,
        catalog_resolver,
        config_store,
        TestValidator,
        CliGitRepository::new(),
    );

    let (generators, _warnings) = generators_service
        .list_generators()
        .expect("sync should create cache directory as needed");

    assert!(!generators.is_empty(), "should discover generators");
    assert!(
        cache_directory.exists(),
        "cache directory should be created"
    );
    let _ = fs::remove_dir_all(sandbox);
}
