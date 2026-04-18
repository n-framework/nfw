use std::path::{Path, PathBuf};

use n_framework_core_cli_abstractions::{PromptError, PromptService, SelectOption};
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
    fn resolve(
        &self,
        _nfw_yaml: &YamlValue,
        _template_id: &str,
        _workspace_root: &Path,
    ) -> Result<PathBuf, String> {
        self.template_root
            .clone()
            .ok_or_else(|| "template not found".to_string())
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

#[derive(Debug, Clone)]
struct MockPromptService;

impl PromptService for MockPromptService {
    fn is_interactive(&self) -> bool {
        false
    }

    fn text(&self, _message: &str, _default: Option<&str>) -> Result<String, PromptError> {
        Ok(String::new())
    }

    fn confirm(&self, _message: &str, default: bool) -> Result<bool, PromptError> {
        Ok(default)
    }

    fn password(&self, _message: &str) -> Result<String, PromptError> {
        Ok(String::new())
    }

    fn select(
        &self,
        _message: &str,
        options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<SelectOption, PromptError> {
        let index = default_index.unwrap_or(0);
        options
            .get(index)
            .cloned()
            .ok_or_else(|| PromptError::cancelled("no options available"))
    }

    fn select_index(
        &self,
        _message: &str,
        _options: &[SelectOption],
        default_index: Option<usize>,
    ) -> Result<usize, PromptError> {
        Ok(default_index.unwrap_or(0))
    }

    fn multiselect(
        &self,
        _message: &str,
        options: &[SelectOption],
        default_indices: &[usize],
    ) -> Result<Vec<SelectOption>, PromptError> {
        let mut selected = Vec::new();
        for &index in default_indices {
            if let Some(opt) = options.get(index) {
                selected.push(opt.clone());
            }
        }
        Ok(selected)
    }
}

fn create_command_with_mocks(
    current_dir: PathBuf,
    template_root: Option<PathBuf>,
    engine: MockTemplateEngine,
) -> GenerateCliCommand<
    MockWorkingDirectoryProvider,
    MockTemplateRootResolver,
    MockTemplateEngine,
    MockPromptService,
> {
    let handler = GenerateCommandHandler::new(
        MockWorkingDirectoryProvider { current_dir },
        MockTemplateRootResolver { template_root },
        engine,
    );
    GenerateCliCommand::new(handler, MockPromptService)
}

fn create_sandbox() -> TempDir {
    tempfile::tempdir().unwrap()
}

fn no_input_request<'a>(
    generator_type: &'a str,
    name: Option<&'a str>,
    feature: Option<&'a str>,
    params: Option<&'a str>,
) -> GenerateRequest<'a> {
    GenerateRequest {
        generator_type,
        name,
        feature,
        params,
        param_json: None,
        no_input: true,
        is_interactive_terminal: false,
    }
}

#[test]
fn execute_fails_on_invalid_name() {
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
        "id: mock-cmd\nsteps: []\n",
    )
    .unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        MockTemplateEngine::success(),
    );

    let result = command.execute(no_input_request(
        "command",
        Some("Invalid Name"),
        None,
        None,
    ));

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("invalid name") || err.contains("invalid identifier"),
        "Error was: {}",
        err
    );
}

#[test]
fn execute_fails_on_invalid_feature() {
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
        "id: mock-cmd\nsteps: []\n",
    )
    .unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        MockTemplateEngine::success(),
    );

    let result = command.execute(no_input_request(
        "command",
        Some("ValidName"),
        Some("Invalid Feature!"),
        None,
    ));

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("invalid feature") || err.contains("invalid identifier"),
        "Error was: {}",
        err
    );
}

#[test]
fn execute_fails_on_missing_nfw_yaml() {
    let sandbox = create_sandbox();
    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        None,
        MockTemplateEngine::success(),
    );

    let result = command.execute(no_input_request("command", Some("ValidName"), None, None));

    assert!(result.is_err());
}

#[test]
fn execute_fails_on_malformed_nfw_yaml() {
    let sandbox = create_sandbox();
    std::fs::write(sandbox.path().join("nfw.yaml"), "name: { invalid yaml").unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        None,
        MockTemplateEngine::success(),
    );

    let result = command.execute(no_input_request("command", Some("ValidName"), None, None));

    assert!(result.is_err());
    // Error message comes from serde_yaml which might vary, but it should be a parsing error
}

#[test]
fn execute_fails_on_malformed_params() {
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
        "id: mock-cmd\nsteps: []\n",
    )
    .unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        MockTemplateEngine::success(),
    );

    let result = command.execute(no_input_request(
        "command",
        Some("ValidName"),
        None,
        Some("InvalidParamFormat"),
    ));

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("parameter format"), "Error was: {}", err);
}

#[test]
fn execute_fails_on_empty_name() {
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
        "id: mock-cmd\nsteps: []\n",
    )
    .unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        MockTemplateEngine::success(),
    );

    // name=None in non-interactive mode triggers a clear error
    let result = command.execute(no_input_request("command", None, None, None));

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("name is required"), "Error was: {}", err);
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
        MockTemplateEngine::success(),
    );

    let result = command.execute(no_input_request("command", Some("ValidName"), None, None));

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
        "workspace:\n  name: test\n  namespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
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
        MockTemplateEngine::success(),
    );

    let result = command.execute(no_input_request(
        "command",
        Some("ValidName"),
        None,
        Some("KeyOnly"),
    ));

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
    let command =
        create_command_with_mocks(sandbox.path().to_path_buf(), Some(template_root), engine);

    let result = command.execute(no_input_request("command", Some("ValidName"), None, None));

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("simulated engine failure"),
        "Expected engine failure message but got: {}",
        err
    );
}

#[test]
fn execute_supports_quoted_commas_in_params() {
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
        "id: mock-cmd\nsteps: []\n",
    )
    .unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        MockTemplateEngine::success(),
    );

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: Some("ValidName"),
        feature: None,
        params: Some("Key1=Value1,Key2=\"Value with, comma\""),
        param_json: None,
        no_input: true,
        is_interactive_terminal: false,
    });

    assert!(result.is_ok());
}

#[test]
fn execute_supports_param_json() {
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
        "id: mock-cmd\nsteps: []\n",
    )
    .unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        MockTemplateEngine::success(),
    );

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: Some("ValidName"),
        feature: None,
        params: None,
        param_json: Some("{\"Complex\": {\"Target\": 123}}"),
        no_input: true,
        is_interactive_terminal: false,
    });

    assert!(result.is_ok());
}

#[test]
fn template_parameters_builders_validate_empty_input() {
    let params = TemplateParameters::new();

    assert!(params.clone().with_name("").is_err());
    assert!(params.clone().with_name("  ").is_err());
    assert!(params.clone().with_feature("").is_err());
    assert!(params.clone().with_namespace("").is_err());

    let valid = params.with_name("Test").unwrap();
    assert_eq!(valid.name(), Some("Test"));
}

#[test]
fn execute_fails_on_param_conflict() {
    let sandbox = create_sandbox();
    std::fs::write(
        sandbox.path().join("nfw.yaml"),
        "workspace:\n  name: test\n  namespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    ).unwrap();

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
        MockTemplateEngine::success(),
    );

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: Some("ValidName"),
        feature: None,
        params: Some("Conflict=Value1"),
        param_json: Some("{\"Conflict\": \"Value2\"}"),
        no_input: true,
        is_interactive_terminal: false,
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("parameter conflict"), "Error was: {}", err);
    assert!(err.contains("'Conflict'"), "Error was: {}", err);
}

#[test]
fn execute_fails_on_malformed_param_json() {
    let sandbox = create_sandbox();
    std::fs::write(
        sandbox.path().join("nfw.yaml"),
        "workspace:\n  name: test\n  namespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    ).unwrap();

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
        MockTemplateEngine::success(),
    );

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: Some("ValidName"),
        feature: None,
        params: None,
        param_json: Some("{ invalid json"),
        no_input: true,
        is_interactive_terminal: false,
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("invalid JSON in --param-json"),
        "Error was: {}",
        err
    );
}

#[test]
fn execute_fails_on_empty_select_options() {
    let sandbox = create_sandbox();
    std::fs::write(
        sandbox.path().join("nfw.yaml"),
        "workspace:\n  name: test\n  namespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    ).unwrap();

    let template_root = sandbox.path().join("templates").join("mock-cmd");
    std::fs::create_dir_all(&template_root).unwrap();
    // Select input with empty options should fail validation
    std::fs::write(
        template_root.join("template.yaml"),
        "id: mock-cmd\ninputs:\n  - id: my-select\n    type: select\n    prompt: Choose\n    options: []\nsteps: []\n",
    ).unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        MockTemplateEngine::success(),
    );

    let result = command.execute(no_input_request(
        "command",
        Some("ValidName"),
        None,
        Some("my-select=val"),
    ));

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("empty options list"), "Error was: {}", err);
}

#[test]
fn execute_fails_on_missing_required_params_no_input() {
    let sandbox = create_sandbox();
    std::fs::write(
        sandbox.path().join("nfw.yaml"),
        "workspace:\n  name: test\n  namespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    ).unwrap();

    let template_root = sandbox.path().join("templates").join("mock-cmd");
    std::fs::create_dir_all(&template_root).unwrap();
    std::fs::write(
        template_root.join("template.yaml"),
        "id: mock-cmd\ninputs:\n  - id: required-field\n    type: text\n    prompt: Enter something\nsteps: []\n",
    ).unwrap();

    let command = create_command_with_mocks(
        sandbox.path().to_path_buf(),
        Some(template_root),
        MockTemplateEngine::success(),
    );

    // Missing 'required-field' in --no-input mode should fail
    let result = command.execute(no_input_request("command", Some("ValidName"), None, None));

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("required parameter 'required-field' was not provided"),
        "Error was: {}",
        err
    );
}
