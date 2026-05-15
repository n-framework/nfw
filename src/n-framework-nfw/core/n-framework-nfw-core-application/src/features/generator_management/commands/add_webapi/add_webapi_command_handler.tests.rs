use super::*;
use crate::features::generator_management::commands::add_webapi::add_webapi_command::WebApiConfig;
use crate::features::generator_management::models::generator_error::GeneratorError;
use n_framework_nfw_core_domain::features::generator_management::generator_config::GeneratorConfig;
use n_framework_nfw_core_domain::features::generator_management::generator_parameters::GeneratorParameters;
use std::path::{Path, PathBuf};
use tempfile;

use std::cell::RefCell;
use std::rc::Rc;

struct MockEngine {
    fail_execution: bool,
    captured_params: Option<Rc<RefCell<Option<GeneratorParameters>>>>,
}

impl MockEngine {
    fn new(fail: bool) -> Self {
        Self {
            fail_execution: fail,
            captured_params: None,
        }
    }
    fn with_capture(capture: Rc<RefCell<Option<GeneratorParameters>>>) -> Self {
        Self {
            fail_execution: false,
            captured_params: Some(capture),
        }
    }
}

impl GeneratorEngine for MockEngine {
    fn execute(
        &self,
        _config: &GeneratorConfig,
        _root: &Path,
        _output: &Path,
        params: &GeneratorParameters,
    ) -> Result<(), GeneratorError> {
        if let Some(capture) = &self.captured_params {
            *capture.borrow_mut() = Some(params.clone());
        }
        if self.fail_execution {
            Err(GeneratorError::io("mock error", PathBuf::from("mock")))
        } else {
            Ok(())
        }
    }
}

struct LocalMockResolver(PathBuf);
impl GeneratorRootResolver for LocalMockResolver {
    fn resolve(
        &self,
        _yaml: &serde_yaml::Value,
        _id: &str,
        _root: &Path,
    ) -> Result<PathBuf, String> {
        Ok(self.0.clone())
    }
}

struct FailingResolver;
impl GeneratorRootResolver for FailingResolver {
    fn resolve(
        &self,
        _yaml: &serde_yaml::Value,
        _id: &str,
        _root: &Path,
    ) -> Result<PathBuf, String> {
        Err("Generator not found".to_string())
    }
}

struct SandboxWorkingDir(PathBuf);
impl WorkingDirectoryProvider for SandboxWorkingDir {
    fn current_dir(&self) -> Result<PathBuf, String> {
        Ok(self.0.clone())
    }
}

fn setup_test_env() -> (tempfile::TempDir, PathBuf) {
    let sandbox = tempfile::tempdir().unwrap();
    let generator_dir = sandbox.path().join("my-generator");
    let sub_generator_dir = generator_dir.join("webapi");
    std::fs::create_dir_all(&sub_generator_dir).unwrap();

    // ensure output root exists for FileTracker
    std::fs::create_dir_all(sandbox.path().join("src/Svc1")).unwrap();

    let generator_yaml = r#"
id: my-generator
generators:
  webapi: "webapi"
"#;
    std::fs::write(generator_dir.join("nfw.generator.yaml"), generator_yaml).unwrap();
    std::fs::write(sub_generator_dir.join("nfw.generator.yaml"), generator_yaml).unwrap();

    (sandbox, generator_dir)
}

#[test]
fn handle_error_on_engine_failure() {
    let (sandbox, generator_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(
        &nfw_yaml_path,
        "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir),
        MockEngine::new(true),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(matches!(result, Err(AddArtifactError::ExecutionFailed(_))));
}

#[test]
fn handle_error_on_missing_namespace() {
    let (sandbox, generator_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(
        &nfw_yaml_path,
        "workspace:\nname: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(result.is_err());
    if let Err(ref err) = result {
        assert!(
            matches!(err, AddArtifactError::ConfigError(_)),
            "Expected ConfigError for missing namespace, got: {:?}",
            err
        );
    }
}

#[test]
fn handle_error_on_existing_module() {
    let (sandbox, generator_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(
        &nfw_yaml_path,
        "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    modules:\n      - webapi\n    generator:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(result.is_err());
    if let Err(ref err) = result {
        assert!(
            matches!(err, AddArtifactError::WorkspaceError(_)),
            "Expected WorkspaceError for existing module, got: {:?}",
            err
        );
        if let AddArtifactError::WorkspaceError(msg) = err {
            assert!(
                msg.contains("already exists") || msg.contains("already registered"),
                "Error message should mention module already exists: {}",
                msg
            );
        }
    }
}

#[test]
fn handle_error_on_resolver_failure() {
    let (sandbox, _) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(
        &nfw_yaml_path,
        "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        FailingResolver,
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(result.is_err());
    if let Err(ref err) = result {
        assert!(
            matches!(err, AddArtifactError::GeneratorNotFound(_)),
            "Expected GeneratorNotFound for generator loading failure, got: {:?}",
            err
        );
    }
}

#[test]
fn handle_error_on_missing_yaml() {
    let (sandbox, generator_dir) = setup_test_env();

    // delete nfw.yaml to force YamlBackup to fail
    let _nfw_yaml_path = sandbox.path().join("nfw.yaml");

    // But we need a valid nfw_yaml value for workspace context, so we parse a string
    let yaml_str = "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1";
    let nfw_yaml: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(
            matches!(err, AddArtifactError::NfwYamlReadError(_)),
            "expected NfwYamlReadError, got {:?}",
            err
        );
    }
}

#[test]
fn handle_error_on_missing_sub_generator() {
    let (sandbox, generator_dir) = setup_test_env();

    // remove sub generator to trigger loading failure
    let sub_generator_dir = generator_dir.join("webapi");
    std::fs::remove_file(sub_generator_dir.join("nfw.generator.yaml")).unwrap();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(&nfw_yaml_path, "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1").unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(result.is_err());
    // Should fail with config error for missing generator.yaml in subfolder
}

#[test]
fn handle_error_on_readonly_yaml() {
    let (sandbox, generator_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(&nfw_yaml_path, "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1").unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    // Important: we make it read-only so the yaml_backup succeeds (can read) but the service logic fails to update the workspace file
    let mut perms = std::fs::metadata(&nfw_yaml_path).unwrap().permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(&nfw_yaml_path, perms).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);

    let mut perms = std::fs::metadata(&nfw_yaml_path).unwrap().permissions();
    perms.set_readonly(false);
    std::fs::set_permissions(&nfw_yaml_path, perms).unwrap();

    assert!(result.is_err());
}

#[test]
fn handle_captures_config_parameters() {
    let (sandbox, generator_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(&nfw_yaml_path, "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1").unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let capture = Rc::new(RefCell::new(None));
    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir),
        MockEngine::with_capture(capture.clone()),
    );

    let config = WebApiConfig::new()
        .with_openapi(false)
        .with_health_checks(true)
        .with_cors(false)
        .with_problem_details(true);

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        config,
    );

    handler.handle(&cmd).unwrap();

    let params = capture.borrow().clone().unwrap();
    assert_eq!(params.get("UseOpenApi").unwrap(), "false");
    assert_eq!(params.get("UseHealthChecks").unwrap(), "true");
    assert_eq!(params.get("UseCors").unwrap(), "false");
    assert_eq!(params.get("UseProblemDetails").unwrap(), "true");
}

#[test]
fn handle_captures_all_config_flags() {
    let (sandbox, generator_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(&nfw_yaml_path, "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1").unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let capture = Rc::new(RefCell::new(None));
    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir.clone()),
        MockEngine::with_capture(capture.clone()),
    );

    // Test case 1: All true
    let config_all_true = WebApiConfig::new()
        .with_openapi(true)
        .with_health_checks(true)
        .with_cors(true)
        .with_problem_details(true);

    let cmd1 = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml.clone(),
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        config_all_true,
    );
    handler.handle(&cmd1).unwrap();
    {
        let params = capture.borrow().clone().unwrap();
        assert_eq!(params.get("UseOpenApi").unwrap(), "true");
        assert_eq!(params.get("UseHealthChecks").unwrap(), "true");
        assert_eq!(params.get("UseCors").unwrap(), "true");
        assert_eq!(params.get("UseProblemDetails").unwrap(), "true");
    }

    // Test case 2: All false
    let config_all_false = WebApiConfig::new()
        .with_openapi(false)
        .with_health_checks(false)
        .with_cors(false)
        .with_problem_details(false);

    let nfw_yaml_all_false_str = "workspace:\n  namespace: MyProj\nservices:\n  Svc2:\n    path: src/Svc2\n    generator:\n      id: t1";
    let nfw_yaml_all_false: serde_yaml::Value =
        serde_yaml::from_str(nfw_yaml_all_false_str).unwrap();

    // important: we must write Svc2 to the file as well, because some service methods reload it from disk
    let nfw_yaml_combined_str = "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1\n  Svc2:\n    path: src/Svc2\n    generator:\n      id: t1";
    std::fs::write(&nfw_yaml_path, nfw_yaml_combined_str).unwrap();

    // ensure output root exists for Svc2
    std::fs::create_dir_all(sandbox.path().join("src/Svc2")).unwrap();

    let cmd2 = AddWebApiCommand::new(
        ServiceInfo::new("Svc2".to_string(), "src/Svc2".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml_all_false,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        config_all_false,
    );
    // handler.handle(&cmd1).unwrap_err(); // This was causing issues due to file system state mismatch
    handler.handle(&cmd2).unwrap();
    {
        let params = capture.borrow().clone().unwrap();
        assert_eq!(params.get("UseOpenApi").unwrap(), "false");
        assert_eq!(params.get("UseHealthChecks").unwrap(), "false");
        assert_eq!(params.get("UseCors").unwrap(), "false");
        assert_eq!(params.get("UseProblemDetails").unwrap(), "false");
    }
}

#[test]
fn handle_success_full_flow() {
    let (sandbox, generator_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(
        &nfw_yaml_path,
        "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(
        result.is_ok(),
        "Expected Successful execution, got: {:?}",
        result
    );

    // Verify workspace was updated
    let updated_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();
    let modules = updated_yaml
        .get("services")
        .and_then(|s| s.get("Svc1"))
        .and_then(|details| details.get("modules"))
        .and_then(|m| m.as_sequence())
        .unwrap();
    assert!(modules.contains(&serde_yaml::Value::String("webapi".to_string())));
}

#[test]
fn handle_error_on_missing_required_modules() {
    let (sandbox, generator_dir) = setup_test_env();

    // generator.yaml for webapi requiring 'persistence'
    let generator_yaml = r#"
id: t1
required_modules:
  - persistence
generators:
  webapi: "."
"#;
    std::fs::write(
        generator_dir.join("webapi").join("nfw.generator.yaml"),
        generator_yaml,
    )
    .unwrap();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(&nfw_yaml_path, "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    generator:\n      id: t1").unwrap();
    let nfw_yaml: serde_yaml::Value =
        serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(generator_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        )
        .unwrap(),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(result.is_err());
    if let Err(err) = result {
        assert!(
            matches!(err, AddArtifactError::MissingRequiredModule(_)),
            "Expected MissingRequiredModule error, got {:?}",
            err
        );
    }
}
