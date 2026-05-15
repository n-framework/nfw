use std::path::Path;

pub trait GitRepository {
    fn clone(&self, url: &str, destination: &Path) -> Result<(), String>;
    fn fetch(&self, repository_path: &Path) -> Result<(), String>;
    fn pull(&self, repository_path: &Path) -> Result<(), String>;
    fn current_branch(&self, repository_path: &Path) -> Result<String, String>;

    /// Checks if a path contains a valid git repository.
    /// Returns `Ok(true)` if valid, `Ok(false)` if not a git repo, or `Err` if the check failed.
    fn is_valid_repo(&self, repository_path: &Path) -> Result<bool, String>;

    /// Validates that a URL has a valid Git URL format (does NOT perform network I/O)
    fn is_valid_git_url_format(&self, url: &str) -> bool;

    /// Validates that a remote URL is reachable (performs network I/O)
    fn is_remote_url_reachable(&self, url: &str) -> bool;
}
