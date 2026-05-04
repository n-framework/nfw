use super::*;
use crate::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use std::path::{Path, PathBuf};
use tempfile;

struct MockWorkingDir;
impl WorkingDirectoryProvider for MockWorkingDir {
    fn current_dir(&self) -> Result<PathBuf, String> {
        Ok(PathBuf::from("/mock/workspace"))
    }
}

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
fn handle_returns_error_when_engine_fails() {
    let sandbox = tempfile::tempdir().unwrap();
    let template_dir = sandbox.path().join("my-template");
    let sub_template_dir = template_dir.join("persistence");
    std::fs::create_dir_all(&sub_template_dir).unwrap();

    let template_yaml = r#"
id: my-template
generators:
  persistence: "persistence"
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

    let handler = AddPersistenceCommandHandler::new(
        MockWorkingDir,
        LocalMockResolver(template_dir),
        MockEngine {
            fail_execution: true,
        },
    );

    let nfw_yaml = serde_yaml::from_str("workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1").unwrap();
    let cmd = AddPersistenceCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            PathBuf::from("/mock/workspace"),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        ),
    )
    .unwrap();

    let result = handler.handle(&cmd);
    assert!(matches!(result, Err(AddArtifactError::ExecutionFailed(_))));
}
