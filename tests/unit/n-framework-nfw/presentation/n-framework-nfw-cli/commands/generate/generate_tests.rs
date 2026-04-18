use std::path::{Path, PathBuf};

use n_framework_nfw_cli::commands::generate::{GenerateCliCommand, GenerateRequest};
use n_framework_nfw_core_application::features::template_management::commands::generate::generate_command_handler::GenerateCommandHandler;
use n_framework_nfw_core_application::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use serde_yaml::Value as YamlValue;
use tempfile::TempDir;

#[derive(Debug, Clone)]
struct MockWorkingDirectoryProvider {
    current_dir: PathBuf,
}

impl WorkingDirectoryProvider for MockWorkingDirectoryProvider {
    fn current_dir(&self) -> Result<PathBuf, String> {
        Ok(self.current_dir.clone())
    }
}

#[derive(Debug, Clone)]
struct MockTemplateRootResolver {
    template_root: Option<PathBuf>,
}

impl TemplateRootResolver for MockTemplateRootResolver {
    fn resolve(&self, _nfw_yaml: &YamlValue, _template_id: &str, _workspace_root: &Path) -> Result<PathBuf, String> {
        self.template_root.clone().ok_or_else(|| "template not found".to_string())
    }
}

#[derive(Debug, Clone)]
struct MockTemplateEngine {
    result: Result<(), TemplateError>,
}

impl MockTemplateEngine {
    fn success() -> Self {
        Self { result: Ok(()) }
    }

    fn failure(error: TemplateError) -> Self {
        Self { result: Err(error) }
    }
}

impl TemplateEngine for MockTemplateEngine {
    fn execute(
        &self,
        _config: &TemplateConfig,
        _template_root: &Path,
        _output_root: &Path,
        _parameters: &TemplateParameters,
    ) -> Result<(), TemplateError> {
        self.result.clone()
    }
}

fn create_command_with_mocks(
    current_dir: PathBuf,
    template_root: Option<PathBuf>,
    engine: MockTemplateEngine,
) -> GenerateCliCommand<MockWorkingDirectoryProvider, MockTemplateRootResolver, MockTemplateEngine> {
    let handler = GenerateCommandHandler::new(
        MockWorkingDirectoryProvider { current_dir },
        MockTemplateRootResolver { template_root },
        engine,
    );
    GenerateCliCommand::new(handler)
}

fn create_sandbox() -> TempDir {
    tempfile::tempdir().unwrap()
}

#[test]
fn execute_fails_on_invalid_name() {
    let sandbox = create_sandbox();
    let command = create_command_with_mocks(sandbox.path().to_path_buf(), None, MockTemplateEngine::success());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "Invalid Name",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("invalid name") || err.contains("invalid identifier"), "Error was: {}", err);
}

#[test]
fn execute_fails_on_invalid_feature() {
    let sandbox = create_sandbox();
    let command = create_command_with_mocks(sandbox.path().to_path_buf(), None, MockTemplateEngine::success());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: Some("Invalid Feature!"),
        params: None,
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("invalid feature") || err.contains("invalid identifier"), "Error was: {}", err);
}

#[test]
fn execute_fails_on_missing_nfw_yaml() {
    let sandbox = create_sandbox();
    let command = create_command_with_mocks(sandbox.path().to_path_buf(), None, MockTemplateEngine::success());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
}

#[test]
fn execute_fails_on_malformed_nfw_yaml() {
    let sandbox = create_sandbox();
    std::fs::write(sandbox.path().join("nfw.yaml"), "name: { invalid yaml").unwrap();

    let command = create_command_with_mocks(sandbox.path().to_path_buf(), None, MockTemplateEngine::success());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    // Error message comes from serde_yaml which might vary, but it should be a parsing error
}

#[test]
fn execute_fails_on_malformed_params() {
    let sandbox = create_sandbox();
    std::fs::write(sandbox.path().join("nfw.yaml"), "workspace:\n  name: test\n  namespace: App").unwrap();

    let command = create_command_with_mocks(sandbox.path().to_path_buf(), None, MockTemplateEngine::success());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: Some("InvalidParamFormat"),
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("parameter format"), "Error was: {}", err);
}

#[test]
fn execute_fails_on_empty_name() {
    let sandbox = create_sandbox();
    let command = create_command_with_mocks(sandbox.path().to_path_buf(), None, MockTemplateEngine::success());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
}

#[test]
fn execute_fails_on_missing_namespace_in_nfw_yaml() {
    let sandbox = create_sandbox();
    // nfw.yaml has no 'namespace' key
    std::fs::write(
        sandbox.path().join("nfw.yaml"),
        "name: test\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    )
    .unwrap();

    let template_root = sandbox.path().join("templates").join("mock-cmd");
    std::fs::create_dir_all(&template_root).unwrap();
    std::fs::write(
        template_root.join("template.yaml"),
        "id: mock-cmd\nsteps: []\n",
    )
    .unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        MockTemplateEngine::success()
    );

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("missing 'workspace.namespace'"),
        "Unexpected error: {}",
        err
    );
}

#[test]
fn execute_fails_on_param_without_value() {
    let sandbox = create_sandbox();
    std::fs::write(
        sandbox.path().join("nfw.yaml"),
        "workspace:\n  name: test\n  namespace: App",
    )
    .unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        None,
        MockTemplateEngine::success()
    );

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: Some("KeyOnly"),
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("parameter format"),
        "Unexpected error: {}",
        err
    );
}

#[test]
fn execute_fails_on_engine_error() {
    let sandbox = create_sandbox();
    std::fs::write(
        sandbox.path().join("nfw.yaml"),
        "workspace:\n  name: test\n  namespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    )
    .unwrap();

    let template_root = sandbox.path().join("templates").join("mock-cmd");
    std::fs::create_dir_all(&template_root).unwrap();
    std::fs::write(
        template_root.join("template.yaml"),
        "id: mock-cmd\nsteps:\n  - action: render\n    source: s\n    destination: d\n",
    )
    .unwrap();

    let engine = MockTemplateEngine::failure(TemplateError::TemplateRenderError {
        message: "simulated engine failure".to_string(),
        step_index: Some(0),
        template_id: Some("mock-cmd".to_string()),
        file_path: None,
        source: None,
    });
    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        engine
    );

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("simulated engine failure"),
        "Expected engine failure message but got: {}",
        err
    );
}

