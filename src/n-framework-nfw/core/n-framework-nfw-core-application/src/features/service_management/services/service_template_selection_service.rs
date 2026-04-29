use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_template_resolution::ServiceTemplateResolution;
use crate::features::service_management::services::abstractions::service_template_selector::ServiceTemplateSelector;
use crate::features::template_management::models::errors::template_selection_error::TemplateSelectionError;
use crate::features::template_management::models::raw_template_metadata::RawTemplateMetadata;
use crate::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::template_selection_service::TemplateSelectionService;
use crate::features::template_management::services::template_type_resolver::read_template_type;
use n_framework_nfw_core_domain::features::versioning::version::Version;
use serde_yaml::Value as YamlValue;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ServiceTemplateSelectionService<D, R>
where
    D: TemplateCatalogDiscoveryService + Clone,
    R: TemplateRootResolver + Clone,
{
    template_selection_service: TemplateSelectionService<D>,
    discovery_service: D,
    root_resolver: R,
}

impl<D, R> ServiceTemplateSelectionService<D, R>
where
    D: TemplateCatalogDiscoveryService + Clone,
    R: TemplateRootResolver + Clone,
{
    pub fn new(discovery_service: D, root_resolver: R) -> Self {
        Self {
            template_selection_service: TemplateSelectionService::new(discovery_service.clone()),
            discovery_service,
            root_resolver,
        }
    }
}

impl<D, R> ServiceTemplateSelector for ServiceTemplateSelectionService<D, R>
where
    D: TemplateCatalogDiscoveryService + Clone,
    R: TemplateRootResolver + Clone,
{
    fn resolve_service_template(
        &self,
        template_identifier: &str,
        workspace_root: &Path,
        nfw_yaml: &YamlValue,
    ) -> Result<ServiceTemplateResolution, AddServiceError> {
        // 1. Try resolving via local template_sources in nfw.yaml
        if let Ok(local_path) =
            self.root_resolver
                .resolve(nfw_yaml, template_identifier, workspace_root)
        {
            let template_type =
                read_template_type(&local_path).map_err(AddServiceError::Internal)?;

            if !template_type.eq_ignore_ascii_case("service") {
                return Err(AddServiceError::InvalidTemplateType {
                    template_id: template_identifier.to_owned(),
                    template_type,
                });
            }

            let metadata_path = local_path.join("template.yaml");
            let yaml = fs::read_to_string(&metadata_path).map_err(|e| {
                AddServiceError::Internal(format!("failed to read local template.yaml: {e}"))
            })?;

            let raw = serde_yaml::from_str::<RawTemplateMetadata>(&yaml).map_err(|e| {
                AddServiceError::Internal(format!("failed to parse local template.yaml: {e}"))
            })?;

            return Ok(ServiceTemplateResolution {
                source_name: "local".to_owned(),
                template_name: raw.name.unwrap_or_else(|| template_identifier.to_owned()),
                template_id: raw.id.unwrap_or_else(|| template_identifier.to_owned()),
                resolved_version: raw
                    .version
                    .and_then(|v| Version::from_str(&v).ok())
                    .unwrap_or_else(|| Version::from_str("1.0.0").unwrap()),
                template_type,
                template_cache_path: local_path,
                description: raw.description.unwrap_or_default(),
            });
        }

        // 2. Fallback to registered catalog discovery
        let selection = self
            .template_selection_service
            .select_template(template_identifier)
            .map_err(map_template_selection_error)?;

        let template_type = read_template_type(&selection.template.cache_path)
            .map_err(AddServiceError::Internal)?;
        if !template_type.eq_ignore_ascii_case("service") {
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
                let template_type = read_template_type(&descriptor.cache_path)
                    .map_err(AddServiceError::Internal)?;
                if !template_type.eq_ignore_ascii_case("service") {
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

#[cfg(test)]
#[path = "service_template_selection_service.tests.rs"]
mod tests;
