use n_framework_core_cli_abstractions::{PromptError, PromptService, SelectOption};
use n_framework_nfw_core_application::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use n_framework_nfw_core_application::features::workspace_management::models::new_command_request::NewCommandRequest;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::workspace_name_validator::WorkspaceNameValidator;
use n_framework_nfw_core_application::features::workspace_management::services::input_resolution_service::InputResolutionService;

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

    fn text(&self, _message: &str, _default: Option<&str>) -> Result<String, PromptError> {
        if let Some(failure) = &self.failure {
            return Err(PromptError::internal(failure.clone()));
        }

        Ok(self.answer.clone().unwrap_or_default())
    }

    fn confirm(&self, _message: &str, _default: bool) -> Result<bool, PromptError> {
        Ok(true)
    }

    fn select(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<SelectOption, PromptError> {
        Err(PromptError::internal("not implemented"))
    }

    fn select_index(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<usize, PromptError> {
        Ok(0)
    }

    fn multiselect(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_indices: &[usize],
    ) -> Result<Vec<SelectOption>, PromptError> {
        Ok(Vec::new())
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
