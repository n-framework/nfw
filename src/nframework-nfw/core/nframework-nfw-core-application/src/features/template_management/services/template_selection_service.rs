use nframework_nfw_core_domain::features::template_management::qualified_template_id::QualifiedTemplateId;
use nframework_nfw_core_domain::features::template_management::template_descriptor::TemplateDescriptor;

use crate::features::template_management::models::errors::template_selection_error::TemplateSelectionError;
use crate::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::template_selection_result::TemplateSelectionResult;

/// Maximum number of unique template matches before considering the identifier ambiguous
const MAX_UNIQUE_MATCH_COUNT: usize = 1;

#[derive(Debug, Clone)]
pub struct TemplateSelectionService<D>
where
    D: TemplateCatalogDiscoveryService,
{
    discovery_service: D,
}

impl<D> TemplateSelectionService<D>
where
    D: TemplateCatalogDiscoveryService,
{
    pub fn new(discovery_service: D) -> Self {
        Self { discovery_service }
    }

    pub fn select_template(
        &self,
        template_identifier: &str,
    ) -> Result<TemplateSelectionResult, TemplateSelectionError> {
        let (catalogs, warnings) = self
            .discovery_service
            .discover_catalogs()
            .map_err(|error| TemplateSelectionError::DiscoverTemplatesFailed(error.to_string()))?;

        let trimmed_identifier = template_identifier.trim();
        if trimmed_identifier.is_empty() {
            return Err(TemplateSelectionError::TemplateNotFound {
                identifier: template_identifier.to_owned(),
            });
        }

        let identifier = QualifiedTemplateId::parse(trimmed_identifier).ok_or_else(|| {
            TemplateSelectionError::TemplateNotFound {
                identifier: template_identifier.to_owned(),
            }
        })?;

        if identifier.is_qualified() {
            let source_name = identifier.source.as_deref().ok_or_else(|| {
                TemplateSelectionError::InternalError {
                    message: "qualified identifier parsing inconsistency: missing source"
                        .to_owned(),
                }
            })?;
            let template_id = identifier.template.as_str();
            for catalog in catalogs {
                if catalog.source_name != source_name {
                    continue;
                }

                if let Some(template) = catalog
                    .templates
                    .into_iter()
                    .find(|template| template.metadata.id == template_id)
                {
                    return Ok(TemplateSelectionResult {
                        source_name: source_name.to_owned(),
                        template,
                        warnings,
                    });
                }
            }

            return Err(TemplateSelectionError::TemplateNotFound {
                identifier: trimmed_identifier.to_owned(),
            });
        }

        let mut matches = Vec::<(String, TemplateDescriptor)>::new();
        for catalog in catalogs {
            for template in catalog.templates {
                if template.metadata.id == identifier.template.as_str() {
                    matches.push((catalog.source_name.clone(), template));
                }
            }
        }

        if matches.is_empty() {
            return Err(TemplateSelectionError::TemplateNotFound {
                identifier: identifier.template.clone(),
            });
        }

        if matches.len() > MAX_UNIQUE_MATCH_COUNT {
            let candidates = matches
                .iter()
                .map(|(source_name, template)| format!("{source_name}/{}", template.metadata.id))
                .collect::<Vec<_>>();

            return Err(TemplateSelectionError::AmbiguousTemplateIdentifier {
                identifier: identifier.template.clone(),
                candidates,
            });
        }

        // At this point, matches.len() == 1, so pop() should always succeed
        let (source_name, template) =
            matches
                .pop()
                .ok_or_else(|| TemplateSelectionError::InternalError {
                    message:
                        "template matching logic error: expected exactly one match but found none"
                            .to_owned(),
                })?;
        Ok(TemplateSelectionResult {
            source_name,
            template,
            warnings,
        })
    }
}
