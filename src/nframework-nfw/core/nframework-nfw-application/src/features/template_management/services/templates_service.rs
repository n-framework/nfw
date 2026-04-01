use nframework_nfw_domain::features::template_management::template_catalog::TemplateCatalog;

use crate::features::cli::configuration::abstraction::config_store::ConfigStore;
use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::models::listed_template::ListedTemplate;
use crate::features::template_management::services::abstraction::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::abstraction::template_listing_service::TemplateListingService;
use crate::features::template_management::services::abstraction::template_catalog_source::TemplateCatalogSource;
use crate::features::template_management::services::abstraction::template_source_synchronizer::TemplateSourceSynchronizer;
use crate::features::template_management::services::abstraction::validator::Validator;
use crate::features::template_management::services::abstraction::yaml_parser::YamlParser;
use crate::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use crate::features::versioning::abstraction::version_comparator::VersionComparator;

#[derive(Debug, Clone)]
pub struct TemplatesService<R, S, Y, V, C, CS>
where
    R: TemplateSourceSynchronizer,
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
{
    source_synchronizer: R,
    catalog_resolver: TemplateCatalogSourceResolver<S, Y, V, C>,
    config_store: CS,
}

impl<R, S, Y, V, C, CS> TemplatesService<R, S, Y, V, C, CS>
where
    R: TemplateSourceSynchronizer,
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
{
    pub fn new(
        source_synchronizer: R,
        catalog_resolver: TemplateCatalogSourceResolver<S, Y, V, C>,
        config_store: CS,
    ) -> Self {
        Self {
            source_synchronizer,
            catalog_resolver,
            config_store,
        }
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

impl<R, S, Y, V, C, CS> TemplateCatalogDiscoveryService for TemplatesService<R, S, Y, V, C, CS>
where
    R: TemplateSourceSynchronizer,
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
{
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<TemplateCatalog>, Vec<String>), TemplatesServiceError> {
        self.discover_catalogs_internal()
    }
}

impl<R, S, Y, V, C, CS> TemplateListingService for TemplatesService<R, S, Y, V, C, CS>
where
    R: TemplateSourceSynchronizer,
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
    CS: ConfigStore,
{
    fn list_templates(&self) -> Result<(Vec<ListedTemplate>, Vec<String>), TemplatesServiceError> {
        self.list_templates_internal()
    }
}
