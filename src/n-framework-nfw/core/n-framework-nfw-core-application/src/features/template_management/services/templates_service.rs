use n_framework_nfw_core_domain::features::template_management::template_catalog::TemplateCatalog;
use n_framework_nfw_core_domain::features::template_management::template_source::TemplateSource;

use crate::features::cli::configuration::abstractions::config_store::ConfigStore;
use crate::features::template_management::constants::source;
use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::models::listed_template::ListedTemplate;
use crate::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::abstractions::template_listing_service::TemplateListingService;
use crate::features::template_management::services::abstractions::git_repository::GitRepository;
use crate::features::template_management::services::abstractions::template_catalog_source::TemplateCatalogSource;
use crate::features::template_management::services::abstractions::template_source_synchronizer::TemplateSourceSynchronizer;
use crate::features::template_management::services::abstractions::validator::Validator;
use crate::features::template_management::services::abstractions::yaml_parser::YamlParser;
use crate::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use crate::features::versioning::abstractions::version_comparator::VersionComparator;

#[derive(Debug, Clone)]
pub struct TemplatesService<R, S, Y, V, C, CS, G>
where
    R: TemplateSourceSynchronizer,
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
    G: GitRepository,
{
    source_synchronizer: R,
    catalog_resolver: TemplateCatalogSourceResolver<S, Y, V, C>,
    config_store: CS,
    validator: V,
    git_repository: G,
}

impl<R, S, Y, V, C, CS, G> TemplatesService<R, S, Y, V, C, CS, G>
where
    R: TemplateSourceSynchronizer,
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
    G: GitRepository,
{
    pub fn new(
        source_synchronizer: R,
        catalog_resolver: TemplateCatalogSourceResolver<S, Y, V, C>,
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

    pub fn ensure_default_source_registered(&self) -> Result<(), TemplatesServiceError> {
        let mut sources = self
            .config_store
            .load_sources()
            .map_err(TemplatesServiceError::LoadSourcesFailed)?;

        if sources
            .iter()
            .any(|source| source.name == source::OFFICIAL_NAME)
        {
            return Ok(());
        }

        sources.push(TemplateSource::new(
            source::OFFICIAL_NAME.to_owned(),
            source::OFFICIAL_URL.to_owned(),
        ));
        self.config_store
            .save_sources(&sources)
            .map_err(TemplatesServiceError::SaveSourcesFailed)
    }

    pub fn add_source(&self, name: &str, url: &str) -> Result<(), TemplatesServiceError> {
        let normalized_name = name.trim();
        let normalized_url = url.trim();

        if !self.validator.is_kebab_case(normalized_name) {
            return Err(TemplatesServiceError::InvalidSourceName(name.to_owned()));
        }

        if normalized_url.is_empty() || !self.git_repository.is_valid_git_url_format(normalized_url)
        {
            return Err(TemplatesServiceError::InvalidSourceUrl(url.to_owned()));
        }

        let mut sources = self
            .config_store
            .load_sources()
            .map_err(TemplatesServiceError::LoadSourcesFailed)?;

        if sources.iter().any(|source| source.name == normalized_name) {
            return Err(TemplatesServiceError::SourceAlreadyExists(
                normalized_name.to_owned(),
            ));
        }

        sources.push(TemplateSource::new(
            normalized_name.to_owned(),
            normalized_url.to_owned(),
        ));
        sources.sort_by(|left, right| left.name.cmp(&right.name));
        self.config_store
            .save_sources(&sources)
            .map_err(TemplatesServiceError::SaveSourcesFailed)
    }

    pub fn remove_source(&self, name: &str) -> Result<(), TemplatesServiceError> {
        let normalized_name = name.trim();
        let mut sources = self
            .config_store
            .load_sources()
            .map_err(TemplatesServiceError::LoadSourcesFailed)?;

        let source_count_before = sources.len();
        sources.retain(|source| source.name != normalized_name);
        if sources.len() == source_count_before {
            return Err(TemplatesServiceError::SourceNotFound(
                normalized_name.to_owned(),
            ));
        }

        self.source_synchronizer
            .clear_source_cache(normalized_name)
            .map_err(TemplatesServiceError::CacheCleanupFailed)?;
        self.config_store
            .save_sources(&sources)
            .map_err(TemplatesServiceError::SaveSourcesFailed)
    }

    pub fn discover_catalogs(
        &self,
    ) -> Result<(Vec<TemplateCatalog>, Vec<String>), TemplatesServiceError> {
        self.discover_catalogs_internal()
    }

    pub fn list_templates(
        &self,
    ) -> Result<(Vec<ListedTemplate>, Vec<String>), TemplatesServiceError> {
        self.list_templates_internal()
    }

    fn discover_catalogs_internal(
        &self,
    ) -> Result<(Vec<TemplateCatalog>, Vec<String>), TemplatesServiceError> {
        let sources = self
            .config_store
            .load_sources()
            .map_err(TemplatesServiceError::LoadSourcesFailed)?;

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
                        return Err(TemplatesServiceError::SourceSyncFailed {
                            source: source.name.clone(),
                            reason,
                        });
                    }
                }
            };

            if let Some(sync_warning) = sync_warning {
                warnings.push(format!(
                    "template source '{}' using cached data: {sync_warning}",
                    source.name
                ));
            }

            match self.catalog_resolver.resolve(&source.name, &cache_path) {
                Ok(catalog) => {
                    if catalog.is_empty() {
                        warnings.push(format!(
                            "template source '{}' contains no valid templates",
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
                        return Err(TemplatesServiceError::SourceDiscoveryFailed {
                            source: source.name.clone(),
                            reason: error_string,
                        });
                    } else {
                        warnings.push(format!(
                            "template source '{}' discovery warning: {error}",
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
                "Could not reach {} template source(s): {}. Using cached data if available.",
                failed_sources.len(),
                failed_sources.join(", ")
            ));
        }

        catalogs.sort_by(|left, right| left.source_name.cmp(&right.source_name));

        Ok((catalogs, warnings))
    }

    fn list_templates_internal(
        &self,
    ) -> Result<(Vec<ListedTemplate>, Vec<String>), TemplatesServiceError> {
        let (catalogs, warnings) = self.discover_catalogs_internal()?;
        let mut templates = Vec::new();

        for catalog in catalogs {
            for descriptor in catalog.templates {
                templates.push(ListedTemplate {
                    id: descriptor.metadata.id,
                    name: descriptor.metadata.name,
                    description: descriptor.metadata.description,
                    version: descriptor.metadata.version,
                    language: descriptor.metadata.language,
                    source_name: catalog.source_name.clone(),
                });
            }
        }

        templates.sort_by(|left, right| {
            left.id
                .cmp(&right.id)
                .then(left.source_name.cmp(&right.source_name))
        });

        Ok((templates, warnings))
    }
}

impl<R, S, Y, V, C, CS, G> TemplateCatalogDiscoveryService
    for TemplatesService<R, S, Y, V, C, CS, G>
where
    R: TemplateSourceSynchronizer,
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
    G: GitRepository,
{
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<TemplateCatalog>, Vec<String>), TemplatesServiceError> {
        self.discover_catalogs_internal()
    }
}

impl<R, S, Y, V, C, CS, G> TemplateListingService for TemplatesService<R, S, Y, V, C, CS, G>
where
    R: TemplateSourceSynchronizer,
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
    G: GitRepository,
{
    fn list_templates(&self) -> Result<(Vec<ListedTemplate>, Vec<String>), TemplatesServiceError> {
        self.list_templates_internal()
    }
}
