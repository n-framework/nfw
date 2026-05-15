use std::path::Path;

use n_framework_nfw_core_domain::features::generator_management::generator_catalog::GeneratorCatalog;
use n_framework_nfw_core_domain::features::generator_management::generator_descriptor::GeneratorDescriptor;

use crate::features::generator_management::models::errors::GeneratorCatalogSourceResolverError;
use crate::features::generator_management::services::abstractions::generator_catalog_source::GeneratorCatalogSource;
use crate::features::generator_management::services::abstractions::validator::Validator;
use crate::features::generator_management::services::abstractions::yaml_parser::YamlParser;
use crate::features::generator_management::services::generator_catalog_parser::GeneratorCatalogParser;
use crate::features::versioning::abstractions::version_comparator::VersionComparator;

#[derive(Debug, Clone)]
pub struct GeneratorCatalogSourceResolver<S, Y, V, C>
where
    S: GeneratorCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
{
    source: S,
    metadata_parser: GeneratorCatalogParser<Y, V, C>,
}

impl<S, Y, V, C> GeneratorCatalogSourceResolver<S, Y, V, C>
where
    S: GeneratorCatalogSource,
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
{
    pub fn new(source: S, metadata_parser: GeneratorCatalogParser<Y, V, C>) -> Self {
        Self {
            source,
            metadata_parser,
        }
    }

    pub fn resolve(
        &self,
        source_name: &str,
        source_root: &Path,
    ) -> Result<GeneratorCatalog, GeneratorCatalogSourceResolverError> {
        let generator_directories = self
            .source
            .discover_generator_directories(source_root)
            .map_err(
                |reason| GeneratorCatalogSourceResolverError::SourceScanFailed {
                    source_name: source_name.to_owned(),
                    reason,
                },
            )?;

        let mut generators = Vec::with_capacity(generator_directories.len());

        for generator_directory in generator_directories {
            let metadata_content = match self.source.read_generator_metadata(&generator_directory) {
                Ok(content) => content,
                Err(_) => continue,
            };

            let metadata = match self
                .metadata_parser
                .parse_generator_metadata(&metadata_content)
            {
                Ok(metadata) => metadata,
                Err(_) => continue,
            };

            generators.push(GeneratorDescriptor::new(metadata, generator_directory));
        }

        generators.sort_by(|left, right| left.metadata.id.cmp(&right.metadata.id));

        Ok(GeneratorCatalog::new(source_name.to_owned(), generators))
    }
}

#[cfg(test)]
#[path = "generator_catalog_source_resolver.tests.rs"]
mod tests;
