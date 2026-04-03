use std::path::PathBuf;
use std::str::FromStr;

use nframework_core_cli_abstraction::{PromptError, PromptService, SelectOption};
use nframework_nfw_application::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use nframework_nfw_application::features::template_management::services::abstraction::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use nframework_nfw_application::features::workspace_management::services::template_selection_for_new_service::TemplateSelectionForNewService;
use nframework_nfw_domain::features::template_management::language::Language;
use nframework_nfw_domain::features::template_management::template_catalog::TemplateCatalog;
use nframework_nfw_domain::features::template_management::template_descriptor::TemplateDescriptor;
use nframework_nfw_domain::features::template_management::template_metadata::TemplateMetadata;
use nframework_nfw_domain::features::versioning::version::Version;

#[derive(Debug, Clone)]
struct StubDiscoveryService {
    catalogs: Vec<TemplateCatalog>,
}

impl TemplateCatalogDiscoveryService for StubDiscoveryService {
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<TemplateCatalog>, Vec<String>), TemplatesServiceError> {
        Ok((self.catalogs.clone(), Vec::new()))
    }
}

#[derive(Debug, Clone, Copy)]
struct StubPromptService;

impl PromptService for StubPromptService {
    fn is_interactive(&self) -> bool {
        false
    }

    fn text(&self, _message: &str, _default: Option<&str>) -> Result<String, PromptError> {
        Ok("stub-value".to_owned())
    }

    fn confirm(&self, _message: &str, _default: bool) -> Result<bool, PromptError> {
        Ok(true)
    }

    fn select(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: usize,
    ) -> Result<SelectOption, PromptError> {
        Err(PromptError::internal("not implemented"))
    }

    fn select_index(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: usize,
    ) -> Result<usize, PromptError> {
        Ok(0)
    }
}

#[test]
fn defaults_to_official_blank_workspace_when_template_is_not_provided() {
    let service = TemplateSelectionForNewService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![
                    descriptor("service-starter", "/tmp/official/service-starter"),
                    descriptor("blank-workspace", "/tmp/official/blank-workspace"),
                ],
            )],
        },
        StubPromptService,
    );

    let selected_template_id = service
        .resolve_template_id(None)
        .expect("default template selection should succeed");

    assert_eq!(selected_template_id, "official/blank-workspace");
}

#[test]
fn falls_back_to_first_official_template_when_blank_workspace_missing() {
    let service = TemplateSelectionForNewService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![descriptor(
                    "service-starter",
                    "/tmp/official/service-starter",
                )],
            )],
        },
        StubPromptService,
    );

    let selected_template_id = service
        .resolve_template_id(None)
        .expect("fallback template selection should succeed");

    assert_eq!(selected_template_id, "official/service-starter");
}

fn descriptor(id: &str, path: &str) -> TemplateDescriptor {
    let metadata = TemplateMetadata::builder()
        .id(id.to_owned())
        .name(format!("Template {id}"))
        .description("Template description".to_owned())
        .version(Version::from_str("1.0.0").expect("version should parse"))
        .language(Language::Dotnet)
        .build()
        .expect("metadata should be valid");

    TemplateDescriptor::new(metadata, PathBuf::from(path))
}
