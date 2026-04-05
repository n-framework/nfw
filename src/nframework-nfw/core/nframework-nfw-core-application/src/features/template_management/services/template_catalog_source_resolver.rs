use std::path::Path;

use nframework_nfw_core_domain::features::template_management::template_catalog::TemplateCatalog;
use nframework_nfw_core_domain::features::template_management::template_descriptor::TemplateDescriptor;

use crate::features::template_management::models::errors::TemplateCatalogSourceResolverError;
use crate::features::template_management::services::abstractions::template_catalog_source::TemplateCatalogSource;
use crate::features::template_management::services::abstractions::validator::Validator;
use crate::features::template_management::services::abstractions::yaml_parser::YamlParser;
use crate::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use crate::features::versioning::abstractions::version_comparator::VersionComparator;

#[derive(Debug, Clone)]
pub struct TemplateCatalogSourceResolver<S, Y, V, C>
where
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
{
    source: S,
    metadata_parser: TemplateCatalogParser<Y, V, C>,
}

impl<S, Y, V, C> TemplateCatalogSourceResolver<S, Y, V, C>
where
    S: TemplateCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
{
    pub fn new(source: S, metadata_parser: TemplateCatalogParser<Y, V, C>) -> Self {
        Self {
            source,
            metadata_parser,
        }
    }

    pub fn resolve(
        &self,
        source_name: &str,
        source_root: &Path,
    ) -> Result<TemplateCatalog, TemplateCatalogSourceResolverError> {
        let template_directories = self
            .source
            .discover_template_directories(source_root)
            .map_err(
                |reason| TemplateCatalogSourceResolverError::SourceScanFailed {
                    source_name: source_name.to_owned(),
                    reason,
                },
            )?;

        let mut templates = Vec::with_capacity(template_directories.len());

        for template_directory in template_directories {
            let metadata_file = template_directory.join("template.yaml");
            let metadata_content = self
                .source
                .read_template_metadata(&template_directory)
                .map_err(
                    |reason| TemplateCatalogSourceResolverError::MetadataReadFailed {
                        template_path: metadata_file.clone(),
                        reason,
                    },
                )?;

            let metadata = self
                .metadata_parser
                .parse_template_metadata(&metadata_content)
                .map_err(
                    |reason| TemplateCatalogSourceResolverError::InvalidTemplateMetadata {
                        template_path: metadata_file,
                        reason: reason.to_string(),
                    },
                )?;

            templates.push(TemplateDescriptor::new(metadata, template_directory));
        }

        templates.sort_by(|left, right| left.metadata.id.cmp(&right.metadata.id));

        Ok(TemplateCatalog::new(source_name.to_owned(), templates))
    }
}
