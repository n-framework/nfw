use std::path::PathBuf;
use n_framework_core_cli_abstractions::{PromptError, PromptService, SelectOption};
use n_framework_nfw_cli::commands::generate::GenerateCliCommand;
use n_framework_nfw_core_application::features::template_management::commands::generate::generate_command_handler::GenerateCommandHandler;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_root_resolver::TemplateRootResolver;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_domain::features::template_management::template_config::{TemplateConfig, TemplateInput, TemplateInputType};
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use n_framework_nfw_core_application::features::template_management::models::template_error::TemplateError;
use serde_json::json;
use std::sync::{Arc, Mutex};

#[derive(Debug, Default)]
struct CallLog {
    calls: Vec<String>,
    responses: Vec<serde_json::Value>,
    next_response_index: usize,
}

#[derive(Debug, Clone)]
struct SpyPromptService {
    log: Arc<Mutex<CallLog>>,
}

impl SpyPromptService {
    fn new(responses: Vec<serde_json::Value>) -> Self {
        Self {
            log: Arc::new(Mutex::new(CallLog {
                calls: Vec::new(),
                responses,
                next_response_index: 0,
            })),
        }
    }

    fn get_calls(&self) -> Vec<String> {
        self.log.lock().unwrap().calls.clone()
    }
}

impl PromptService for SpyPromptService {
    fn is_interactive(&self) -> bool {
        true
    }

    fn text(&self, message: &str, _default: Option<&str>) -> Result<String, PromptError> {
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

    fn confirm(&self, message: &str, _default: bool) -> Result<bool, PromptError> {
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
    ) -> Result<SelectOption, PromptError> {
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
            .ok_or_else(|| PromptError::cancelled("not found"))
    }

    fn select_index(
        &self,
        message: &str,
        _options: &[SelectOption],
        _default_idx: Option<usize>,
    ) -> Result<usize, PromptError> {
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
    ) -> Result<Vec<SelectOption>, PromptError> {
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

    fn password(&self, message: &str) -> Result<String, PromptError> {
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

// Minimal mocks for handler dependencies
#[allow(dead_code)]
#[derive(Clone)]
struct MockW;
impl WorkingDirectoryProvider for MockW {
    fn current_dir(&self) -> Result<PathBuf, String> {
        Ok(PathBuf::from("/tmp"))
    }
}
#[allow(dead_code)]
#[derive(Clone)]
struct MockR;
impl TemplateRootResolver for MockR {
    fn resolve(
        &self,
        _: &serde_yaml::Value,
        _: &str,
        _: &std::path::Path,
    ) -> Result<PathBuf, String> {
        Ok(PathBuf::from("/tmp"))
    }
}
#[allow(dead_code)]
#[derive(Clone)]
struct MockE {
    result: Result<(), TemplateError>,
}
impl TemplateEngine for MockE {
    fn execute(
        &self,
        _: &TemplateConfig,
        _: &std::path::Path,
        _: &std::path::Path,
        _: &TemplateParameters,
    ) -> Result<(), TemplateError> {
        self.result.clone()
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

fn run_prompt_test(
    input: &TemplateInput,
    responses: Vec<serde_json::Value>,
) -> (serde_json::Value, Vec<String>) {
    let spy = SpyPromptService::new(responses);
    let handler = GenerateCommandHandler::new(MockW, MockR, MockE { result: Ok(()) });
    let cli = GenerateCliCommand::new(handler, spy.clone());
    let result = cli.prompt_for_input(input).unwrap();
    (serde_json::to_value(result).unwrap(), spy.get_calls())
}

#[test]
fn test_text_prompt() {
    let input = TemplateInput {
        id: Some("id".into()),
        input_type: TemplateInputType::Text,
        prompt: "Prompt".into(),
        default: None,
        options: None,
        properties: None,
        items: None,
    };
    let (val, calls) = run_prompt_test(&input, vec![json!("hello")]);
    assert_eq!(val, json!("hello"));
    assert_eq!(calls, vec!["text: Prompt"]);
}

#[test]
fn test_password_prompt() {
    let input = TemplateInput {
        id: Some("id".into()),
        input_type: TemplateInputType::Password,
        prompt: "Prompt".into(),
        default: None,
        options: None,
        properties: None,
        items: None,
    };
    let (val, calls) = run_prompt_test(&input, vec![json!("secret")]);
    assert_eq!(val, json!("secret"));
    assert_eq!(calls, vec!["password: Prompt"]);
}

#[test]
fn test_confirm_prompt() {
    let input = TemplateInput {
        id: Some("id".into()),
        input_type: TemplateInputType::Confirm,
        prompt: "Prompt".into(),
        default: None,
        options: None,
        properties: None,
        items: None,
    };
    let (val, calls) = run_prompt_test(&input, vec![json!(true)]);
    assert_eq!(val, json!(true));
    assert_eq!(calls, vec!["confirm: Prompt"]);
}

#[test]
fn test_select_prompt() {
    let input = TemplateInput {
        id: Some("id".into()),
        input_type: TemplateInputType::Select,
        prompt: "Prompt".into(),
        default: None,
        options: Some(vec!["opt1".into(), "opt2".into()]),
        properties: None,
        items: None,
    };
    let (val, calls) = run_prompt_test(&input, vec![json!("opt2")]);
    assert_eq!(val, json!("opt2"));
    assert_eq!(calls, vec!["select: Prompt"]);
}

#[test]
fn test_multiselect_prompt() {
    let input = TemplateInput {
        id: Some("id".into()),
        input_type: TemplateInputType::Multiselect,
        prompt: "Prompt".into(),
        default: None,
        options: Some(vec!["opt1".into(), "opt2".into(), "opt3".into()]),
        properties: None,
        items: None,
    };
    let (val, calls) = run_prompt_test(&input, vec![json!(vec!["opt1", "opt2"])]);
    assert_eq!(val, json!(vec!["opt1", "opt2"]));
    assert_eq!(calls, vec!["multiselect: Prompt"]);
}

#[test]
fn test_object_prompt_recursive() {
    let input = TemplateInput {
        id: Some("obj".to_string()),
        input_type: TemplateInputType::Object,
        prompt: "ignored".to_string(),
        default: None,
        options: None,
        properties: Some(vec![
            TemplateInput {
                id: Some("p1".to_string()),
                input_type: TemplateInputType::Text,
                prompt: "P1 Prompt".to_string(),
                default: None,
                options: None,
                properties: None,
                items: None,
            },
            TemplateInput {
                id: Some("p2".to_string()),
                input_type: TemplateInputType::Text,
                prompt: "P2 Prompt".to_string(),
                default: None,
                options: None,
                properties: None,
                items: None,
            },
        ]),
        items: None,
    };

    let (val, calls) = run_prompt_test(&input, vec![json!("val1"), json!("val2")]);
    assert_json_eq!(val, json!({ "p1": "val1", "p2": "val2" }));
    assert_eq!(calls, vec!["text: P1 Prompt", "text: P2 Prompt"]);
}

#[test]
fn test_list_prompt_dynamic() {
    let input = TemplateInput {
        id: Some("list".to_string()),
        input_type: TemplateInputType::List,
        prompt: "List Prompt".to_string(),
        default: None,
        options: None,
        properties: None,
        items: Some(Box::new(TemplateInput {
            id: None,
            input_type: TemplateInputType::Text,
            prompt: "Item Prompt".to_string(),
            default: None,
            options: None,
            properties: None,
            items: None,
        })),
    };

    let (val, calls) = run_prompt_test(
        &input,
        vec![
            json!(true),
            json!("item1"),
            json!(true),
            json!("item2"),
            json!(false),
        ],
    );
    assert_json_eq!(val, json!(vec!["item1", "item2"]));
    assert_eq!(
        calls,
        vec![
            "confirm: Add an item to List Prompt?",
            "text: Item Prompt",
            "confirm: Add an item to List Prompt?",
            "text: Item Prompt",
            "confirm: Add an item to List Prompt?"
        ]
    );
}
