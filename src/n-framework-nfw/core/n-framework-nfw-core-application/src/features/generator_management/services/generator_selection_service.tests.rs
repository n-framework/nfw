use super::*;
use std::str::FromStr;
use crate::features::generator_management::models::errors::generator_selection_error::GeneratorSelectionError;
use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use crate::features::generator_management::services::abstractions::generator_catalog_discovery_service::GeneratorCatalogDiscoveryService;
use n_framework_nfw_core_domain::features::generator_management::language::Language;
use n_framework_nfw_core_domain::features::generator_management::generator_catalog::GeneratorCatalog;
use n_framework_nfw_core_domain::features::generator_management::generator_descriptor::GeneratorDescriptor;
use n_framework_nfw_core_domain::features::generator_management::generator_metadata::GeneratorMetadata;
use n_framework_nfw_core_domain::features::versioning::version::Version;

#[derive(Debug, Clone)]
struct MockDiscoveryService {
    catalogs: Vec<GeneratorCatalog>,
}

impl GeneratorCatalogDiscoveryService for MockDiscoveryService {
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<GeneratorCatalog>, Vec<String>), GeneratorsServiceError> {
        Ok((self.catalogs.clone(), vec![]))
    }
}

#[test]
fn selects_generator_by_qualified_identifier() {
    let service = GeneratorSelectionService::new(MockDiscoveryService {
        catalogs: vec![
            GeneratorCatalog::new(
                "official".to_owned(),
                vec![GeneratorDescriptor::new(
                    metadata("web-api", "Web API"),
                    "/tmp/official/web-api".into(),
                )],
            ),
            GeneratorCatalog::new(
                "community".to_owned(),
                vec![GeneratorDescriptor::new(
                    metadata("web-api", "Community Web API"),
                    "/tmp/community/web-api".into(),
                )],
            ),
        ],
    });

    let selected = service
        .select_generator("official/web-api")
        .expect("selection should succeed");

    assert_eq!(selected.source_name, "official");
    assert_eq!(selected.generator.metadata.name, "Web API");
}

#[test]
fn returns_ambiguous_error_for_unqualified_identifier_conflict() {
    let service = GeneratorSelectionService::new(MockDiscoveryService {
        catalogs: vec![
            GeneratorCatalog::new(
                "official".to_owned(),
                vec![GeneratorDescriptor::new(
                    metadata("web-api", "Web API"),
                    "/tmp/official/web-api".into(),
                )],
            ),
            GeneratorCatalog::new(
                "community".to_owned(),
                vec![GeneratorDescriptor::new(
                    metadata("web-api", "Community Web API"),
                    "/tmp/community/web-api".into(),
                )],
            ),
        ],
    });

    let error = service
        .select_generator("web-api")
        .expect_err("selection should fail");

    match error {
        GeneratorSelectionError::AmbiguousGeneratorIdentifier {
            identifier,
            candidates,
        } => {
            assert_eq!(identifier, "web-api");
            assert_eq!(candidates.len(), 2);
            assert!(
                candidates
                    .iter()
                    .any(|candidate| candidate == "official/web-api")
            );
            assert!(
                candidates
                    .iter()
                    .any(|candidate| candidate == "community/web-api")
            );
        }
        _ => panic!("unexpected error"),
    }
}

fn metadata(id: &str, name: &str) -> GeneratorMetadata {
    GeneratorMetadata {
        id: id.to_owned(),
        name: name.to_owned(),
        description: "Generator".to_owned(),
        version: Version::from_str("1.0.0").expect("version should parse"),
        language: Language::Rust,
        tags: vec![],
        author: None,
        min_cli_version: None,
        source_url: None,
    }
}
