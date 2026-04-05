use crate::features::template_management::constants::source;
use crate::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::template_management::services::template_selection_result::TemplateSelectionResult;
use crate::features::template_management::services::template_type_resolver::read_template_type;
use crate::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use nframework_core_cli_abstraction::{PromptService, SelectOption};
use nframework_nfw_domain::features::template_management::template_descriptor::TemplateDescriptor;

const DEFAULT_TEMPLATE_ID_PREFERENCES: [&str; 3] = ["blank-workspace", "blank", "workspace-blank"];

#[derive(Debug, Clone)]
pub struct TemplateSelectionForNewService<S, P>
where
    S: TemplateCatalogDiscoveryService + Clone,
    P: PromptService + Clone,
{
    template_catalog_discovery_service: S,
    prompt_service: P,
}

impl<S, P> TemplateSelectionForNewService<S, P>
where
    S: TemplateCatalogDiscoveryService + Clone,
    P: PromptService + Clone,
{
    pub fn new(template_catalog_discovery_service: S, prompt_service: P) -> Self {
        Self {
            template_catalog_discovery_service,
            prompt_service,
        }
    }

    pub fn resolve_template_selection(
        &self,
        requested_template: Option<&str>,
        allow_interactive: bool,
    ) -> Result<TemplateSelectionResult, WorkspaceNewError> {
        let (catalogs, warnings) = self
            .template_catalog_discovery_service
            .discover_catalogs()
            .map_err(|error| WorkspaceNewError::Internal(error.to_string()))?;

        let mut templates = Vec::<(String, TemplateDescriptor)>::new();
        for catalog in catalogs {
            for template in catalog.templates {
                if is_workspace_template(&template)? {
                    templates.push((catalog.source_name.clone(), template));
                }
            }
        }

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
                        || format!("{}/{}", source_name, template.metadata.id) == requested_template
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
            let default_match = templates
                .iter()
                .find(|(source_name, template)| {
                    source_name == source::OFFICIAL_NAME
                        && template.metadata.id == preferred_template_id
                })
                .cloned();

            if let Some((source_name, template)) = default_match {
                if allow_interactive && self.prompt_service.is_interactive() {
                    return self.prompt_template_selection(
                        templates,
                        warnings,
                        source_name,
                        template,
                    );
                }

                return Ok(TemplateSelectionResult {
                    source_name,
                    template,
                    warnings,
                });
            }
        }

        if allow_interactive && self.prompt_service.is_interactive() {
            return self.prompt_template_selection_interactive(templates, warnings);
        }

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

    fn prompt_template_selection(
        &self,
        templates: Vec<(String, TemplateDescriptor)>,
        warnings: Vec<String>,
        default_source_name: String,
        default_template: TemplateDescriptor,
    ) -> Result<TemplateSelectionResult, WorkspaceNewError> {
        let options: Vec<SelectOption> = templates
            .iter()
            .map(|(source_name, template)| {
                SelectOption::new(
                    format!("{}/{}", source_name, template.metadata.id),
                    format!("{}/{}", source_name, template.metadata.id),
                )
                .with_description(&template.metadata.description)
            })
            .collect();

        let default_index = templates
            .iter()
            .position(|(source_name, template)| {
                source_name == &default_source_name
                    && template.metadata.id == default_template.metadata.id
            })
            .unwrap_or(0);

        let selected_index = self
            .prompt_service
            .select_index("Select a template:", &options, Some(default_index))
            .map_err(|error| WorkspaceNewError::PromptFailed(error.to_string()))?;

        let (source_name, template) = templates
            .into_iter()
            .nth(selected_index)
            .expect("selected index out of bounds");

        Ok(TemplateSelectionResult {
            source_name,
            template,
            warnings,
        })
    }

    fn prompt_template_selection_interactive(
        &self,
        templates: Vec<(String, TemplateDescriptor)>,
        warnings: Vec<String>,
    ) -> Result<TemplateSelectionResult, WorkspaceNewError> {
        let options: Vec<SelectOption> = templates
            .iter()
            .map(|(source_name, template)| {
                SelectOption::new(
                    format!("{}/{}", source_name, template.metadata.id),
                    format!("{}/{}", source_name, template.metadata.id),
                )
                .with_description(&template.metadata.description)
            })
            .collect();

        let default_index = templates
            .iter()
            .position(|(source_name, _)| source_name == source::OFFICIAL_NAME)
            .or_else(|| templates.first().map(|_| 0))
            .unwrap_or(0);

        let selected_index = self
            .prompt_service
            .select_index("Select a template:", &options, Some(default_index))
            .map_err(|error| WorkspaceNewError::PromptFailed(error.to_string()))?;

        let (source_name, template) = templates
            .into_iter()
            .nth(selected_index)
            .expect("selected index out of bounds");

        Ok(TemplateSelectionResult {
            source_name,
            template,
            warnings,
        })
    }

    pub fn resolve_template_id(
        &self,
        requested_template: Option<&str>,
    ) -> Result<String, WorkspaceNewError> {
        let selection = self.resolve_template_selection(requested_template, false)?;
        Ok(format!(
            "{}/{}",
            selection.source_name, selection.template.metadata.id
        ))
    }
}

fn is_workspace_template(template: &TemplateDescriptor) -> Result<bool, WorkspaceNewError> {
    let has_workspace_tag = template
        .metadata
        .tags
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case("workspace"));
    if has_workspace_tag {
        return Ok(true);
    }

    let has_service_tag = template
        .metadata
        .tags
        .iter()
        .any(|tag| tag.eq_ignore_ascii_case("service"));
    if has_service_tag {
        return Ok(false);
    }

    let template_type = read_template_type(&template.cache_path)
        .map_err(|error| WorkspaceNewError::Internal(error.to_string()))?;

    Ok(template_type.eq_ignore_ascii_case("workspace"))
}
