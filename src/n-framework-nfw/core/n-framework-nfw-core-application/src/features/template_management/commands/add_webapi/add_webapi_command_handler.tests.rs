use super::*;
use crate::features::template_management::commands::add_webapi::add_webapi_command::WebApiConfig;
use crate::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use std::path::{Path, PathBuf};
use tempfile;

use std::cell::RefCell;
use std::rc::Rc;

struct MockEngine {
    fail_execution: bool,
    captured_params: Option<Rc<RefCell<Option<TemplateParameters>>>>,
}

impl MockEngine {
    fn new(fail: bool) -> Self {
        Self {
            fail_execution: fail,
            captured_params: None,
        }
    }
    fn with_capture(capture: Rc<RefCell<Option<TemplateParameters>>>) -> Self {
        Self {
            fail_execution: false,
            captured_params: Some(capture),
        }
    }
}

impl TemplateEngine for MockEngine {
    fn execute(
        &self,
        _config: &TemplateConfig,
        _root: &Path,
        _output: &Path,
        params: &TemplateParameters,
    ) -> Result<(), TemplateError> {
        if let Some(capture) = &self.captured_params {
            *capture.borrow_mut() = Some(params.clone());
        }
        if self.fail_execution {
            Err(TemplateError::io("mock error", PathBuf::from("mock")))
        } else {
            Ok(())
        }
    }
}

struct LocalMockResolver(PathBuf);
impl TemplateRootResolver for LocalMockResolver {
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
impl TemplateRootResolver for FailingResolver {
    fn resolve(
        &self,
        _yaml: &serde_yaml::Value,
        _id: &str,
        _root: &Path,
    ) -> Result<PathBuf, String> {
        Err("Template not found".to_string())
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
    let template_dir = sandbox.path().join("my-template");
    let sub_template_dir = template_dir.join("webapi");
    std::fs::create_dir_all(&sub_template_dir).unwrap();

    let template_yaml = r#"
id: my-template
generators:
  webapi: "webapi"
"#;
    std::fs::write(template_dir.join("template.yaml"), template_yaml).unwrap();
    std::fs::write(sub_template_dir.join("template.yaml"), template_yaml).unwrap();

    (sandbox, template_dir)
}

#[test]
fn handle_error_on_engine_failure() {
    let (sandbox, template_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(
        &nfw_yaml_path,
        "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml = serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(template_dir),
        MockEngine::new(true),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        ),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(matches!(result, Err(AddArtifactError::ExecutionFailed(_))));
}

#[test]
fn handle_error_on_missing_namespace() {
    let (sandbox, template_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(
        &nfw_yaml_path,
        "workspace:\nname: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml = serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(template_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        ),
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
    let (sandbox, template_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(
        &nfw_yaml_path,
        "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    modules:\n      - webapi\n    template:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml = serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(template_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        ),
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
        "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml = serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

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
        ),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(result.is_err());
    if let Err(ref err) = result {
        assert!(
            matches!(err, AddArtifactError::TemplateNotFound(_)),
            "Expected TemplateNotFound for template loading failure, got: {:?}",
            err
        );
    }
}

#[test]
fn handle_error_on_missing_yaml() {
    let (sandbox, template_dir) = setup_test_env();

    // delete nfw.yaml to force YamlBackup to fail
    let _nfw_yaml_path = sandbox.path().join("nfw.yaml");

    // But we need a valid nfw_yaml value for workspace context, so we parse a string
    let yaml_str = "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1";
    let nfw_yaml: serde_yaml::Value = serde_yaml::from_str(yaml_str).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(template_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        ),
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
fn handle_error_on_missing_sub_template() {
    let (sandbox, template_dir) = setup_test_env();

    // remove sub template to trigger loading failure
    let sub_template_dir = template_dir.join("webapi");
    std::fs::remove_file(sub_template_dir.join("template.yaml")).unwrap();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(&nfw_yaml_path, "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1").unwrap();
    let nfw_yaml = serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(template_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        ),
        WebApiConfig::default(),
    );

    let result = handler.handle(&cmd);
    assert!(result.is_err());
    // Should fail with config error for missing template.yaml in subfolder
}

#[test]
fn handle_error_on_readonly_yaml() {
    let (sandbox, template_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(&nfw_yaml_path, "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1").unwrap();
    let nfw_yaml = serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    // Important: we make it read-only so the yaml_backup succeeds (can read) but the service logic fails to update the workspace file
    let mut perms = std::fs::metadata(&nfw_yaml_path).unwrap().permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(&nfw_yaml_path, perms).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(template_dir),
        MockEngine::new(false),
    );

    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            sandbox.path().to_path_buf(),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        ),
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
    let (sandbox, template_dir) = setup_test_env();

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(&nfw_yaml_path, "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1").unwrap();
    let nfw_yaml = serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let capture = Rc::new(RefCell::new(None));
    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        LocalMockResolver(template_dir),
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
        ),
        config,
    );

    handler.handle(&cmd).unwrap();

    let params = capture.borrow().clone().unwrap();
    assert_eq!(params.get("UseOpenApi").unwrap(), "false");
    assert_eq!(params.get("UseHealthChecks").unwrap(), "true");
    assert_eq!(params.get("UseCors").unwrap(), "false");
    assert_eq!(params.get("UseProblemDetails").unwrap(), "true");
}
