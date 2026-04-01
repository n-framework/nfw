use nframework_nfw_domain::features::template_management::template_descriptor::TemplateDescriptor;

use crate::features::template_management::models::errors::template_selection_error::TemplateSelectionError;
use crate::features::template_management::services::abstraction::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::template_selection_result::TemplateSelectionResult;

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

        if let Some((source_name, template_id)) = parse_qualified_identifier(trimmed_identifier) {
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
                if template.metadata.id == trimmed_identifier {
                    matches.push((catalog.source_name.clone(), template));
                }
            }
        }

        if matches.is_empty() {
            return Err(TemplateSelectionError::TemplateNotFound {
                identifier: trimmed_identifier.to_owned(),
            });
        }

        if matches.len() > 1 {
            let candidates = matches
                .iter()
                .map(|(source_name, template)| format!("{source_name}/{}", template.metadata.id))
                .collect::<Vec<_>>();

            return Err(TemplateSelectionError::AmbiguousTemplateIdentifier {
                identifier: trimmed_identifier.to_owned(),
                candidates,
            });
        }

        let (source_name, template) = matches
            .pop()
            .expect("template matches must contain exactly one item");
        Ok(TemplateSelectionResult {
            source_name,
            template,
            warnings,
        })
    }
}

fn parse_qualified_identifier(identifier: &str) -> Option<(&str, &str)> {
    let (source_name, template_id) = identifier.split_once('/')?;
    if source_name.trim().is_empty() || template_id.trim().is_empty() {
        return None;
    }

    Some((source_name.trim(), template_id.trim()))
}
