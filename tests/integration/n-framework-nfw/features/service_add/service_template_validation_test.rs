use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_template_selector::ServiceTemplateSelector;
use n_framework_nfw_core_application::features::service_management::services::service_template_selection_service::ServiceTemplateSelectionService;
use n_framework_nfw_core_application::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
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

#[test]
fn fails_for_unknown_template_identifier() {
    let sandbox = create_sandbox_directory("service-template-validation-not-found");
    let service_template_path = create_template_dir(&sandbox, "dotnet-service", "service");

    let selector = ServiceTemplateSelectionService::new(StubDiscoveryService {
        catalogs: vec![TemplateCatalog::new(
            "official".to_owned(),
            vec![descriptor("dotnet-service", service_template_path)],
        )],
    });

    let error = selector
        .resolve_service_template("official/missing")
        .expect_err("missing template should fail");

    match error {
        AddServiceError::TemplateNotFound(identifier) => {
            assert_eq!(identifier, "official/missing");
        }
        other => panic!("unexpected error: {other}"),
    }

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn fails_for_template_with_non_service_type() {
    let sandbox = create_sandbox_directory("service-template-validation-type");
    let workspace_template_path = create_template_dir(&sandbox, "blank-workspace", "workspace");

    let selector = ServiceTemplateSelectionService::new(StubDiscoveryService {
        catalogs: vec![TemplateCatalog::new(
            "official".to_owned(),
            vec![descriptor("blank-workspace", workspace_template_path)],
        )],
    });

    let error = selector
        .resolve_service_template("official/blank-workspace")
        .expect_err("wrong template type should fail");

    match error {
        AddServiceError::InvalidTemplateType {
            template_id,
            template_type,
        } => {
            assert_eq!(template_id, "official/blank-workspace");
            assert_eq!(template_type, "workspace");
        }
        other => panic!("unexpected error: {other}"),
    }

    cleanup_sandbox_directory(&sandbox);
}

fn descriptor(id: &str, cache_path: PathBuf) -> TemplateDescriptor {
    let metadata = TemplateMetadata::builder()
        .id(id.to_owned())
        .name(format!("Template {id}"))
        .description("Template description".to_owned())
        .version(Version::from_str("1.0.0").expect("version should parse"))
        .language(Language::Dotnet)
        .build()
        .expect("metadata should be valid");

    TemplateDescriptor::new(metadata, cache_path)
}

fn create_template_dir(root: &Path, template_name: &str, template_type: &str) -> PathBuf {
    let template_root = root.join(template_name);
    fs::create_dir_all(template_root.join("content"))
        .expect("template content directory should be created");
    fs::write(
        template_root.join("template.yaml"),
        format!(
            "id: {template_name}\nname: {template_name}\ndescription: test\nversion: 1.0.0\ntype: {template_type}\n"
        ),
    )
    .expect("template metadata should be written");
    template_root
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
