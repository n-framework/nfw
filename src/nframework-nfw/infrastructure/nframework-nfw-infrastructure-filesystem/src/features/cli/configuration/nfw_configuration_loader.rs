use std::fs;
use std::path::PathBuf;

use nframework_nfw_application::features::cli::configuration::abstraction::nfw_configuration_loader::NfwConfigurationLoader;
use nframework_nfw_application::features::cli::configuration::abstraction::path_resolver::PathResolver;
use nframework_nfw_application::features::cli::configuration::nfw_configuration::NfwConfiguration;
use nframework_nfw_domain::features::template_management::template_source::TemplateSource;

use crate::features::cli::configuration::models::source_config::SourceConfig;
use crate::features::cli::configuration::models::sources_file::SourcesFile;

#[derive(Debug, Clone)]
pub struct NfwFileSystemConfigurationLoader<P>
where
    P: PathResolver,
{
    path_resolver: P,
}

impl<P> NfwFileSystemConfigurationLoader<P>
where
    P: PathResolver,
{
    pub fn new(path_resolver: P) -> Self {
        Self { path_resolver }
    }

    fn resolve_sources_file_path(&self) -> Result<PathBuf, String> {
        Ok(self.path_resolver.config_dir()?.join("sources.yaml"))
    }
}

impl<P> NfwConfigurationLoader for NfwFileSystemConfigurationLoader<P>
where
    P: PathResolver,
{
    fn load_configuration(&self) -> Result<NfwConfiguration, String> {
        let cache_directory = self.path_resolver.cache_dir()?;
        let config_directory = self.path_resolver.config_dir()?;
        let sources_file_path = self.resolve_sources_file_path()?;

        let template_sources = if !sources_file_path.is_file() {
            Vec::new()
        } else {
            let content = fs::read_to_string(&sources_file_path).map_err(|error| {
                format!(
                    "failed to read configuration file '{}': {error}",
                    sources_file_path.display()
                )
            })?;
            let parsed: SourcesFile =
                serde_yaml::from_str(&content).map_err(|error| error.to_string())?;
            parsed
                .sources
                .into_iter()
                .map(|source| TemplateSource::new(source.name, source.url, source.enabled))
                .collect()
        };

        Ok(NfwConfiguration::new(
            template_sources,
            cache_directory,
            config_directory,
        ))
    }

    fn save_configuration(&self, configuration: &NfwConfiguration) -> Result<(), String> {
        let config_directory = self.path_resolver.config_dir()?;
        fs::create_dir_all(&config_directory).map_err(|error| {
            format!(
                "failed to create nfw config directory '{}': {error}",
                config_directory.display()
            )
        })?;

        let sources_file_path = config_directory.join("sources.yaml");
        let sources_file = SourcesFile {
            sources: configuration
                .template_sources
                .iter()
                .map(|source| SourceConfig {
                    name: source.name.clone(),
                    url: source.url.clone(),
                    enabled: source.enabled,
                })
                .collect(),
        };
        let serialized = serde_yaml::to_string(&sources_file).map_err(|error| error.to_string())?;
        fs::write(&sources_file_path, serialized).map_err(|error| {
            format!(
                "failed to write configuration file '{}': {error}",
                sources_file_path.display()
            )
        })
    }
}
