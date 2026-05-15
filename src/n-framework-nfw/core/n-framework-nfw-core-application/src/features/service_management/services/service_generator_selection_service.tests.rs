use std::fs;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::services::abstractions::service_generator_selector::{
    ServiceGeneratorSelectionContext, ServiceGeneratorSelector,
};
use crate::features::service_management::services::service_generator_selection_service::ServiceGeneratorSelectionService;
use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use crate::features::generator_management::services::abstractions::generator_catalog_discovery_service::GeneratorCatalogDiscoveryService;
use n_framework_nfw_core_domain::features::generator_management::language::Language;
use n_framework_nfw_core_domain::features::generator_management::generator_catalog::GeneratorCatalog;
use n_framework_nfw_core_domain::features::generator_management::generator_descriptor::GeneratorDescriptor;
use n_framework_nfw_core_domain::features::generator_management::generator_metadata::GeneratorMetadata;
use n_framework_nfw_core_domain::features::versioning::version::Version;
use serde_yaml::Value as YamlValue;
use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct StubDiscoveryService {
    catalogs: Vec<GeneratorCatalog>,
}

impl GeneratorCatalogDiscoveryService for StubDiscoveryService {
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<GeneratorCatalog>, Vec<String>), GeneratorsServiceError> {
        Ok((self.catalogs.clone(), Vec::new()))
    }
}

#[derive(Debug, Clone)]
struct StubRootResolver {
    known_generators: HashMap<String, PathBuf>,
}

impl GeneratorRootResolver for StubRootResolver {
    fn resolve(
        &self,
        _nfw_yaml: &YamlValue,
        generator_id: &str,
        _workspace_root: &Path,
    ) -> Result<PathBuf, String> {
        self.known_generators
            .get(generator_id)
            .cloned()
            .ok_or_else(|| "not found".to_owned())
    }
}

#[test]
fn list_service_generators_filters_non_service_generators() {
    let sandbox = create_sandbox_directory("service-selection-list");
    let service_generator_dir = create_generator_directory(&sandbox, "dotnet-service", "service");
    let workspace_generator_dir =
        create_generator_directory(&sandbox, "blank-workspace", "workspace");

    let service = ServiceGeneratorSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![GeneratorCatalog::new(
                "official".to_owned(),
                vec![
                    descriptor("dotnet-service", service_generator_dir.clone()),
                    descriptor("blank-workspace", workspace_generator_dir),
                ],
            )],
        },
        StubRootResolver {
            known_generators: [("official/dotnet-service".to_owned(), service_generator_dir)]
                .into_iter()
                .collect(),
        },
    );

    let generators = service
        .list_service_generators()
        .expect("service generator listing should succeed");

    assert_eq!(generators.len(), 1);
    assert_eq!(
        generators[0].qualified_generator_id(),
        "official/dotnet-service"
    );

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn resolve_service_generator_rejects_wrong_generator_type() {
    let sandbox = create_sandbox_directory("service-selection-type");
    let workspace_generator_dir =
        create_generator_directory(&sandbox, "blank-workspace", "workspace");

    let service = ServiceGeneratorSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![GeneratorCatalog::new(
                "official".to_owned(),
                vec![descriptor(
                    "blank-workspace",
                    workspace_generator_dir.clone(),
                )],
            )],
        },
        StubRootResolver {
            known_generators: [(
                "official/blank-workspace".to_owned(),
                workspace_generator_dir,
            )]
            .into_iter()
            .collect(),
        },
    );

    let error = service
        .resolve_service_generator(
            "official/blank-workspace",
            ServiceGeneratorSelectionContext::new(Path::new("."), &YamlValue::Null),
        )
        .expect_err("workspace generator type should be rejected");

    match error {
        AddServiceError::InvalidGeneratorType { generator_id, .. } => {
            assert_eq!(generator_id, "official/blank-workspace");
        }
        other => panic!("unexpected error: {other}"),
    }

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn list_service_generators_accepts_service_tag_without_type_field() {
    let sandbox = create_sandbox_directory("service-selection-list-tags");
    let service_generator_dir =
        create_generator_directory_with_tags(&sandbox, "dotnet-service", &["service", "dotnet"]);
    let workspace_generator_dir =
        create_generator_directory_with_tags(&sandbox, "blank-workspace", &["workspace"]);

    let service = ServiceGeneratorSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![GeneratorCatalog::new(
                "official".to_owned(),
                vec![
                    descriptor("dotnet-service", service_generator_dir.clone()),
                    descriptor("blank-workspace", workspace_generator_dir),
                ],
            )],
        },
        StubRootResolver {
            known_generators: [("official/dotnet-service".to_owned(), service_generator_dir)]
                .into_iter()
                .collect(),
        },
    );

    let generators = service
        .list_service_generators()
        .expect("service generator listing should succeed");

    assert_eq!(generators.len(), 1);
    assert_eq!(
        generators[0].qualified_generator_id(),
        "official/dotnet-service"
    );

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn resolve_service_generator_accepts_service_tag_without_type_field() {
    let sandbox = create_sandbox_directory("service-selection-resolve-tags");
    let service_generator_dir =
        create_generator_directory_with_tags(&sandbox, "dotnet-service", &["service", "dotnet"]);

    let service = ServiceGeneratorSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![GeneratorCatalog::new(
                "official".to_owned(),
                vec![descriptor("dotnet-service", service_generator_dir.clone())],
            )],
        },
        StubRootResolver {
            known_generators: [("official/dotnet-service".to_owned(), service_generator_dir)]
                .into_iter()
                .collect(),
        },
    );

    let resolution = service
        .resolve_service_generator(
            "official/dotnet-service",
            ServiceGeneratorSelectionContext::new(Path::new("."), &YamlValue::Null),
        )
        .expect("service tag should classify generator as service");

    assert_eq!(resolution.generator_type, "service");
    assert_eq!(
        resolution.qualified_generator_id(),
        "official/dotnet-service"
    );

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn resolve_service_generator_accepts_case_insensitive_type_field() {
    let sandbox = create_sandbox_directory("service-selection-resolve-case-insensitive-type");
    let service_generator_dir = create_generator_directory(&sandbox, "dotnet-service", "Service");

    let service = ServiceGeneratorSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![GeneratorCatalog::new(
                "official".to_owned(),
                vec![descriptor("dotnet-service", service_generator_dir.clone())],
            )],
        },
        StubRootResolver {
            known_generators: [("official/dotnet-service".to_owned(), service_generator_dir)]
                .into_iter()
                .collect(),
        },
    );

    let resolution = service
        .resolve_service_generator(
            "official/dotnet-service",
            ServiceGeneratorSelectionContext::new(Path::new("."), &YamlValue::Null),
        )
        .expect("type field should be matched case-insensitively");

    assert_eq!(
        resolution.qualified_generator_id(),
        "official/dotnet-service"
    );

    cleanup_sandbox_directory(&sandbox);
}

fn descriptor(id: &str, cache_path: PathBuf) -> GeneratorDescriptor {
    let metadata = GeneratorMetadata::builder()
        .id(id.to_owned())
        .name(format!("Generator {id}"))
        .description("Generator description".to_owned())
        .version(Version::from_str("1.0.0").expect("version should parse"))
        .language(Language::Dotnet)
        .build()
        .expect("metadata should be valid");

    GeneratorDescriptor::new(metadata, cache_path)
}

fn create_generator_directory(root: &std::path::Path, name: &str, generator_type: &str) -> PathBuf {
    let generator_root = root.join(name);
    fs::create_dir_all(generator_root.join("content"))
        .expect("generator content directory should be created");
    fs::write(
        generator_root.join("nfw.generator.yaml"),
        format!(
            "id: {name}\nname: {name}\ndescription: test\nversion: 1.0.0\ntype: {generator_type}\n"
        ),
    )
    .expect("generator metadata should be written");

    generator_root
}

fn create_generator_directory_with_tags(
    root: &std::path::Path,
    name: &str,
    tags: &[&str],
) -> PathBuf {
    let generator_root = root.join(name);
    fs::create_dir_all(generator_root.join("content"))
        .expect("generator content directory should be created");
    let rendered_tags = tags
        .iter()
        .map(|tag| format!("  - {tag}\n"))
        .collect::<String>();
    fs::write(
        generator_root.join("nfw.generator.yaml"),
        format!(
            "id: {name}\nname: {name}\ndescription: test\nversion: 1.0.0\ntags:\n{rendered_tags}"
        ),
    )
    .expect("generator metadata should be written");

    generator_root
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
