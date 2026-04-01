use std::path::Path;

pub trait GitRepository {
    fn clone(&self, url: &str, destination: &Path) -> Result<(), String>;
    fn fetch(&self, repository_path: &Path) -> Result<(), String>;
    fn current_branch(&self, repository_path: &Path) -> Result<String, String>;
    fn is_valid_repo(&self, repository_path: &Path) -> bool;
    fn is_valid_remote_url(&self, url: &str) -> bool;
}
