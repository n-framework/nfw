use std::fs;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::services::abstractions::service_template_selector::{
    ServiceTemplateSelectionContext, ServiceTemplateSelector,
};
use crate::features::service_management::services::service_template_selection_service::ServiceTemplateSelectionService;
use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use n_framework_nfw_core_domain::features::template_management::language::Language;
use n_framework_nfw_core_domain::features::template_management::template_catalog::TemplateCatalog;
use n_framework_nfw_core_domain::features::template_management::template_descriptor::TemplateDescriptor;
use n_framework_nfw_core_domain::features::template_management::template_metadata::TemplateMetadata;
use n_framework_nfw_core_domain::features::versioning::version::Version;
use serde_yaml::Value as YamlValue;
use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
struct StubRootResolver {
    known_templates: HashMap<String, PathBuf>,
}

impl TemplateRootResolver for StubRootResolver {
    fn resolve(
        &self,
        _nfw_yaml: &YamlValue,
        template_id: &str,
        _workspace_root: &Path,
    ) -> Result<PathBuf, String> {
        self.known_templates
            .get(template_id)
            .cloned()
            .ok_or_else(|| "not found".to_owned())
    }
}

#[test]
fn list_service_templates_filters_non_service_templates() {
    let sandbox = create_sandbox_directory("service-selection-list");
    let service_template_dir = create_template_directory(&sandbox, "dotnet-service", "service");
    let workspace_template_dir =
        create_template_directory(&sandbox, "blank-workspace", "workspace");

    let service = ServiceTemplateSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![
                    descriptor("dotnet-service", service_template_dir.clone()),
                    descriptor("blank-workspace", workspace_template_dir),
                ],
            )],
        },
        StubRootResolver {
            known_templates: [("official/dotnet-service".to_owned(), service_template_dir)]
                .into_iter()
                .collect(),
        },
    );

    let templates = service
        .list_service_templates()
        .expect("service template listing should succeed");

    assert_eq!(templates.len(), 1);
    assert_eq!(
        templates[0].qualified_template_id(),
        "official/dotnet-service"
    );

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn resolve_service_template_rejects_wrong_template_type() {
    let sandbox = create_sandbox_directory("service-selection-type");
    let workspace_template_dir =
        create_template_directory(&sandbox, "blank-workspace", "workspace");

    let service = ServiceTemplateSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![descriptor(
                    "blank-workspace",
                    workspace_template_dir.clone(),
                )],
            )],
        },
        StubRootResolver {
            known_templates: [(
                "official/blank-workspace".to_owned(),
                workspace_template_dir,
            )]
            .into_iter()
            .collect(),
        },
    );

    let error = service
        .resolve_service_template(
            "official/blank-workspace",
            ServiceTemplateSelectionContext::new(Path::new("."), &YamlValue::Null),
        )
        .expect_err("workspace template type should be rejected");

    match error {
        AddServiceError::InvalidTemplateType { template_id, .. } => {
            assert_eq!(template_id, "official/blank-workspace");
        }
        other => panic!("unexpected error: {other}"),
    }

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn list_service_templates_accepts_service_tag_without_type_field() {
    let sandbox = create_sandbox_directory("service-selection-list-tags");
    let service_template_dir =
        create_template_directory_with_tags(&sandbox, "dotnet-service", &["service", "dotnet"]);
    let workspace_template_dir =
        create_template_directory_with_tags(&sandbox, "blank-workspace", &["workspace"]);

    let service = ServiceTemplateSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![
                    descriptor("dotnet-service", service_template_dir.clone()),
                    descriptor("blank-workspace", workspace_template_dir),
                ],
            )],
        },
        StubRootResolver {
            known_templates: [("official/dotnet-service".to_owned(), service_template_dir)]
                .into_iter()
                .collect(),
        },
    );

    let templates = service
        .list_service_templates()
        .expect("service template listing should succeed");

    assert_eq!(templates.len(), 1);
    assert_eq!(
        templates[0].qualified_template_id(),
        "official/dotnet-service"
    );

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn resolve_service_template_accepts_service_tag_without_type_field() {
    let sandbox = create_sandbox_directory("service-selection-resolve-tags");
    let service_template_dir =
        create_template_directory_with_tags(&sandbox, "dotnet-service", &["service", "dotnet"]);

    let service = ServiceTemplateSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![descriptor("dotnet-service", service_template_dir.clone())],
            )],
        },
        StubRootResolver {
            known_templates: [("official/dotnet-service".to_owned(), service_template_dir)]
                .into_iter()
                .collect(),
        },
    );

    let resolution = service
        .resolve_service_template(
            "official/dotnet-service",
            ServiceTemplateSelectionContext::new(Path::new("."), &YamlValue::Null),
        )
        .expect("service tag should classify template as service");

    assert_eq!(resolution.template_type, "service");
    assert_eq!(
        resolution.qualified_template_id(),
        "official/dotnet-service"
    );

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn resolve_service_template_accepts_case_insensitive_type_field() {
    let sandbox = create_sandbox_directory("service-selection-resolve-case-insensitive-type");
    let service_template_dir = create_template_directory(&sandbox, "dotnet-service", "Service");

    let service = ServiceTemplateSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![TemplateCatalog::new(
                "official".to_owned(),
                vec![descriptor("dotnet-service", service_template_dir.clone())],
            )],
        },
        StubRootResolver {
            known_templates: [("official/dotnet-service".to_owned(), service_template_dir)]
                .into_iter()
                .collect(),
        },
    );

    let resolution = service
        .resolve_service_template(
            "official/dotnet-service",
            ServiceTemplateSelectionContext::new(Path::new("."), &YamlValue::Null),
        )
        .expect("type field should be matched case-insensitively");

    assert_eq!(
        resolution.qualified_template_id(),
        "official/dotnet-service"
    );

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

fn create_template_directory(root: &std::path::Path, name: &str, template_type: &str) -> PathBuf {
    let template_root = root.join(name);
    fs::create_dir_all(template_root.join("content"))
        .expect("template content directory should be created");
    fs::write(
        template_root.join("template.yaml"),
        format!(
            "id: {name}\nname: {name}\ndescription: test\nversion: 1.0.0\ntype: {template_type}\n"
        ),
    )
    .expect("template metadata should be written");

    template_root
}

fn create_template_directory_with_tags(
    root: &std::path::Path,
    name: &str,
    tags: &[&str],
) -> PathBuf {
    let template_root = root.join(name);
    fs::create_dir_all(template_root.join("content"))
        .expect("template content directory should be created");
    let rendered_tags = tags
        .iter()
        .map(|tag| format!("  - {tag}\n"))
        .collect::<String>();
    fs::write(
        template_root.join("template.yaml"),
        format!(
            "id: {name}\nname: {name}\ndescription: test\nversion: 1.0.0\ntags:\n{rendered_tags}"
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

fn cleanup_sandbox_directory(path: &std::path::Path) {
    let _ = fs::remove_dir_all(path);
}
