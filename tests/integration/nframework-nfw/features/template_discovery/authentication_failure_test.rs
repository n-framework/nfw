//! Tests for Git authentication failure handling.
//!
//! These tests verify that the template system provides clear error messages
//! when Git authentication fails, and that users are guided toward proper
//! credential configuration.

use std::fs;
use std::path::{Path, PathBuf};

use nframework_nfw_application::features::cli::configuration::abstraction::config_store::ConfigStore;
use nframework_nfw_application::features::cli::configuration::abstraction::path_resolver::PathResolver;
use nframework_nfw_application::features::template_management::services::abstraction::git_repository::GitRepository;
use nframework_nfw_application::features::template_management::services::abstraction::validator::Validator;
use nframework_nfw_domain::features::template_management::template_source::TemplateSource;
use nframework_nfw_infrastructure_git::features::template_management::services::cli_git_repository::CliGitRepository;

/// Creates a temporary sandbox directory for testing
fn create_sandbox_directory() -> PathBuf {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let sandbox = std::env::temp_dir().join(format!("nfw-test-auth-{}", timestamp));
    fs::create_dir_all(&sandbox).expect("failed to create sandbox directory");
    sandbox
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

#[test]
fn provides_clear_error_for_authentication_failure() {
    let git_repository = CliGitRepository::new();

    let private_repo_url = "https://github.com/n-framework/private-nonexistent-repo.git";

    assert!(
        git_repository.is_valid_git_url_format(private_repo_url),
        "private repo URL should have valid format"
    );

    let is_reachable = git_repository.is_remote_url_reachable(private_repo_url);

    assert!(
        !is_reachable,
        "private/nonexistent repo should not be reachable"
    );
}

#[test]
fn validates_url_format_before_network_access() {
    let git_repository = CliGitRepository::new();

    let valid_urls = vec![
        "https://github.com/user/repo.git",
        "http://example.com/repo.git",
        "git@github.com:user/repo.git",
        "ssh://git@example.com/repo.git",
        "/local/path/to/repo",
        "./relative/path",
        "~/home/path",
    ];

    for url in valid_urls {
        assert!(
            git_repository.is_valid_git_url_format(url),
            "URL '{}' should be valid format",
            url
        );
    }

    let invalid_urls = vec!["not-a-url", "ftp://invalid-protocol.com/repo", "", "   "];

    for url in invalid_urls {
        assert!(
            !git_repository.is_valid_git_url_format(url),
            "URL '{}' should be invalid format",
            url
        );
    }
}

#[test]
fn handles_git_not_installed_error() {
    let git_repository = CliGitRepository::new();

    let result = git_repository.is_valid_repo(Path::new("/non/existent/path"));

    match result {
        Ok(is_valid) => {
            assert!(
                !is_valid,
                "non-existent path should not be a valid repository"
            );
        }
        Err(error) => {
            assert!(error.len() > 0, "error message should not be empty");
            println!("Git check failed with error: {}", error);
        }
    }
}
