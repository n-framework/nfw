use n_framework_nfw_core_domain::features::generator_management::generator_descriptor::GeneratorDescriptor;
use n_framework_nfw_core_domain::features::generator_management::qualified_generator_id::QualifiedGeneratorId;

use crate::features::generator_management::models::errors::generator_selection_error::GeneratorSelectionError;
use crate::features::generator_management::services::abstractions::generator_catalog_discovery_service::GeneratorCatalogDiscoveryService;
use crate::features::generator_management::services::generator_selection_result::GeneratorSelectionResult;

/// Maximum number of unique generator matches before considering the identifier ambiguous
const MAX_UNIQUE_MATCH_COUNT: usize = 1;

#[derive(Debug, Clone)]
pub struct GeneratorSelectionService<D>
where
    D: GeneratorCatalogDiscoveryService,
{
    discovery_service: D,
}

impl<D> GeneratorSelectionService<D>
where
    D: GeneratorCatalogDiscoveryService,
{
    pub fn new(discovery_service: D) -> Self {
        Self { discovery_service }
    }

    pub fn select_generator(
        &self,
        generator_identifier: &str,
    ) -> Result<GeneratorSelectionResult, GeneratorSelectionError> {
        let (catalogs, warnings) = self
            .discovery_service
            .discover_catalogs()
            .map_err(|error| {
                GeneratorSelectionError::DiscoverGeneratorsFailed(error.to_string())
            })?;

        let trimmed_identifier = generator_identifier.trim();
        if trimmed_identifier.is_empty() {
            return Err(GeneratorSelectionError::GeneratorNotFound {
                identifier: generator_identifier.to_owned(),
            });
        }

        let identifier = QualifiedGeneratorId::parse(trimmed_identifier).ok_or_else(|| {
            GeneratorSelectionError::GeneratorNotFound {
                identifier: generator_identifier.to_owned(),
            }
        })?;

        if identifier.is_qualified() {
            let source_name = identifier.source.as_deref().ok_or_else(|| {
                GeneratorSelectionError::InternalError {
                    message: "qualified identifier parsing inconsistency: missing source"
                        .to_owned(),
                }
            })?;
            let generator_id = identifier.generator.as_str();
            for catalog in catalogs {
                if catalog.source_name != source_name {
                    continue;
                }

                if let Some(generator) = catalog
                    .generators
                    .into_iter()
                    .find(|generator| generator.metadata.id == generator_id)
                {
                    return Ok(GeneratorSelectionResult {
                        source_name: source_name.to_owned(),
                        generator,
                        warnings,
                    });
                }
            }

            return Err(GeneratorSelectionError::GeneratorNotFound {
                identifier: trimmed_identifier.to_owned(),
            });
        }

        let mut matches = Vec::<(String, GeneratorDescriptor)>::new();
        for catalog in catalogs {
            for generator in catalog.generators {
                if generator.metadata.id == identifier.generator.as_str() {
                    matches.push((catalog.source_name.clone(), generator));
                }
            }
        }

        if matches.is_empty() {
            return Err(GeneratorSelectionError::GeneratorNotFound {
                identifier: identifier.generator.clone(),
            });
        }

        if matches.len() > MAX_UNIQUE_MATCH_COUNT {
            let candidates = matches
                .iter()
                .map(|(source_name, generator)| format!("{source_name}/{}", generator.metadata.id))
                .collect::<Vec<_>>();

            return Err(GeneratorSelectionError::AmbiguousGeneratorIdentifier {
                identifier: identifier.generator.clone(),
                candidates,
            });
        }

        // At this point, matches.len() == 1, so pop() should always succeed
        let (source_name, generator) =
            matches
                .pop()
                .ok_or_else(|| GeneratorSelectionError::InternalError {
                    message:
                        "generator matching logic error: expected exactly one match but found none"
                            .to_owned(),
                })?;
        Ok(GeneratorSelectionResult {
            source_name,
            generator,
            warnings,
        })
    }
}

#[cfg(test)]
#[path = "generator_selection_service.tests.rs"]
mod tests;
