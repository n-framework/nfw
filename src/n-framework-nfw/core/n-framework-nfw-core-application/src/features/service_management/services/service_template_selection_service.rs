use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_template_resolution::ServiceTemplateResolution;
use crate::features::service_management::services::abstractions::service_template_selector::{
    ServiceTemplateSelectionContext, ServiceTemplateSelector,
};
use crate::features::template_management::models::raw_template_metadata::RawTemplateMetadata;
use crate::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::template_type_resolver::read_template_type;
use n_framework_nfw_core_domain::features::versioning::version::Version;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct ServiceTemplateSelectionService<D, R>
where
    D: TemplateCatalogDiscoveryService + Clone,
    R: TemplateRootResolver + Clone,
{
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
        context: ServiceTemplateSelectionContext<'_>,
    ) -> Result<ServiceTemplateResolution, AddServiceError> {
        let (source, _id) = Self::parse_identifier(template_identifier);

        let local_path = context.workspace_root.join(template_identifier);
        let template_root = if source == "local" {
            if local_path.is_dir() {
                local_path.clone()
            } else {
                return Err(AddServiceError::TemplateNotFound(
                    template_identifier.to_owned(),
                ));
            }
        } else {
            self.root_resolver
                .resolve(context.nfw_yaml, template_identifier, context.workspace_root)
                .map_err(|e| {
                    tracing::warn!(
                        "Failed to resolve template '{}' locally at {}, falling back to catalog search. Error: {}",
                        template_identifier,
                        local_path.display(),
                        e
                    );
                    AddServiceError::TemplateNotFound(template_identifier.to_owned())
                })?
        };

        let template_type =
            read_template_type(&template_root).map_err(AddServiceError::TemplateReadError)?;

        if !template_type.eq_ignore_ascii_case("service") {
            return Err(AddServiceError::InvalidTemplateType {
                template_id: template_identifier.to_owned(),
                template_type,
            });
        }

        let metadata_path = template_root.join("template.yaml");
        let yaml = fs::read_to_string(&metadata_path).map_err(|e| {
            AddServiceError::TemplateReadError(format!("failed to read template.yaml: {e}"))
        })?;

        let raw = serde_yaml::from_str::<RawTemplateMetadata>(&yaml).map_err(|e| {
            AddServiceError::TemplateConfigError(format!("failed to parse template.yaml: {e}"))
        })?;

        Ok(ServiceTemplateResolution {
            source_name: source.to_owned(),
            template_name: raw.name.ok_or_else(|| {
                AddServiceError::TemplateConfigError(format!(
                    "Template metadata 'name' is missing in {}",
                    metadata_path.display()
                ))
            })?,
            template_id: raw.id.ok_or_else(|| {
                AddServiceError::TemplateConfigError(format!(
                    "Template metadata 'id' is missing in {}",
                    metadata_path.display()
                ))
            })?,
            resolved_version: match raw.version {
                Some(ref v) => Version::from_str(v).map_err(|_| {
                    AddServiceError::TemplateConfigError(format!(
                        "Template metadata 'version' is invalid ('{}') in {}",
                        v,
                        metadata_path.display()
                    ))
                })?,
                None => {
                    return Err(AddServiceError::Internal(format!(
                        "Template metadata 'version' is missing in {}",
                        metadata_path.display()
                    )));
                }
            },
            template_type,
            template_cache_path: template_root,
            description: raw.description.unwrap_or_default(),
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

impl<D, R> ServiceTemplateSelectionService<D, R>
where
    D: TemplateCatalogDiscoveryService + Clone,
    R: TemplateRootResolver + Clone,
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
#[path = "service_template_selection_service.tests.rs"]
mod tests;
