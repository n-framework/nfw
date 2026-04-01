use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use nframework_nfw_application::features::cli::configuration::abstraction::path_resolver::PathResolver;
use nframework_nfw_application::features::template_management::services::abstraction::git_repository::GitRepository;
use nframework_nfw_application::features::template_management::services::abstraction::template_source_synchronizer::TemplateSourceSynchronizer;
use nframework_nfw_domain::features::template_management::template_source::TemplateSource;

#[derive(Debug, Clone)]
pub struct GitTemplateCatalogSource<G, P>
where
    G: GitRepository,
    P: PathResolver,
{
    git_repository: G,
    path_resolver: P,
}

impl<G, P> GitTemplateCatalogSource<G, P>
where
    G: GitRepository,
    P: PathResolver,
{
    pub fn new(git_repository: G, path_resolver: P) -> Self {
        Self {
            git_repository,
            path_resolver,
        }
    }

    fn resolve_cache_root(&self) -> Result<PathBuf, String> {
        let cache_root = self.path_resolver.cache_dir()?.join("templates");
        fs::create_dir_all(&cache_root).map_err(|error| {
            format!(
                "failed to create template cache root '{}': {error}",
                cache_root.display()
            )
        })?;

        Ok(cache_root)
    }

    fn source_cache_path(&self, source: &TemplateSource) -> Result<PathBuf, String> {
        if source.name.trim().is_empty() {
            return Err("template source name cannot be empty".to_owned());
        }

        Ok(self.resolve_cache_root()?.join(&source.name))
    }

    fn refresh_working_tree(&self, cache_path: &Path) -> Result<(), String> {
        let output = Command::new("git")
            .arg("pull")
            .arg("--ff-only")
            .current_dir(cache_path)
            .output()
            .map_err(|error| {
                format!(
                    "failed to execute git pull in cache '{}': {error}",
                    cache_path.display()
                )
            })?;

        if output.status.success() {
            return Ok(());
        }

        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        if stderr.is_empty() {
            Err(format!(
                "git pull failed in cache '{}' with status {}",
                cache_path.display(),
                output.status
            ))
        } else {
            Err(stderr)
        }
    }
}

impl<G, P> TemplateSourceSynchronizer for GitTemplateCatalogSource<G, P>
where
    G: GitRepository,
    P: PathResolver,
{
    fn sync_source(&self, source: &TemplateSource) -> Result<(PathBuf, Option<String>), String> {
        let cache_path = self.source_cache_path(source)?;
        let cache_exists = cache_path.is_dir();

        if cache_exists && !self.git_repository.is_valid_repo(&cache_path) {
            fs::remove_dir_all(&cache_path).map_err(|error| {
                format!(
                    "template source '{}' cache is corrupted and could not be removed ('{}'): {error}",
                    source.name,
                    cache_path.display()
                )
            })?;
        }

        if !cache_path.is_dir() {
            self.git_repository
                .clone(&source.url, &cache_path)
                .map_err(|error| {
                    format!(
                        "failed to clone template source '{}' from '{}': {error}",
                        source.name, source.url
                    )
                })?;
            return Ok((cache_path, None));
        }

        match self.git_repository.fetch(&cache_path) {
            Ok(()) => match self.refresh_working_tree(&cache_path) {
                Ok(()) => Ok((cache_path, None)),
                Err(error) => Ok((
                    cache_path,
                    Some(format!(
                        "could not fast-forward template source '{}'; using existing cache ({error})",
                        source.name
                    )),
                )),
            },
            Err(error) => Ok((
                cache_path,
                Some(format!(
                    "could not refresh remote '{}'; using existing cache ({error})",
                    source.url
                )),
            )),
        }
    }

    fn clear_source_cache(&self, source_name: &str) -> Result<(), String> {
        let cache_root = self.resolve_cache_root()?;
        let source_cache_path = cache_root.join(source_name);
        if !source_cache_path.exists() {
            return Ok(());
        }

        fs::remove_dir_all(&source_cache_path).map_err(|error| {
            format!(
                "failed to remove source cache '{}': {error}",
                source_cache_path.display()
            )
        })
    }
}
