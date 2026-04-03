use nframework_nfw_application::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use nframework_nfw_application::features::workspace_management::models::new_command_request::NewCommandRequest;
use nframework_nfw_application::features::workspace_management::services::abstraction::prompt_service::PromptService;
use nframework_nfw_application::features::workspace_management::services::abstraction::workspace_name_validator::WorkspaceNameValidator;
use nframework_nfw_application::features::workspace_management::services::input_resolution_service::InputResolutionService;

#[derive(Debug, Clone)]
struct StubPromptService {
    interactive: bool,
    answer: Option<String>,
    failure: Option<String>,
}

impl PromptService for StubPromptService {
    fn is_interactive(&self) -> bool {
        self.interactive
    }

    fn prompt(&self, _message: &str) -> Result<String, String> {
        if let Some(failure) = &self.failure {
            return Err(failure.clone());
        }

        Ok(self.answer.clone().unwrap_or_default())
    }
}

#[derive(Debug, Clone, Default)]
struct StubWorkspaceNameValidator;

impl WorkspaceNameValidator for StubWorkspaceNameValidator {
    fn is_valid_workspace_name(&self, value: &str) -> bool {
        !value.trim().is_empty()
            && value
                .chars()
                .all(|character| character.is_ascii_alphanumeric() || character == '-')
    }
}

#[test]
fn resolves_explicit_workspace_name_without_prompting() {
    let service = InputResolutionService::new(
        StubPromptService {
            interactive: true,
            answer: Some("ignored".to_owned()),
            failure: None,
        },
        StubWorkspaceNameValidator,
    );
    let request = NewCommandRequest::new(
        Some("billing-platform".to_owned()),
        Some("official/blank-workspace".to_owned()),
        false,
        true,
    );

    let result = service.resolve_workspace_name(&request);

    assert_eq!(result, Ok("billing-platform".to_owned()));
}

#[test]
fn prompts_for_missing_workspace_name_in_interactive_mode() {
    let service = InputResolutionService::new(
        StubPromptService {
            interactive: true,
            answer: Some("order-platform".to_owned()),
            failure: None,
        },
        StubWorkspaceNameValidator,
    );
    let request = NewCommandRequest::new(
        None,
        Some("official/blank-workspace".to_owned()),
        false,
        true,
    );

    let result = service.resolve_workspace_name(&request);

    assert_eq!(result, Ok("order-platform".to_owned()));
}

#[test]
fn fails_fast_when_no_input_is_enabled_and_workspace_name_is_missing() {
    let service = InputResolutionService::new(
        StubPromptService {
            interactive: true,
            answer: Some("ignored".to_owned()),
            failure: None,
        },
        StubWorkspaceNameValidator,
    );
    let request = NewCommandRequest::new(
        None,
        Some("official/blank-workspace".to_owned()),
        true,
        false,
    );

    let result = service.resolve_workspace_name(&request);

    assert_eq!(result, Err(WorkspaceNewError::MissingWorkspaceName));
}

#[test]
fn rejects_invalid_workspace_name_from_prompt_response() {
    let service = InputResolutionService::new(
        StubPromptService {
            interactive: true,
            answer: Some("invalid name".to_owned()),
            failure: None,
        },
        StubWorkspaceNameValidator,
    );
    let request = NewCommandRequest::new(
        None,
        Some("official/blank-workspace".to_owned()),
        false,
        true,
    );

    let result = service.resolve_workspace_name(&request);

    assert_eq!(
        result,
        Err(WorkspaceNewError::InvalidWorkspaceName(
            "invalid name".to_owned()
        ))
    );
}
