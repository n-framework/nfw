use super::*;
use crate::features::template_management::commands::add_webapi::add_webapi_command::WebApiConfig;
use crate::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use std::path::{Path, PathBuf};
use tempfile;

struct MockEngine {
    fail_execution: bool,
}
impl TemplateEngine for MockEngine {
    fn execute(
        &self,
        _config: &TemplateConfig,
        _root: &Path,
        _output: &Path,
        _params: &TemplateParameters,
    ) -> Result<(), TemplateError> {
        if self.fail_execution {
            Err(TemplateError::io("mock error", PathBuf::from("mock")))
        } else {
            Ok(())
        }
    }
}

#[test]
fn given_mock_engine_fails_when_handle_then_returns_execution_failed_error() {
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

    struct SandboxWorkingDir(PathBuf);
    impl WorkingDirectoryProvider for SandboxWorkingDir {
        fn current_dir(&self) -> Result<PathBuf, String> {
            Ok(self.0.clone())
        }
    }

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
        MockEngine {
            fail_execution: true,
        },
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
fn given_namespace_missing_in_yaml_when_handle_then_returns_config_error() {
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

    struct SandboxWorkingDir(PathBuf);
    impl WorkingDirectoryProvider for SandboxWorkingDir {
        fn current_dir(&self) -> Result<PathBuf, String> {
            Ok(self.0.clone())
        }
    }

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
        MockEngine {
            fail_execution: false,
        },
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
fn given_webapi_module_exists_when_handle_then_returns_workspace_error() {
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

    struct SandboxWorkingDir(PathBuf);
    impl WorkingDirectoryProvider for SandboxWorkingDir {
        fn current_dir(&self) -> Result<PathBuf, String> {
            Ok(self.0.clone())
        }
    }

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
        MockEngine {
            fail_execution: false,
        },
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
fn given_template_resolver_fails_when_handle_then_returns_template_not_found_error() {
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

    #[allow(dead_code)]
    struct FailingResolver(PathBuf);
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

    let nfw_yaml_path = sandbox.path().join("nfw.yaml");
    std::fs::write(
        &nfw_yaml_path,
        "workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1",
    )
    .unwrap();
    let nfw_yaml = serde_yaml::from_str(&std::fs::read_to_string(&nfw_yaml_path).unwrap()).unwrap();

    let handler = AddWebApiCommandHandler::new(
        SandboxWorkingDir(sandbox.path().to_path_buf()),
        FailingResolver(template_dir),
        MockEngine {
            fail_execution: false,
        },
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
