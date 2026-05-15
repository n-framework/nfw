use crate::features::generator_management::constants::source;
use crate::features::generator_management::services::abstractions::generator_catalog_discovery_service::GeneratorCatalogDiscoveryService;
use crate::features::generator_management::services::generator_selection_result::GeneratorSelectionResult;
use crate::features::generator_management::services::generator_type_resolver::read_generator_type;
use crate::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use n_framework_core_cli_abstractions::{InteractivePrompt, Logger, SelectOption};
use n_framework_nfw_core_domain::features::generator_management::generator_descriptor::GeneratorDescriptor;

const DEFAULT_GENERATOR_ID_PREFERENCES: [&str; 3] = ["blank-workspace", "blank", "workspace-blank"];

#[derive(Debug, Clone)]
pub struct GeneratorSelectionForNewService<S, P>
where
    S: GeneratorCatalogDiscoveryService + Clone,
    P: InteractivePrompt + Logger + Clone,
{
    generator_catalog_discovery_service: S,
    prompt_service: P,
}

impl<S, P> GeneratorSelectionForNewService<S, P>
where
    S: GeneratorCatalogDiscoveryService + Clone,
    P: InteractivePrompt + Logger + Clone,
{
    pub fn new(generator_catalog_discovery_service: S, prompt_service: P) -> Self {
        Self {
            generator_catalog_discovery_service,
            prompt_service,
        }
    }

    pub fn resolve_generator_selection(
        &self,
        requested_generator: Option<&str>,
        allow_interactive: bool,
    ) -> Result<GeneratorSelectionResult, WorkspaceNewError> {
        let spinner = if allow_interactive && self.prompt_service.is_interactive() {
            Some(
                self.prompt_service
                    .spinner("Discovering generators...")
                    .map_err(|error| WorkspaceNewError::Internal(error.to_string()))?,
            )
        } else {
            None
        };

        let discovery_result = self.generator_catalog_discovery_service.discover_catalogs();

        if let Some(spinner) = &spinner {
            if discovery_result.is_ok() {
                spinner.stop("Generators discovered");
            } else {
                spinner.error("Failed to discover generators");
            }
        }

        let (catalogs, warnings) =
            discovery_result.map_err(|error| WorkspaceNewError::Internal(error.to_string()))?;

        let mut generators = Vec::<(String, GeneratorDescriptor)>::new();
        for catalog in catalogs {
            for generator in catalog.generators {
                if is_workspace_generator(&generator)? {
                    generators.push((catalog.source_name.clone(), generator));
                }
            }
        }

        if generators.is_empty() {
            if let Some(req) = requested_generator {
                return Err(WorkspaceNewError::GeneratorNotFound(req.to_owned()));
            } else {
                return Err(WorkspaceNewError::NoWorkspaceGeneratorsDiscovered);
            }
        }

        if let Some(requested_generator) = requested_generator {
            let exact_matches = generators
                .iter()
                .filter(|(source_name, generator)| {
                    generator.metadata.id == requested_generator
                        || format!("{}/{}", source_name, generator.metadata.id)
                            == requested_generator
                })
                .map(|(source_name, generator)| (source_name.clone(), generator.clone()))
                .collect::<Vec<_>>();

            return match exact_matches.len() {
                0 => Err(WorkspaceNewError::GeneratorNotFound(
                    requested_generator.to_owned(),
                )),
                1 => Ok(GeneratorSelectionResult {
                    source_name: exact_matches[0].0.clone(),
                    generator: exact_matches[0].1.clone(),
                    warnings: warnings.clone(),
                }),
                _ => Err(WorkspaceNewError::AmbiguousGenerator(
                    requested_generator.to_owned(),
                )),
            };
        }

        for preferred_generator_id in DEFAULT_GENERATOR_ID_PREFERENCES {
            let default_match = generators
                .iter()
                .find(|(source_name, generator)| {
                    source_name == source::OFFICIAL_NAME
                        && generator.metadata.id == preferred_generator_id
                })
                .cloned();

            if let Some((source_name, generator)) = default_match {
                if allow_interactive && self.prompt_service.is_interactive() {
                    return self.prompt_generator_selection(
                        generators,
                        warnings,
                        source_name,
                        generator,
                    );
                }

                return Ok(GeneratorSelectionResult {
                    source_name,
                    generator,
                    warnings,
                });
            }
        }

        if allow_interactive && self.prompt_service.is_interactive() {
            return self.prompt_generator_selection_interactive(generators, warnings);
        }

        let fallback = generators
            .iter()
            .find(|(source_name, _)| source_name == source::OFFICIAL_NAME)
            .or_else(|| generators.first())
            .expect("BUG: generators non-empty invariant violated");

        Ok(GeneratorSelectionResult {
            source_name: fallback.0.clone(),
            generator: fallback.1.clone(),
            warnings,
        })
    }

    fn prompt_generator_selection(
        &self,
        generators: Vec<(String, GeneratorDescriptor)>,
        warnings: Vec<String>,
        default_source_name: String,
        default_generator: GeneratorDescriptor,
    ) -> Result<GeneratorSelectionResult, WorkspaceNewError> {
        let options: Vec<SelectOption> = generators
            .iter()
            .map(|(source_name, generator)| {
                SelectOption::new(
                    format!("{}/{}", source_name, generator.metadata.id),
                    format!("{}/{}", source_name, generator.metadata.id),
                )
                .with_description(&generator.metadata.description)
            })
            .collect();

        let default_index = generators
            .iter()
            .position(|(source_name, generator)| {
                source_name == &default_source_name
                    && generator.metadata.id == default_generator.metadata.id
            })
            .unwrap_or(0);

        let selected_index = self
            .prompt_service
            .select_index("Select a generator:", &options, Some(default_index))
            .map_err(|error| WorkspaceNewError::PromptFailed(error.to_string()))?;

        let (source_name, generator) = generators
            .into_iter()
            .nth(selected_index)
            .expect("selected index out of bounds");

        Ok(GeneratorSelectionResult {
            source_name,
            generator,
            warnings,
        })
    }

    fn prompt_generator_selection_interactive(
        &self,
        generators: Vec<(String, GeneratorDescriptor)>,
        warnings: Vec<String>,
    ) -> Result<GeneratorSelectionResult, WorkspaceNewError> {
        let options: Vec<SelectOption> = generators
            .iter()
            .map(|(source_name, generator)| {
                SelectOption::new(
                    format!("{}/{}", source_name, generator.metadata.id),
                    format!("{}/{}", source_name, generator.metadata.id),
                )
                .with_description(&generator.metadata.description)
            })
            .collect();

        let default_index = generators
            .iter()
            .position(|(source_name, _)| source_name == source::OFFICIAL_NAME)
            .or_else(|| generators.first().map(|_| 0))
            .unwrap_or(0);

        let selected_index = self
            .prompt_service
            .select_index("Select a generator:", &options, Some(default_index))
            .map_err(|error| WorkspaceNewError::PromptFailed(error.to_string()))?;

        let (source_name, generator) = generators
            .into_iter()
            .nth(selected_index)
            .expect("selected index out of bounds");

        Ok(GeneratorSelectionResult {
            source_name,
            generator,
            warnings,
        })
    }

    pub fn resolve_generator_id(
        &self,
        requested_generator: Option<&str>,
    ) -> Result<String, WorkspaceNewError> {
        let selection = self.resolve_generator_selection(requested_generator, false)?;
        Ok(format!(
            "{}/{}",
            selection.source_name, selection.generator.metadata.id
        ))
    }
}

fn is_workspace_generator(generator: &GeneratorDescriptor) -> Result<bool, WorkspaceNewError> {
    let has_workspace_tag = generator
        .metadata
        .tags
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case("workspace"));
    if has_workspace_tag {
        return Ok(true);
    }

    let has_service_tag = generator
        .metadata
        .tags
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case("service"));
    if has_service_tag {
        return Ok(false);
    }

    let generator_type = read_generator_type(&generator.cache_path)
        .map_err(|error| WorkspaceNewError::Internal(error.to_string()))?;

    Ok(generator_type.eq_ignore_ascii_case("workspace"))
}

#[cfg(test)]
#[path = "generator_selection_for_new_service.tests.rs"]
mod tests;
