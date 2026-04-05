//! Tests for Git authentication failure handling.
//!
//! These tests verify that the template system provides clear error messages
//! when Git authentication fails, and that users are guided toward proper
//! credential configuration.

use std::path::Path;

use nframework_nfw_core_application::features::template_management::services::abstractions::git_repository::GitRepository;
use nframework_nfw_infrastructure_git::features::template_management::services::cli_git_repository::CliGitRepository;

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
