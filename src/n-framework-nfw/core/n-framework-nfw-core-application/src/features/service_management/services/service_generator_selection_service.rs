use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_generator_resolution::ServiceGeneratorResolution;
use crate::features::service_management::services::abstractions::service_generator_selector::{
    ServiceGeneratorSelectionContext, ServiceGeneratorSelector,
};
use crate::features::generator_management::constants::generator;
use crate::features::generator_management::models::raw_generator_metadata::RawGeneratorMetadata;
use crate::features::generator_management::services::abstractions::generator_catalog_discovery_service::GeneratorCatalogDiscoveryService;
use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
use crate::features::generator_management::services::generator_type_resolver::read_generator_type;
use n_framework_nfw_core_domain::features::versioning::version::Version;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ServiceGeneratorSelectionService<D, R>
where
    D: GeneratorCatalogDiscoveryService + Clone,
    R: GeneratorRootResolver + Clone,
{
    discovery_service: D,
    root_resolver: R,
}

impl<D, R> ServiceGeneratorSelectionService<D, R>
where
    D: GeneratorCatalogDiscoveryService + Clone,
    R: GeneratorRootResolver + Clone,
{
    pub fn new(discovery_service: D, root_resolver: R) -> Self {
        Self {
            discovery_service,
            root_resolver,
        }
    }
}

impl<D, R> ServiceGeneratorSelector for ServiceGeneratorSelectionService<D, R>
where
    D: GeneratorCatalogDiscoveryService + Clone,
    R: GeneratorRootResolver + Clone,
{
    fn resolve_service_generator(
        &self,
        generator_identifier: &str,
        context: ServiceGeneratorSelectionContext<'_>,
    ) -> Result<ServiceGeneratorResolution, AddServiceError> {
        let (source, _) = Self::parse_identifier(generator_identifier);

        let generator_root = self.root_resolver
            .resolve(context.nfw_yaml, generator_identifier, context.workspace_root)
            .map_err(|e| {
                tracing::warn!(
                    "Failed to resolve generator '{}' locally, falling back to catalog search. Error: {}",
                    generator_identifier,
                    e
                );
                AddServiceError::GeneratorNotFound(generator_identifier.to_owned())
            })?;

        let generator_type =
            read_generator_type(&generator_root).map_err(AddServiceError::GeneratorReadError)?;

        if !generator_type.eq_ignore_ascii_case("service") {
            return Err(AddServiceError::InvalidGeneratorType {
                generator_id: generator_identifier.to_owned(),
                generator_type,
            });
        }

        let metadata_path = generator_root.join(generator::METADATA_FILE);
        let yaml = fs::read_to_string(&metadata_path).map_err(|e| {
            AddServiceError::GeneratorReadError(format!("failed to read generator.yaml: {e}"))
        })?;

        let raw = serde_yaml::from_str::<RawGeneratorMetadata>(&yaml).map_err(|e| {
            AddServiceError::GeneratorConfigError(format!("failed to parse generator.yaml: {e}"))
        })?;

        Ok(ServiceGeneratorResolution {
            source_name: source.to_owned(),
            generator_name: raw.name.ok_or_else(|| {
                AddServiceError::GeneratorConfigError(format!(
                    "Generator metadata 'name' is missing in {}",
                    metadata_path.display()
                ))
            })?,
            generator_id: raw.id.ok_or_else(|| {
                AddServiceError::GeneratorConfigError(format!(
                    "Generator metadata 'id' is missing in {}",
                    metadata_path.display()
                ))
            })?,
            resolved_version: match raw.version {
                Some(ref v) => Version::from_str(v).map_err(|_| {
                    AddServiceError::GeneratorConfigError(format!(
                        "Generator metadata 'version' is invalid ('{}') in {}",
                        v,
                        metadata_path.display()
                    ))
                })?,
                None => {
                    return Err(AddServiceError::Internal(format!(
                        "Generator metadata 'version' is missing in {}",
                        metadata_path.display()
                    )));
                }
            },
            generator_type,
            generator_cache_path: generator_root,
            description: raw.description.unwrap_or_default(),
        })
    }

    fn list_service_generators(&self) -> Result<Vec<ServiceGeneratorResolution>, AddServiceError> {
        let (catalogs, _warnings) = self
            .discovery_service
            .discover_catalogs()
            .map_err(|error| AddServiceError::Internal(error.to_string()))?;

        let mut generators = Vec::<ServiceGeneratorResolution>::new();
        for catalog in catalogs {
            for descriptor in catalog.generators {
                let generator_type = read_generator_type(&descriptor.cache_path)
                    .map_err(AddServiceError::Internal)?;
                if !generator_type.eq_ignore_ascii_case("service") {
                    continue;
                }

                generators.push(ServiceGeneratorResolution {
                    source_name: catalog.source_name.clone(),
                    generator_name: descriptor.metadata.name,
                    generator_id: descriptor.metadata.id,
                    resolved_version: descriptor.metadata.version,
                    generator_type,
                    generator_cache_path: descriptor.cache_path,
                    description: descriptor.metadata.description,
                });
            }
        }

        generators.sort_by(|left, right| {
            left.generator_id
                .cmp(&right.generator_id)
                .then(left.source_name.cmp(&right.source_name))
        });

        Ok(generators)
    }
}

impl<D, R> ServiceGeneratorSelectionService<D, R>
where
    D: GeneratorCatalogDiscoveryService + Clone,
    R: GeneratorRootResolver + Clone,
{
    fn parse_identifier(identifier: &str) -> (&str, &str) {
        if let Some(pos) = identifier.find('/') {
            (&identifier[..pos], &identifier[pos + 1..])
        } else {
            ("local", identifier)
        }
    }
}

#[cfg(test)]
#[path = "service_generator_selection_service.tests.rs"]
mod tests;
