use super::*;
use std::fs;
use tempfile::TempDir;

use crate::features::generator_management::models::generator_error::GeneratorError;
use crate::features::generator_management::services::abstractions::generator_root_resolver::GeneratorRootResolver;
use crate::features::generator_management::services::generator_engine::GeneratorEngine;
use crate::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::generator_management::generator_config::GeneratorConfig;
use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;

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
impl GeneratorRootResolver for MockResolver {
    fn resolve(&self, _yaml: &YamlValue, _id: &str, _root: &Path) -> Result<PathBuf, String> {
        Ok(PathBuf::from("/generators/unused"))
    }
}

struct MockEngine;
impl GeneratorEngine for MockEngine {
    fn execute(
        &self,
        _config: &GeneratorConfig,
        _root: &Path,
        _output: &Path,
        _params: &GeneratorParameters,
    ) -> Result<(), GeneratorError> {
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
fn add_service_module_returns_error_when_module_exists() {
    let (sandbox, service) = setup_workspace();
    let nfw_yaml = r#"
workspace:
  namespace: MyProj
services:
  MyService:
    path: src/MyService
    generator:
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

    // Add again - should fail with clear error
    let result = service.add_service_module(sandbox.path(), "MyService", "mediator");
    assert!(result.is_err());
    if let Err(AddArtifactError::WorkspaceError(msg)) = result {
        assert!(
            msg.contains("already registered") || msg.contains("already exists"),
            "Error should mention module already exists: {}",
            msg
        );
    } else {
        panic!("Expected WorkspaceError, got {:?}", result);
    }

    // Verify still only one entry (no duplicate)
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

struct CustomMockResolver {
    target: PathBuf,
}
impl GeneratorRootResolver for CustomMockResolver {
    fn resolve(&self, _yaml: &YamlValue, _id: &str, _root: &Path) -> Result<PathBuf, String> {
        Ok(self.target.clone())
    }
}

#[test]
fn load_generator_context_resolves_dynamic_sub_generator() {
    let sandbox = tempfile::tempdir().unwrap();
    let generator_dir = sandbox.path().join("my-generator");
    let sub_generator_dir = generator_dir.join("persistence");
    fs::create_dir_all(&sub_generator_dir).unwrap();

    let generator_yaml = r#"
id: my-generator
generators:
  persistence: "persistence"
"#;
    fs::write(generator_dir.join("nfw.generator.yaml"), generator_yaml).unwrap();
    fs::write(sub_generator_dir.join("nfw.generator.yaml"), generator_yaml).unwrap();

    let nfw_yaml = r#"
workspace:
  namespace: MyProj
services:
  MyService:
    path: src/MyService
    generator:
      id: "my-generator"
"#;
    fs::write(sandbox.path().join("nfw.yaml"), nfw_yaml).unwrap();

    let service = ArtifactGenerationService::new(
        MockWorkingDir {
            current: sandbox.path().to_path_buf(),
        },
        CustomMockResolver {
            target: generator_dir.clone(),
        },
        MockEngine,
    );

    let workspace_context = service.get_workspace_context().unwrap();
    let services = service.extract_services(&workspace_context).unwrap();
    let target_service = services
        .into_iter()
        .find(|s| s.name() == "MyService")
        .unwrap();

    let ctx = service
        .load_generator_context(workspace_context, &target_service, "persistence")
        .unwrap();

    assert_eq!(
        ctx.generator_root, sub_generator_dir,
        "Should resolve to sub-generator 'persistence' directory"
    );
}

#[test]
fn load_generator_context_fails_if_sub_generator_missing() {
    let sandbox = tempfile::tempdir().unwrap();
    let generator_dir = sandbox.path().join("my-generator");
    fs::create_dir_all(&generator_dir).unwrap();

    let generator_yaml = r#"
id: my-generator
generators:
  persistence: "persistence"
"#;
    fs::write(generator_dir.join("nfw.generator.yaml"), generator_yaml).unwrap();

    let nfw_yaml = r#"
workspace:
  namespace: MyProj
services:
  MyService:
    path: src/MyService
    generator:
      id: "my-generator"
"#;
    fs::write(sandbox.path().join("nfw.yaml"), nfw_yaml).unwrap();

    let service = ArtifactGenerationService::new(
        MockWorkingDir {
            current: sandbox.path().to_path_buf(),
        },
        CustomMockResolver {
            target: generator_dir.clone(),
        },
        MockEngine,
    );

    let workspace_context = service.get_workspace_context().unwrap();
    let services = service.extract_services(&workspace_context).unwrap();
    let target_service = services
        .into_iter()
        .find(|s| s.name() == "MyService")
        .unwrap();

    let result = service.load_generator_context(workspace_context, &target_service, "persistence");

    assert!(
        result.is_err(),
        "Should fail since persistence sub-generator is missing"
    );
}

#[test]
fn validate_identifiers_success() {
    let (_, service) = setup_workspace();
    assert!(
        service
            .validate_identifiers("ValidName", Some("ValidFeature"))
            .is_ok()
    );
    assert!(service.validate_identifiers("valid_name-123", None).is_ok());
}

#[test]
fn validate_identifiers_invalid_name() {
    let (_, service) = setup_workspace();
    let result = service.validate_identifiers("Invalid Name!", None);
    assert!(result.is_err());
    if let Err(AddArtifactError::InvalidIdentifier(msg)) = result {
        assert!(msg.contains("invalid name"));
    } else {
        panic!("Expected InvalidIdentifier, got {:?}", result);
    }
}

#[test]
fn validate_identifiers_empty_name() {
    let (_, service) = setup_workspace();
    let result = service.validate_identifiers("", None);
    assert!(result.is_err());
    if let Err(AddArtifactError::InvalidIdentifier(msg)) = result {
        assert!(msg.contains("name cannot be empty"));
    } else {
        panic!("Expected InvalidIdentifier, got {:?}", result);
    }
}

#[test]
fn validate_required_modules_fails_on_missing() {
    let (_, service) = setup_workspace();
    let nfw_yaml = serde_yaml::from_str(
        r#"
services:
  MyService:
    path: "src/MyService"
    modules: ["mediator"]
"#,
    )
    .unwrap();

    // We need to simulate required_modules. Since GeneratorConfig fields are private,
    // we use a yaml string to deserialize it.
    let config_yaml = "id: test\nrequired_modules: [\"persistence\"]\nsteps:\n  - action: run_command\n    command: echo";
    let config: GeneratorConfig = serde_yaml::from_str(config_yaml).unwrap();

    let result = service.validate_required_modules(&config, &nfw_yaml, Path::new("src/MyService"));
    assert!(result.is_err());
    if let Err(AddArtifactError::MissingRequiredModule(msg)) = result {
        assert!(msg.contains("module 'persistence' is required but not installed"));
    } else {
        panic!("Expected MissingRequiredModule, got {:?}", result);
    }
}

#[test]
fn extract_namespace_fails_on_missing() {
    let (_, service) = setup_workspace();
    let nfw_yaml = serde_yaml::from_str("workspace: {}").unwrap();
    let result = service.extract_namespace(&nfw_yaml);
    assert!(result.is_err());
    if let Err(AddArtifactError::ConfigError(msg)) = result {
        assert!(msg.contains("Missing 'workspace.namespace'"));
    } else {
        panic!("Expected ConfigError, got {:?}", result);
    }
}
#[test]
fn has_service_module_checks_registration_correctly() {
    let (sandbox, service) = setup_workspace();
    let nfw_yaml = r#"
services:
  MySvc:
    path: src/MySvc
    modules: ["webapi"]
"#;
    fs::write(sandbox.path().join("nfw.yaml"), nfw_yaml).unwrap();

    assert!(
        service
            .has_service_module(sandbox.path(), "MySvc", "webapi")
            .unwrap()
    );
    assert!(
        !service
            .has_service_module(sandbox.path(), "MySvc", "mediator")
            .unwrap()
    );
    assert!(
        !service
            .has_service_module(sandbox.path(), "OtherSvc", "webapi")
            .unwrap()
    );
}
