use std::path::Path;
use std::process::Command;

use nframework_nfw_application::features::template_management::services::abstraction::git_repository::GitRepository;

/// Shallow clone depth for git operations - clones only the latest commit
const GIT_SHALLOW_CLONE_DEPTH: &str = "1";

#[derive(Debug, Default, Clone, Copy)]
pub struct CliGitRepository;

impl CliGitRepository {
    pub fn new() -> Self {
        Self
    }

    fn run_git_command(args: &[&str], working_dir: Option<&Path>) -> Result<String, String> {
        let mut command = Command::new("git");
        command.args(args);

        if let Some(path) = working_dir {
            command.current_dir(path);
        }

        let output = command
            .output()
            .map_err(|error| format!("failed to execute git command: {error}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
            return Err(if stderr.is_empty() {
                format!("git command failed with status {}", output.status)
            } else {
                stderr
            });
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
    }
}

impl GitRepository for CliGitRepository {
    fn clone(&self, url: &str, destination: &Path) -> Result<(), String> {
        let destination_str = destination
            .to_str()
            .ok_or_else(|| "destination path is not valid UTF-8".to_owned())?;

        Self::run_git_command(
            &[
                "clone",
                "--depth",
                GIT_SHALLOW_CLONE_DEPTH,
                url,
                destination_str,
            ],
            None,
        )
        .map(|_| ())
    }

    fn fetch(&self, repository_path: &Path) -> Result<(), String> {
        Self::run_git_command(
            &["fetch", "--all", "--tags", "--prune"],
            Some(repository_path),
        )
        .map(|_| ())
    }

    fn current_branch(&self, repository_path: &Path) -> Result<String, String> {
        Self::run_git_command(
            &["rev-parse", "--abbrev-ref", "HEAD"],
            Some(repository_path),
        )
    }

    fn is_valid_repo(&self, repository_path: &Path) -> bool {
        Self::run_git_command(
            &["rev-parse", "--is-inside-work-tree"],
            Some(repository_path),
        )
        .map(|output| output == "true")
        .unwrap_or(false)
    }

    fn is_valid_remote_url(&self, url: &str) -> bool {
        Self::run_git_command(&["ls-remote", "--heads", url], None).is_ok()
    }
}
