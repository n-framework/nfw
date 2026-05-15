use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;
use n_framework_nfw_core_application::features::service_management::services::abstractions::service_generator_selector::{
    ServiceGeneratorSelectionContext, ServiceGeneratorSelector,
};
use n_framework_nfw_core_application::features::service_management::services::service_generator_selection_service::ServiceGeneratorSelectionService;
use n_framework_nfw_core_application::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::generator_catalog_discovery_service::GeneratorCatalogDiscoveryService;
use n_framework_nfw_core_domain::features::generator_management::language::Language;
use n_framework_nfw_core_domain::features::generator_management::generator_catalog::GeneratorCatalog;
use n_framework_nfw_core_domain::features::generator_management::generator_descriptor::GeneratorDescriptor;
use n_framework_nfw_core_domain::features::generator_management::generator_metadata::GeneratorMetadata;
use n_framework_nfw_core_domain::features::versioning::version::Version;
use serde_yaml::Value as YamlValue;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;

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
    known_generators: std::collections::HashMap<String, PathBuf>,
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
fn fails_for_unknown_generator_identifier() {
    let sandbox = create_sandbox_directory("service-generator-validation-not-found");
    let service_generator_path = create_generator_dir(&sandbox, "dotnet-service", "service");

    let selector = ServiceGeneratorSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![GeneratorCatalog::new(
                "official".to_owned(),
                vec![descriptor("dotnet-service", service_generator_path.clone())],
            )],
        },
        StubRootResolver {
            known_generators: [("official/dotnet-service".to_owned(), service_generator_path)]
                .into_iter()
                .collect(),
        },
    );

    let error = selector
        .resolve_service_generator(
            "official/missing",
            ServiceGeneratorSelectionContext::new(Path::new("."), &YamlValue::Null),
        )
        .expect_err("missing generator should fail");

    match error {
        AddServiceError::GeneratorNotFound(identifier) => {
            assert_eq!(identifier, "official/missing");
        }
        other => panic!("unexpected error: {other}"),
    }

    cleanup_sandbox_directory(&sandbox);
}

#[test]
fn fails_for_generator_with_non_service_type() {
    let sandbox = create_sandbox_directory("service-generator-validation-type");
    let workspace_generator_path = create_generator_dir(&sandbox, "blank-workspace", "workspace");

    let selector = ServiceGeneratorSelectionService::new(
        StubDiscoveryService {
            catalogs: vec![GeneratorCatalog::new(
                "official".to_owned(),
                vec![descriptor(
                    "blank-workspace",
                    workspace_generator_path.clone(),
                )],
            )],
        },
        StubRootResolver {
            known_generators: [(
                "official/blank-workspace".to_owned(),
                workspace_generator_path,
            )]
            .into_iter()
            .collect(),
        },
    );

    let error = selector
        .resolve_service_generator(
            "official/blank-workspace",
            ServiceGeneratorSelectionContext::new(Path::new("."), &YamlValue::Null),
        )
        .expect_err("wrong generator type should fail");

    match error {
        AddServiceError::InvalidGeneratorType {
            generator_id,
            generator_type,
        } => {
            assert_eq!(generator_id, "official/blank-workspace");
            assert_eq!(generator_type, "workspace");
        }
        other => panic!("unexpected error: {other}"),
    }

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

fn create_generator_dir(root: &Path, generator_name: &str, generator_type: &str) -> PathBuf {
    let generator_root = root.join(generator_name);
    fs::create_dir_all(generator_root.join("content"))
        .expect("generator content directory should be created");
    fs::write(
        generator_root.join("nfw.generator.yaml"),
        format!(
            "id: {generator_name}\nname: {generator_name}\ndescription: test\nversion: 1.0.0\ntype: {generator_type}\n"
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

fn cleanup_sandbox_directory(path: &Path) {
    let _ = fs::remove_dir_all(path);
}
