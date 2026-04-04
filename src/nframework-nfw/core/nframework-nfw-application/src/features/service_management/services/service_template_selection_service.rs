use std::fs;

use serde_yaml::Value;

use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_template_resolution::ServiceTemplateResolution;
use crate::features::service_management::services::abstraction::service_template_selector::ServiceTemplateSelector;
use crate::features::template_management::models::errors::template_selection_error::TemplateSelectionError;
use crate::features::template_management::services::abstraction::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::template_selection_service::TemplateSelectionService;

#[derive(Debug, Clone)]
pub struct ServiceTemplateSelectionService<D>
where
    D: TemplateCatalogDiscoveryService + Clone,
{
    template_selection_service: TemplateSelectionService<D>,
    discovery_service: D,
}

impl<D> ServiceTemplateSelectionService<D>
where
    D: TemplateCatalogDiscoveryService + Clone,
{
    pub fn new(discovery_service: D) -> Self {
        Self {
            template_selection_service: TemplateSelectionService::new(discovery_service.clone()),
            discovery_service,
        }
    }
}

impl<D> ServiceTemplateSelector for ServiceTemplateSelectionService<D>
where
    D: TemplateCatalogDiscoveryService + Clone,
{
    fn resolve_service_template(
        &self,
        template_identifier: &str,
    ) -> Result<ServiceTemplateResolution, AddServiceError> {
        let selection = self
            .template_selection_service
            .select_template(template_identifier)
            .map_err(map_template_selection_error)?;

        let template_type = read_template_type(&selection.template.cache_path)?;
        if template_type != "service" {
            return Err(AddServiceError::InvalidTemplateType {
                template_id: format!(
                    "{}/{}",
                    selection.source_name, selection.template.metadata.id
                ),
                template_type,
            });
        }

        Ok(ServiceTemplateResolution {
            source_name: selection.source_name,
            template_name: selection.template.metadata.name,
            template_id: selection.template.metadata.id,
            resolved_version: selection.template.metadata.version,
            template_type,
            template_cache_path: selection.template.cache_path,
            description: selection.template.metadata.description,
        })
    }

    fn list_service_templates(&self) -> Result<Vec<ServiceTemplateResolution>, AddServiceError> {
        let (catalogs, _warnings) = self
            .discovery_service
            .discover_catalogs()
            .map_err(|error| AddServiceError::Internal(error.to_string()))?;

        let mut templates = Vec::<ServiceTemplateResolution>::new();
        for catalog in catalogs {
            for descriptor in catalog.templates {
                let template_type = read_template_type(&descriptor.cache_path)?;
                if template_type != "service" {
                    continue;
                }

                templates.push(ServiceTemplateResolution {
                    source_name: catalog.source_name.clone(),
                    template_name: descriptor.metadata.name,
                    template_id: descriptor.metadata.id,
                    resolved_version: descriptor.metadata.version,
                    template_type,
                    template_cache_path: descriptor.cache_path,
                    description: descriptor.metadata.description,
                });
            }
        }

        templates.sort_by(|left, right| {
            left.template_id
                .cmp(&right.template_id)
                .then(left.source_name.cmp(&right.source_name))
        });

        Ok(templates)
    }
}

fn map_template_selection_error(error: TemplateSelectionError) -> AddServiceError {
    match error {
        TemplateSelectionError::TemplateNotFound { identifier } => {
            AddServiceError::TemplateNotFound(identifier)
        }
        TemplateSelectionError::AmbiguousTemplateIdentifier { identifier, .. } => {
            AddServiceError::AmbiguousTemplate(identifier)
        }
        TemplateSelectionError::DiscoverTemplatesFailed(reason)
        | TemplateSelectionError::InternalError { message: reason } => {
            AddServiceError::Internal(reason)
        }
    }
}

fn read_template_type(template_cache_path: &std::path::Path) -> Result<String, AddServiceError> {
    let metadata_path = template_cache_path.join("template.yaml");
    let content = fs::read_to_string(&metadata_path).map_err(|error| {
        AddServiceError::Internal(format!(
            "failed to read '{}': {error}",
            metadata_path.display()
        ))
    })?;
    let value = serde_yaml::from_str::<Value>(&content).map_err(|error| {
        AddServiceError::Internal(format!(
            "invalid template metadata '{}': {error}",
            metadata_path.display()
        ))
    })?;

    let explicit_type = value
        .get("type")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|candidate| !candidate.is_empty())
        .map(ToOwned::to_owned);

    if let Some(explicit_type) = explicit_type {
        return Ok(explicit_type);
    }

    let inferred_from_tags = value
        .get("tags")
        .and_then(Value::as_sequence)
        .and_then(|tags| {
            let has_service_tag = tags
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .any(|tag| tag.eq_ignore_ascii_case("service"));
            if has_service_tag {
                return Some("service".to_owned());
            }

            let has_workspace_tag = tags
                .iter()
                .filter_map(Value::as_str)
                .map(str::trim)
                .any(|tag| tag.eq_ignore_ascii_case("workspace"));
            if has_workspace_tag {
                return Some("workspace".to_owned());
            }

            None
        });

    Ok(inferred_from_tags.unwrap_or_else(|| "unknown".to_owned()))
}
