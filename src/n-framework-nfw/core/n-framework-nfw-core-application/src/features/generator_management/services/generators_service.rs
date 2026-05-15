use n_framework_nfw_core_domain::features::generator_management::generator_catalog::GeneratorCatalog;
use n_framework_nfw_core_domain::features::generator_management::generator_source::GeneratorSource;

use crate::features::cli::configuration::abstractions::config_store::ConfigStore;
use crate::features::generator_management::constants::source;
use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use crate::features::generator_management::models::listed_generator::ListedGenerator;
use crate::features::generator_management::services::abstractions::generator_catalog_discovery_service::GeneratorCatalogDiscoveryService;
use crate::features::generator_management::services::abstractions::generator_listing_service::GeneratorListingService;
use crate::features::generator_management::services::abstractions::git_repository::GitRepository;
use crate::features::generator_management::services::abstractions::generator_catalog_source::GeneratorCatalogSource;
use crate::features::generator_management::services::abstractions::generator_source_synchronizer::GeneratorSourceSynchronizer;
use crate::features::generator_management::services::abstractions::validator::Validator;
use crate::features::generator_management::services::abstractions::yaml_parser::YamlParser;
use crate::features::generator_management::services::generator_catalog_source_resolver::GeneratorCatalogSourceResolver;
use crate::features::versioning::abstractions::version_comparator::VersionComparator;

#[derive(Debug, Clone)]
pub struct GeneratorsService<R, S, Y, V, C, CS, G>
where
    R: GeneratorSourceSynchronizer,
    S: GeneratorCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
    G: GitRepository,
{
    source_synchronizer: R,
    catalog_resolver: GeneratorCatalogSourceResolver<S, Y, V, C>,
    config_store: CS,
    validator: V,
    git_repository: G,
}

impl<R, S, Y, V, C, CS, G> GeneratorsService<R, S, Y, V, C, CS, G>
where
    R: GeneratorSourceSynchronizer,
    S: GeneratorCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
    G: GitRepository,
{
    pub fn new(
        source_synchronizer: R,
        catalog_resolver: GeneratorCatalogSourceResolver<S, Y, V, C>,
        config_store: CS,
        validator: V,
        git_repository: G,
    ) -> Self {
        Self {
            source_synchronizer,
            catalog_resolver,
            config_store,
            validator,
            git_repository,
        }
    }

    pub fn ensure_default_source_registered(&self) -> Result<(), GeneratorsServiceError> {
        let mut sources = self
            .config_store
            .load_sources()
            .map_err(GeneratorsServiceError::LoadSourcesFailed)?;

        if sources
            .iter()
            .any(|source| source.name == source::OFFICIAL_NAME)
        {
            return Ok(());
        }

        sources.push(GeneratorSource::new(
            source::OFFICIAL_NAME.to_owned(),
            source::OFFICIAL_URL.to_owned(),
        ));
        self.config_store
            .save_sources(&sources)
            .map_err(GeneratorsServiceError::SaveSourcesFailed)
    }

    pub fn add_source(&self, name: &str, url: &str) -> Result<(), GeneratorsServiceError> {
        let normalized_name = name.trim();
        let normalized_url = url.trim();

        if !self.validator.is_kebab_case(normalized_name) {
            return Err(GeneratorsServiceError::InvalidSourceName(name.to_owned()));
        }

        if normalized_url.is_empty() || !self.git_repository.is_valid_git_url_format(normalized_url)
        {
            return Err(GeneratorsServiceError::InvalidSourceUrl(url.to_owned()));
        }

        let mut sources = self
            .config_store
            .load_sources()
            .map_err(GeneratorsServiceError::LoadSourcesFailed)?;

        if sources.iter().any(|source| source.name == normalized_name) {
            return Err(GeneratorsServiceError::SourceAlreadyExists(
                normalized_name.to_owned(),
            ));
        }

        sources.push(GeneratorSource::new(
            normalized_name.to_owned(),
            normalized_url.to_owned(),
        ));
        sources.sort_by(|left, right| left.name.cmp(&right.name));
        self.config_store
            .save_sources(&sources)
            .map_err(GeneratorsServiceError::SaveSourcesFailed)
    }

    pub fn remove_source(&self, name: &str) -> Result<(), GeneratorsServiceError> {
        let normalized_name = name.trim();
        let mut sources = self
            .config_store
            .load_sources()
            .map_err(GeneratorsServiceError::LoadSourcesFailed)?;

        let source_count_before = sources.len();
        sources.retain(|source| source.name != normalized_name);
        if sources.len() == source_count_before {
            return Err(GeneratorsServiceError::SourceNotFound(
                normalized_name.to_owned(),
            ));
        }

        self.source_synchronizer
            .clear_source_cache(normalized_name)
            .map_err(GeneratorsServiceError::CacheCleanupFailed)?;
        self.config_store
            .save_sources(&sources)
            .map_err(GeneratorsServiceError::SaveSourcesFailed)
    }

    pub fn discover_catalogs(
        &self,
    ) -> Result<(Vec<GeneratorCatalog>, Vec<String>), GeneratorsServiceError> {
        self.discover_catalogs_internal()
    }

    pub fn list_generators(
        &self,
    ) -> Result<(Vec<ListedGenerator>, Vec<String>), GeneratorsServiceError> {
        self.list_generators_internal()
    }

    fn discover_catalogs_internal(
        &self,
    ) -> Result<(Vec<GeneratorCatalog>, Vec<String>), GeneratorsServiceError> {
        let sources = self
            .config_store
            .load_sources()
            .map_err(GeneratorsServiceError::LoadSourcesFailed)?;

        let mut catalogs = Vec::new();
        let mut warnings = Vec::new();
        let mut transient_failures = Vec::new();

        for source in sources.into_iter().filter(|source| source.enabled) {
            let sync_result = self.source_synchronizer.sync_source(&source);
            let (cache_path, sync_warning) = match sync_result {
                Ok(value) => value,
                Err(reason) => {
                    let is_transient = reason.contains("could not refresh remote")
                        || reason.contains("could not fast-forward")
                        || reason.contains("network")
                        || reason.contains("connection")
                        || reason.contains("timeout");

                    if is_transient {
                        transient_failures.push((source.name.clone(), reason));
                        continue;
                    } else {
                        return Err(GeneratorsServiceError::SourceSyncFailed {
                            source: source.name.clone(),
                            reason,
                        });
                    }
                }
            };

            if let Some(sync_warning) = sync_warning {
                warnings.push(format!(
                    "generator source '{}' using cached data: {sync_warning}",
                    source.name
                ));
            }

            match self.catalog_resolver.resolve(&source.name, &cache_path) {
                Ok(catalog) => {
                    if catalog.is_empty() {
                        warnings.push(format!(
                            "generator source '{}' contains no valid generators",
                            source.name
                        ));
                    }
                    catalogs.push(catalog);
                }
                Err(error) => {
                    let error_string = error.to_string();
                    let is_critical = error_string.contains("cache is corrupted")
                        || error_string.contains("permission denied")
                        || error_string.contains("not a directory");

                    if is_critical {
                        return Err(GeneratorsServiceError::SourceDiscoveryFailed {
                            source: source.name.clone(),
                            reason: error_string,
                        });
                    } else {
                        warnings.push(format!(
                            "generator source '{}' discovery warning: {error}",
                            source.name
                        ));
                    }
                }
            }
        }

        if !transient_failures.is_empty() {
            let failed_sources: Vec<&str> = transient_failures
                .iter()
                .map(|(name, _)| name.as_str())
                .collect();
            warnings.push(format!(
                "Could not reach {} generator source(s): {}. Using cached data if available.",
                failed_sources.len(),
                failed_sources.join(", ")
            ));
        }

        catalogs.sort_by(|left, right| left.source_name.cmp(&right.source_name));

        Ok((catalogs, warnings))
    }

    fn list_generators_internal(
        &self,
    ) -> Result<(Vec<ListedGenerator>, Vec<String>), GeneratorsServiceError> {
        let (catalogs, warnings) = self.discover_catalogs_internal()?;
        let mut generators = Vec::new();

        for catalog in catalogs {
            for descriptor in catalog.generators {
                generators.push(ListedGenerator {
                    id: descriptor.metadata.id,
                    name: descriptor.metadata.name,
                    description: descriptor.metadata.description,
                    version: descriptor.metadata.version,
                    language: descriptor.metadata.language,
                    source_name: catalog.source_name.clone(),
                });
            }
        }

        generators.sort_by(|left, right| {
            left.id
                .cmp(&right.id)
                .then(left.source_name.cmp(&right.source_name))
        });

        Ok((generators, warnings))
    }
}

impl<R, S, Y, V, C, CS, G> GeneratorCatalogDiscoveryService
    for GeneratorsService<R, S, Y, V, C, CS, G>
where
    R: GeneratorSourceSynchronizer,
    S: GeneratorCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
    G: GitRepository,
{
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<GeneratorCatalog>, Vec<String>), GeneratorsServiceError> {
        self.discover_catalogs_internal()
    }
}

impl<R, S, Y, V, C, CS, G> GeneratorListingService for GeneratorsService<R, S, Y, V, C, CS, G>
where
    R: GeneratorSourceSynchronizer,
    S: GeneratorCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
    G: GitRepository,
{
    fn list_generators(
        &self,
    ) -> Result<(Vec<ListedGenerator>, Vec<String>), GeneratorsServiceError> {
        self.list_generators_internal()
    }
}

#[cfg(test)]
#[path = "generators_service.tests.rs"]
mod tests;
