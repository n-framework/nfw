use crate::features::template_management::constants::source;
use crate::features::template_management::services::abstraction::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::template_selection_result::TemplateSelectionResult;
use crate::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;

const DEFAULT_TEMPLATE_ID_PREFERENCES: [&str; 3] = ["blank-workspace", "blank", "workspace-blank"];

#[derive(Debug, Clone)]
pub struct TemplateSelectionForNewService<S>
where
    S: TemplateCatalogDiscoveryService,
{
    template_catalog_discovery_service: S,
}

impl<S> TemplateSelectionForNewService<S>
where
    S: TemplateCatalogDiscoveryService,
{
    pub fn new(template_catalog_discovery_service: S) -> Self {
        Self {
            template_catalog_discovery_service,
        }
    }

    pub fn resolve_template_selection(
        &self,
        requested_template: Option<&str>,
    ) -> Result<TemplateSelectionResult, WorkspaceNewError> {
        let (catalogs, warnings) = self
            .template_catalog_discovery_service
            .discover_catalogs()
            .map_err(|error| WorkspaceNewError::Internal(error.to_string()))?;

        let templates = catalogs
            .into_iter()
            .flat_map(|catalog| {
                catalog
                    .templates
                    .into_iter()
                    .map(move |template| (catalog.source_name.clone(), template))
            })
            .collect::<Vec<_>>();

        if templates.is_empty() {
            return Err(WorkspaceNewError::TemplateNotFound(
                requested_template.unwrap_or("<default>").to_owned(),
            ));
        }

        if let Some(requested_template) = requested_template {
            let exact_matches = templates
                .iter()
                .filter(|(source_name, template)| {
                    template.metadata.id == requested_template
                        || format!("{}/{}", source_name, template.metadata.id)
                            == requested_template
                })
                .map(|(source_name, template)| (source_name.clone(), template.clone()))
                .collect::<Vec<_>>();

            return match exact_matches.len() {
                0 => Err(WorkspaceNewError::TemplateNotFound(
                    requested_template.to_owned(),
                )),
                1 => Ok(TemplateSelectionResult {
                    source_name: exact_matches[0].0.clone(),
                    template: exact_matches[0].1.clone(),
                    warnings: warnings.clone(),
                }),
                _ => Err(WorkspaceNewError::AmbiguousTemplate(
                    requested_template.to_owned(),
                )),
            };
        }

        for preferred_template_id in DEFAULT_TEMPLATE_ID_PREFERENCES {
            if let Some((source_name, template)) = templates.iter().find(|(source_name, template)| {
                source_name == source::OFFICIAL_NAME && template.metadata.id == preferred_template_id
            }) {
                return Ok(TemplateSelectionResult {
                    source_name: source_name.clone(),
                    template: template.clone(),
                    warnings,
                });
            }
        }

        // SAFETY: We verified templates.is_empty() == false above at line 45
        let fallback = templates
            .iter()
            .find(|(source_name, _)| source_name == source::OFFICIAL_NAME)
            .or_else(|| templates.first())
            .expect("BUG: templates non-empty invariant violated");

        Ok(TemplateSelectionResult {
            source_name: fallback.0.clone(),
            template: fallback.1.clone(),
            warnings,
        })
    }

    pub fn resolve_template_id(
        &self,
        requested_template: Option<&str>,
    ) -> Result<String, WorkspaceNewError> {
        let selection = self.resolve_template_selection(requested_template)?;
        Ok(format!(
            "{}/{}",
            selection.source_name, selection.template.metadata.id
        ))
    }
}
