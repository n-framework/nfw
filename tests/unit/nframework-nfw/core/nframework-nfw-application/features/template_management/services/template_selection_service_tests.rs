use std::str::FromStr;

use nframework_nfw_application::features::template_management::models::errors::template_selection_error::TemplateSelectionError;
use nframework_nfw_application::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use nframework_nfw_application::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use nframework_nfw_application::features::template_management::services::template_selection_service::TemplateSelectionService;
use nframework_nfw_domain::features::template_management::language::Language;
use nframework_nfw_domain::features::template_management::template_catalog::TemplateCatalog;
use nframework_nfw_domain::features::template_management::template_descriptor::TemplateDescriptor;
use nframework_nfw_domain::features::template_management::template_metadata::TemplateMetadata;
use nframework_nfw_domain::features::versioning::version::Version;

#[derive(Debug, Clone)]
struct MockDiscoveryService {
    catalogs: Vec<TemplateCatalog>,
}

impl TemplateCatalogDiscoveryService for MockDiscoveryService {
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<TemplateCatalog>, Vec<String>), TemplatesServiceError> {
        Ok((self.catalogs.clone(), vec![]))
    }
}

#[test]
fn selects_template_by_qualified_identifier() {
    let service = TemplateSelectionService::new(MockDiscoveryService {
        catalogs: vec![
            TemplateCatalog::new(
                "official".to_owned(),
                vec![TemplateDescriptor::new(
                    metadata("web-api", "Web API"),
                    "/tmp/official/web-api".into(),
                )],
            ),
            TemplateCatalog::new(
                "community".to_owned(),
                vec![TemplateDescriptor::new(
                    metadata("web-api", "Community Web API"),
                    "/tmp/community/web-api".into(),
                )],
            ),
        ],
    });

    let selected = service
        .select_template("official/web-api")
        .expect("selection should succeed");

    assert_eq!(selected.source_name, "official");
    assert_eq!(selected.template.metadata.name, "Web API");
}

#[test]
fn returns_ambiguous_error_for_unqualified_identifier_conflict() {
    let service = TemplateSelectionService::new(MockDiscoveryService {
        catalogs: vec![
            TemplateCatalog::new(
                "official".to_owned(),
                vec![TemplateDescriptor::new(
                    metadata("web-api", "Web API"),
                    "/tmp/official/web-api".into(),
                )],
            ),
            TemplateCatalog::new(
                "community".to_owned(),
                vec![TemplateDescriptor::new(
                    metadata("web-api", "Community Web API"),
                    "/tmp/community/web-api".into(),
                )],
            ),
        ],
    });

    let error = service
        .select_template("web-api")
        .expect_err("selection should fail");

    match error {
        TemplateSelectionError::AmbiguousTemplateIdentifier {
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

fn metadata(id: &str, name: &str) -> TemplateMetadata {
    TemplateMetadata {
        id: id.to_owned(),
        name: name.to_owned(),
        description: "Template".to_owned(),
        version: Version::from_str("1.0.0").expect("version should parse"),
        language: Language::Rust,
        tags: vec![],
        author: None,
        min_cli_version: None,
        source_url: None,
    }
}
