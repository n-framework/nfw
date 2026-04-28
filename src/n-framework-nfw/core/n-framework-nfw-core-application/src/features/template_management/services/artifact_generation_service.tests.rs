use super::*;
use std::fs;
use tempfile::TempDir;

use crate::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use crate::features::template_management::services::template_engine::TemplateEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;

// --- Mocks ---

struct MockWorkingDir {
    current: PathBuf,
}

impl WorkingDirectoryProvider for MockWorkingDir {
    fn current_dir(&self) -> Result<PathBuf, String> {
        Ok(self.current.clone())
    }
}

struct MockResolver;
impl TemplateRootResolver for MockResolver {
    fn resolve(&self, _yaml: &YamlValue, _id: &str, _root: &Path) -> Result<PathBuf, String> {
        Ok(PathBuf::from("/templates/unused"))
    }
}

struct MockEngine;
impl TemplateEngine for MockEngine {
    fn execute(
        &self,
        _config: &TemplateConfig,
        _root: &Path,
        _output: &Path,
        _params: &TemplateParameters,
    ) -> Result<(), crate::features::template_management::models::template_error::TemplateError>
    {
        Ok(())
    }
}

// --- Helpers ---

fn setup_workspace() -> (
    TempDir,
    ArtifactGenerationService<MockWorkingDir, MockResolver, MockEngine>,
) {
    let sandbox = tempfile::tempdir().unwrap();
    let service = ArtifactGenerationService::new(
        MockWorkingDir {
            current: sandbox.path().to_path_buf(),
        },
        MockResolver,
        MockEngine,
    );
    (sandbox, service)
}

#[test]
fn add_service_module_is_idempotent() {
    let (sandbox, service) = setup_workspace();
    let nfw_yaml = r#"
workspace:
  namespace: MyProj
services:
  MyService:
    path: src/MyService
    template:
      id: dotnet-service
"#;
    fs::write(sandbox.path().join("nfw.yaml"), nfw_yaml).unwrap();

    // Add once
    service
        .add_service_module(sandbox.path(), "MyService", "mediator")
        .unwrap();

    // Verify added
    let content = fs::read_to_string(sandbox.path().join("nfw.yaml")).unwrap();
    assert!(content.contains("- mediator"));

    // Add again
    service
        .add_service_module(sandbox.path(), "MyService", "mediator")
        .unwrap();

    // Verify still only one entry
    let content = fs::read_to_string(sandbox.path().join("nfw.yaml")).unwrap();
    let occurrences = content.matches("- mediator").count();
    assert_eq!(occurrences, 1, "Module should only be added once");
}

#[test]
fn add_service_module_fails_on_malformed_yaml() {
    let (sandbox, service) = setup_workspace();
    let nfw_yaml = "invalid: yaml: [ : }";
    fs::write(sandbox.path().join("nfw.yaml"), nfw_yaml).unwrap();

    let result = service.add_service_module(sandbox.path(), "MyService", "mediator");
    assert!(result.is_err());
    if let Err(AddArtifactError::NfwYamlParseError(msg)) = result {
        assert!(msg.contains("failed to parse workspace config"));
    } else {
        panic!("Expected NfwYamlParseError, got {:?}", result);
    }
}

#[test]
fn add_service_module_fails_missing_services_key() {
    let (sandbox, service) = setup_workspace();
    let nfw_yaml = "workspace: { namespace: MyProj }";
    fs::write(sandbox.path().join("nfw.yaml"), nfw_yaml).unwrap();

    let result = service.add_service_module(sandbox.path(), "MyService", "mediator");
    assert!(result.is_err());
    if let Err(AddArtifactError::WorkspaceError(msg)) = result {
        assert!(msg.contains("missing 'services' mapping"));
    } else {
        panic!("Expected WorkspaceError, got {:?}", result);
    }
}

#[test]
fn add_service_module_fails_missing_target_service() {
    let (sandbox, service) = setup_workspace();
    let nfw_yaml = "workspace: { namespace: MyProj }\nservices: {}";
    fs::write(sandbox.path().join("nfw.yaml"), nfw_yaml).unwrap();

    let result = service.add_service_module(sandbox.path(), "OtherService", "mediator");
    assert!(result.is_err());
    if let Err(AddArtifactError::WorkspaceError(msg)) = result {
        assert!(msg.contains("service 'OtherService' not found"));
    } else {
        panic!("Expected WorkspaceError, got {:?}", result);
    }
}

#[test]
fn add_service_module_creates_modules_key_if_missing() {
    let (sandbox, service) = setup_workspace();
    let nfw_yaml = r#"
services:
  MyService:
    path: src/MyService
"#;
    fs::write(sandbox.path().join("nfw.yaml"), nfw_yaml).unwrap();

    service
        .add_service_module(sandbox.path(), "MyService", "mediator")
        .unwrap();

    let content = fs::read_to_string(sandbox.path().join("nfw.yaml")).unwrap();
    assert!(content.contains("modules:"));
    assert!(content.contains("- mediator"));
}
