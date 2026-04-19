use super::*;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use n_framework_core_cli_abstractions::{InteractiveError, InteractivePrompt, Logger, LoggingError, SelectOption, Spinner};
use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use crate::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use n_framework_nfw_core_domain::features::template_management::language::Language;
use n_framework_nfw_core_domain::features::template_management::template_catalog::TemplateCatalog;
use n_framework_nfw_core_domain::features::template_management::template_descriptor::TemplateDescriptor;
use n_framework_nfw_core_domain::features::template_management::template_metadata::TemplateMetadata;
use n_framework_nfw_core_domain::features::versioning::version::Version;

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

impl InteractivePrompt for StubPromptService {
    fn is_interactive(&self) -> bool {
        false
    }

    fn text(&self, _message: &str, _default: Option<&str>) -> Result<String, InteractiveError> {
        Ok("stub-value".to_owned())
    }

    fn confirm(&self, _message: &str, _default: bool) -> Result<bool, InteractiveError> {
        Ok(true)
    }

    fn password(&self, _message: &str) -> Result<String, InteractiveError> {
        Ok("stub-password".to_owned())
    }

    fn select(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<SelectOption, InteractiveError> {
        Err(InteractiveError::internal("not implemented"))
    }

    fn select_index(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<usize, InteractiveError> {
        Ok(0)
    }

    fn multiselect(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_indices: &[usize],
    ) -> Result<Vec<SelectOption>, InteractiveError> {
        Ok(Vec::new())
    }
}

impl Logger for StubPromptService {
    fn intro(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn outro(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_cancel(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_info(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_step(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_success(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_warning(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_error(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn spinner(&self, _message: &str) -> Result<Box<dyn Spinner>, LoggingError> {
        struct NoopSpinner;
        impl Spinner for NoopSpinner {
            fn stop(&self, _message: &str) {}
            fn success(&self, _message: &str) {}
            fn error(&self, _message: &str) {}
            fn cancel(&self, _message: &str) {}
            fn set_message(&self, _message: &str) {}
            fn is_finished(&self) -> bool {
                true
            }
        }
        Ok(Box::new(NoopSpinner))
    }
}

#[test]
fn defaults_to_official_blank_workspace_when_template_is_not_provided() {
    let service = TemplateSelectionForNewService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![
                    descriptor(
                        "service-starter",
                        "/tmp/official/service-starter",
                        &["service"],
                    ),
                    descriptor(
                        "blank-workspace",
                        "/tmp/official/blank-workspace",
                        &["workspace"],
                    ),
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
fn returns_template_not_found_when_only_service_templates_exist() {
    let service = TemplateSelectionForNewService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![descriptor(
                    "service-starter",
                    "/tmp/official/service-starter",
                    &["service"],
                )],
            )],
        },
        StubPromptService,
    );

    let error = service
        .resolve_template_id(None)
        .expect_err("workspace template selection should fail");

    assert_eq!(error, WorkspaceNewError::NoWorkspaceTemplatesDiscovered);
}

#[test]
fn filters_out_service_templates_when_workspace_templates_exist() {
    let service = TemplateSelectionForNewService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![
                    descriptor(
                        "dotnet-service",
                        "/tmp/official/dotnet-service",
                        &["service"],
                    ),
                    descriptor(
                        "workspace-starter",
                        "/tmp/official/workspace-starter",
                        &["workspace"],
                    ),
                ],
            )],
        },
        StubPromptService,
    );

    let selected_template_id = service
        .resolve_template_id(None)
        .expect("workspace template selection should succeed");
    assert_eq!(selected_template_id, "official/workspace-starter");
}

#[test]
fn accepts_workspace_template_from_explicit_type_without_tags() {
    let sandbox_root = create_sandbox_directory("workspace-template-type");
    let workspace_template_path = sandbox_root.join("workspace-template");
    fs::create_dir_all(&workspace_template_path).expect("template directory should be created");
    fs::write(
        workspace_template_path.join("template.yaml"),
        "id: workspace-starter\nname: Workspace Starter\ndescription: workspace\nversion: 1.0.0\ntype: workspace\n",
    )
    .expect("template metadata should be written");

    let service = TemplateSelectionForNewService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![descriptor_with_path(
                    "workspace-starter",
                    workspace_template_path.clone(),
                    &[],
                )],
            )],
        },
        StubPromptService,
    );

    let selected_template_id = service
        .resolve_template_id(None)
        .expect("workspace template selection should succeed");

    assert_eq!(selected_template_id, "official/workspace-starter");
    cleanup_sandbox_directory(&sandbox_root);
}

#[test]
fn accepts_workspace_template_from_case_insensitive_type_without_tags() {
    let sandbox_root = create_sandbox_directory("workspace-template-type-case-insensitive");
    let workspace_template_path = sandbox_root.join("workspace-template");
    fs::create_dir_all(&workspace_template_path).expect("template directory should be created");
    fs::write(
        workspace_template_path.join("template.yaml"),
        "id: workspace-starter\nname: Workspace Starter\ndescription: workspace\nversion: 1.0.0\ntype: WORKSPACE\n",
    )
    .expect("template metadata should be written");

    let service = TemplateSelectionForNewService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![descriptor_with_path(
                    "workspace-starter",
                    workspace_template_path.clone(),
                    &[],
                )],
            )],
        },
        StubPromptService,
    );

    let selected_template_id = service
        .resolve_template_id(None)
        .expect("workspace template selection should succeed");

    assert_eq!(selected_template_id, "official/workspace-starter");
    cleanup_sandbox_directory(&sandbox_root);
}

fn descriptor(id: &str, path: &str, tags: &[&str]) -> TemplateDescriptor {
    descriptor_with_path(id, PathBuf::from(path), tags)
}

fn descriptor_with_path(path_id: &str, path: PathBuf, tags: &[&str]) -> TemplateDescriptor {
    let metadata = TemplateMetadata::builder()
        .id(path_id.to_owned())
        .name(format!("Template {path_id}"))
        .description("Template description".to_owned())
        .version(Version::from_str("1.0.0").expect("version should parse"))
        .language(Language::Dotnet)
        .tags(tags.iter().map(|value| (*value).to_owned()).collect())
        .build()
        .expect("metadata should be valid");

    TemplateDescriptor::new(metadata, path)
}

fn create_sandbox_directory(test_name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("nfw-{test_name}-{unique}"));
    fs::create_dir_all(&path).expect("sandbox directory should be created");
    path
}

fn cleanup_sandbox_directory(path: &Path) {
    let _ = fs::remove_dir_all(path);
}
