use nframework_nfw_domain::features::template_management::template_catalog::TemplateCatalog;
use nframework_nfw_domain::features::template_management::template_source::TemplateSource;

use crate::features::cli::configuration::abstraction::config_store::ConfigStore;
use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::models::listed_template::ListedTemplate;
use crate::features::template_management::services::abstraction::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::abstraction::template_listing_service::TemplateListingService;
use crate::features::template_management::services::abstraction::git_repository::GitRepository;
use crate::features::template_management::services::abstraction::template_catalog_source::TemplateCatalogSource;
use crate::features::template_management::services::abstraction::template_source_synchronizer::TemplateSourceSynchronizer;
use crate::features::template_management::services::abstraction::validator::Validator;
use crate::features::template_management::services::abstraction::yaml_parser::YamlParser;
use crate::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use crate::features::versioning::abstraction::version_comparator::VersionComparator;

const OFFICIAL_SOURCE_NAME: &str = "official";
const OFFICIAL_SOURCE_URL: &str = "https://github.com/n-framework/nfw-templates";

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
            .any(|source| source.name == OFFICIAL_SOURCE_NAME)
        {
            return Ok(());
        }

        sources.push(TemplateSource::new(
            OFFICIAL_SOURCE_NAME.to_owned(),
            OFFICIAL_SOURCE_URL.to_owned(),
            true,
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

        if normalized_url.is_empty() || !self.git_repository.is_valid_remote_url(normalized_url) {
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
            true,
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

        for source in sources.into_iter().filter(|source| source.enabled) {
            let sync_result = self.source_synchronizer.sync_source(&source);
            let (cache_path, sync_warning) = match sync_result {
                Ok(value) => value,
                Err(reason) => {
                    warnings.push(format!(
                        "template source '{}' is unreachable: {reason}",
                        source.name
                    ));
                    continue;
                }
            };

            if let Some(sync_warning) = sync_warning {
                warnings.push(format!(
                    "template source '{}' fallback to cache: {sync_warning}",
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
                Err(error) => warnings.push(format!(
                    "template source '{}' discovery warning: {error}",
                    source.name
                )),
            }
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
