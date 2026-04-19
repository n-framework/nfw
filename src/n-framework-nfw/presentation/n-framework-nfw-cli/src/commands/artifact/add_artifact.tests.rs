use super::*;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use serde_json::json;
use serde_yaml::Value as YamlValue;
use tempfile::TempDir;
use n_framework_core_cli_abstractions::{InteractiveError, InteractivePrompt, Logger, LoggingError, SelectOption, Spinner};
use n_framework_nfw_core_application::features::template_management::commands::add_artifact::add_artifact_command_handler::AddArtifactCommandHandler;
use n_framework_nfw_core_application::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::template_management::template_config::{TemplateConfig, TemplateInput, TemplateInputType};
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;

// --- Mocks and Spies ---

#[derive(Debug, Default)]
struct CallLog {
    calls: Vec<String>,
    responses: Vec<serde_json::Value>,
    next_response_index: usize,
}

#[derive(Debug, Clone)]
struct SpyPromptService {
    log: Arc<Mutex<CallLog>>,
    interactive: bool,
}

impl SpyPromptService {
    fn new(responses: Vec<serde_json::Value>) -> Self {
        Self {
            log: Arc::new(Mutex::new(CallLog {
                calls: Vec::new(),
                responses,
                next_response_index: 0,
            })),
            interactive: true,
        }
    }

    fn non_interactive() -> Self {
        Self {
            log: Arc::new(Mutex::new(CallLog::default())),
            interactive: false,
        }
    }

    fn get_calls(&self) -> Vec<String> {
        self.log.lock().unwrap().calls.clone()
    }
}

impl InteractivePrompt for SpyPromptService {
    fn is_interactive(&self) -> bool {
        self.interactive
    }

    fn text(&self, message: &str, _default: Option<&str>) -> Result<String, InteractiveError> {
        let mut log = self.log.lock().unwrap();
        log.calls.push(format!("text: {}", message));
        let resp = log
            .responses
            .get(log.next_response_index)
            .cloned()
            .unwrap_or(json!("default-text"));
        log.next_response_index += 1;
        Ok(resp.as_str().unwrap().to_string())
    }

    fn confirm(&self, message: &str, _default: bool) -> Result<bool, InteractiveError> {
        let mut log = self.log.lock().unwrap();
        log.calls.push(format!("confirm: {}", message));
        let resp = log
            .responses
            .get(log.next_response_index)
            .cloned()
            .unwrap_or(json!(true));
        log.next_response_index += 1;
        Ok(resp.as_bool().unwrap())
    }

    fn select(
        &self,
        message: &str,
        options: &[SelectOption],
        _default_idx: Option<usize>,
    ) -> Result<SelectOption, InteractiveError> {
        let mut log = self.log.lock().unwrap();
        log.calls.push(format!("select: {}", message));
        let resp = log
            .responses
            .get(log.next_response_index)
            .cloned()
            .unwrap_or(json!(options[0].value()));
        log.next_response_index += 1;
        let val = resp.as_str().unwrap();
        options
            .iter()
            .find(|o| o.value() == val)
            .cloned()
            .ok_or_else(|| InteractiveError::cancelled("not found"))
    }

    fn select_index(
        &self,
        message: &str,
        _options: &[SelectOption],
        _default_idx: Option<usize>,
    ) -> Result<usize, InteractiveError> {
        let mut log = self.log.lock().unwrap();
        log.calls.push(format!("select_index: {}", message));
        let resp = log
            .responses
            .get(log.next_response_index)
            .cloned()
            .unwrap_or(json!(0));
        log.next_response_index += 1;
        Ok(resp.as_u64().unwrap() as usize)
    }

    fn multiselect(
        &self,
        message: &str,
        options: &[SelectOption],
        _default_idxs: &[usize],
    ) -> Result<Vec<SelectOption>, InteractiveError> {
        let mut log = self.log.lock().unwrap();
        log.calls.push(format!("multiselect: {}", message));
        let resp = log
            .responses
            .get(log.next_response_index)
            .cloned()
            .unwrap_or(json!(Vec::<String>::new()));
        log.next_response_index += 1;
        let vals = resp.as_array().unwrap();
        let mut result = Vec::new();
        for v in vals {
            let s = v.as_str().unwrap();
            if let Some(opt) = options.iter().find(|o| o.value() == s) {
                result.push(opt.clone());
            }
        }
        Ok(result)
    }

    fn password(&self, message: &str) -> Result<String, InteractiveError> {
        let mut log = self.log.lock().unwrap();
        log.calls.push(format!("password: {}", message));
        let resp = log
            .responses
            .get(log.next_response_index)
            .cloned()
            .unwrap_or(json!("secret"));
        log.next_response_index += 1;
        Ok(resp.as_str().unwrap().to_string())
    }
}

impl Logger for SpyPromptService {
    fn intro(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn outro(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_cancel(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_info(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_step(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_success(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_warning(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn log_error(&self, _message: &str) -> Result<(), LoggingError> {
        Ok(())
    }
    fn spinner(&self, _message: &str) -> Result<Box<dyn Spinner>, LoggingError> {
        struct NoopSpinner;
        impl Spinner for NoopSpinner {
            fn stop(&self, _message: &str) {}
            fn success(&self, _message: &str) {}
            fn error(&self, _message: &str) {}
            fn cancel(&self, _message: &str) {}
            fn set_message(&self, _message: &str) {}
            fn is_finished(&self) -> bool {
                true
            }
        }
        Ok(Box::new(NoopSpinner))
    }
}

#[derive(Clone)]
struct MockWorkingDirectoryProvider {
    current_dir: PathBuf,
}
impl WorkingDirectoryProvider for MockWorkingDirectoryProvider {
    fn current_dir(&self) -> Result<PathBuf, String> {
        Ok(self.current_dir.clone())
    }
}

#[derive(Clone)]
struct MockTemplateRootResolver {
    template_root: Option<PathBuf>,
}
impl TemplateRootResolver for MockTemplateRootResolver {
    fn resolve(&self, _: &YamlValue, _: &str, _: &Path) -> Result<PathBuf, String> {
        self.template_root
            .clone()
            .ok_or_else(|| "not found".to_string())
    }
}

#[derive(Clone)]
struct MockTemplateEngine {
    result: Result<(), TemplateError>,
}
impl TemplateEngine for MockTemplateEngine {
    fn execute(
        &self,
        _: &TemplateConfig,
        _: &Path,
        _: &Path,
        _: &TemplateParameters,
    ) -> Result<(), TemplateError> {
        self.result.clone()
    }
}

// --- Helpers ---

fn create_command_with_mocks(
    current_dir: PathBuf,
    template_root: Option<PathBuf>,
    engine_result: Result<(), TemplateError>,
    prompt: SpyPromptService,
) -> AddArtifactCliCommand<
    MockWorkingDirectoryProvider,
    MockTemplateRootResolver,
    MockTemplateEngine,
    SpyPromptService,
> {
    let handler = AddArtifactCommandHandler::new(
        MockWorkingDirectoryProvider { current_dir },
        MockTemplateRootResolver { template_root },
        MockTemplateEngine {
            result: engine_result,
        },
    );
    AddArtifactCliCommand::new(handler, prompt)
}

fn create_sandbox() -> TempDir {
    tempfile::tempdir().unwrap()
}

fn no_input_request<'a>(
    generator_type: &'a str,
    name: Option<&'a str>,
    feature: Option<&'a str>,
    params: Option<&'a str>,
) -> AddArtifactRequest<'a> {
    AddArtifactRequest {
        generator_type,
        name,
        feature,
        params,
        param_json: None,
        no_input: true,
        is_interactive_terminal: false,
    }
}

macro_rules! assert_json_eq {
    ($left:expr, $right:expr) => {
        assert_eq!(
            serde_json::to_value($left).unwrap(),
            serde_json::to_value($right).unwrap()
        );
    };
}

// --- Tests ---

#[test]
fn test_text_prompt() {
    let input = TemplateInput::new("id".into(), TemplateInputType::Text, "Prompt".into());
    let spy = SpyPromptService::new(vec![json!("hello")]);
    let command = create_command_with_mocks(
        PathBuf::from("/tmp"),
        Some(PathBuf::from("/tmp")),
        Ok(()),
        spy.clone(),
    );
    let val = command.prompt_for_input(&input).unwrap();
    assert_eq!(val, json!("hello"));
    assert_eq!(spy.get_calls(), vec!["text: Prompt"]);
}

#[test]
fn test_password_prompt() {
    let input = TemplateInput::new("id".into(), TemplateInputType::Password, "Prompt".into());
    let spy = SpyPromptService::new(vec![json!("secret")]);
    let command = create_command_with_mocks(
        PathBuf::from("/tmp"),
        Some(PathBuf::from("/tmp")),
        Ok(()),
        spy.clone(),
    );
    let val = command.prompt_for_input(&input).unwrap();
    assert_eq!(val, json!("secret"));
    assert_eq!(spy.get_calls(), vec!["password: Prompt"]);
}

#[test]
fn test_confirm_prompt() {
    let input = TemplateInput::new("id".into(), TemplateInputType::Confirm, "Prompt".into());
    let spy = SpyPromptService::new(vec![json!(true)]);
    let command = create_command_with_mocks(
        PathBuf::from("/tmp"),
        Some(PathBuf::from("/tmp")),
        Ok(()),
        spy.clone(),
    );
    let val = command.prompt_for_input(&input).unwrap();
    assert_eq!(val, json!(true));
    assert_eq!(spy.get_calls(), vec!["confirm: Prompt"]);
}

#[test]
fn test_select_prompt() {
    let input = TemplateInput::new("id".into(), TemplateInputType::Select, "Prompt".into())
        .with_options(vec!["opt1".into(), "opt2".into()]);
    let spy = SpyPromptService::new(vec![json!("opt2")]);
    let command = create_command_with_mocks(
        PathBuf::from("/tmp"),
        Some(PathBuf::from("/tmp")),
        Ok(()),
        spy.clone(),
    );
    let val = command.prompt_for_input(&input).unwrap();
    assert_eq!(val, json!("opt2"));
    assert_eq!(spy.get_calls(), vec!["select: Prompt"]);
}

#[test]
fn test_multiselect_prompt() {
    let input = TemplateInput::new("id".into(), TemplateInputType::Multiselect, "Prompt".into())
        .with_options(vec!["opt1".into(), "opt2".into(), "opt3".into()]);
    let spy = SpyPromptService::new(vec![json!(vec!["opt1", "opt2"])]);
    let command = create_command_with_mocks(
        PathBuf::from("/tmp"),
        Some(PathBuf::from("/tmp")),
        Ok(()),
        spy.clone(),
    );
    let val = command.prompt_for_input(&input).unwrap();
    assert_eq!(val, json!(vec!["opt1", "opt2"]));
    assert_eq!(spy.get_calls(), vec!["multiselect: Prompt"]);
}

#[test]
fn test_object_prompt_recursive() {
    let input = TemplateInput::new(
        "obj".to_string(),
        TemplateInputType::Object,
        "ignored".to_string(),
    )
    .with_properties(vec![
        TemplateInput::new(
            "p1".to_string(),
            TemplateInputType::Text,
            "P1 Prompt".to_string(),
        ),
        TemplateInput::new(
            "p2".to_string(),
            TemplateInputType::Text,
            "P2 Prompt".to_string(),
        ),
    ]);
    let spy = SpyPromptService::new(vec![json!("val1"), json!("val2")]);
    let command = create_command_with_mocks(
        PathBuf::from("/tmp"),
        Some(PathBuf::from("/tmp")),
        Ok(()),
        spy.clone(),
    );
    let val = command.prompt_for_input(&input).unwrap();
    assert_json_eq!(val, json!({ "p1": "val1", "p2": "val2" }));
    assert_eq!(spy.get_calls(), vec!["text: P1 Prompt", "text: P2 Prompt"]);
}

#[test]
fn execute_fails_on_invalid_name() {
    let sandbox = create_sandbox();
    std::fs::write(sandbox.path().join("nfw.yaml"), "workspace:\n  name: test\n  namespace: App\nservices:\n  TestService:\n    path: src/TestService\n    template:\n      id: mock-service\ntemplate_sources:\n  local: templates\n").unwrap();
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
        Ok(()),
        SpyPromptService::non_interactive(),
    );
    let result = command.execute(no_input_request(
        "command",
        Some("Invalid Name"),
        None,
        None,
    ));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid"));
}

#[test]
fn execute_supports_quoted_commas_in_params() {
    let sandbox = create_sandbox();
    std::fs::write(sandbox.path().join("nfw.yaml"), "workspace:\n  name: test\n  namespace: App\nservices:\n  TestService:\n    path: src/TestService\n    template:\n      id: mock-service\ntemplate_sources:\n  local: templates\n").unwrap();
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
        Ok(()),
        SpyPromptService::non_interactive(),
    );
    let result = command.execute(AddArtifactRequest {
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
fn execute_fails_on_param_conflict() {
    let sandbox = create_sandbox();
    std::fs::write(sandbox.path().join("nfw.yaml"), "workspace:\n  name: test\n  namespace: App\nservices:\n  TestService:\n    path: src/TestService\n    template:\n      id: mock-service\ntemplate_sources:\n  local: templates\n").unwrap();
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
        Ok(()),
        SpyPromptService::non_interactive(),
    );
    let result = command.execute(AddArtifactRequest {
        generator_type: "command",
        name: Some("ValidName"),
        feature: None,
        params: Some("Conflict=Value1"),
        param_json: Some("{\"Conflict\": \"Value2\"}"),
        no_input: true,
        is_interactive_terminal: false,
    });
    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("parameter conflict")
    );
}
