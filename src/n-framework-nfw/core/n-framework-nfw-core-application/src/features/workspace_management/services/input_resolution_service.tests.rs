use super::*;
use crate::features::workspace_management::models::errors::workspace_new_error::WorkspaceNewError;
use crate::features::workspace_management::models::new_command_request::NewCommandRequest;
use n_framework_core_cli_abstractions::{
    InteractiveError, InteractivePrompt, Logger, LoggingError, SelectOption, Spinner,
};

#[derive(Debug, Clone)]
struct StubPromptService {
    interactive: bool,
    answer: Option<String>,
    failure: Option<String>,
}

impl InteractivePrompt for StubPromptService {
    fn is_interactive(&self) -> bool {
        self.interactive
    }

    fn text(&self, _message: &str, _default: Option<&str>) -> Result<String, InteractiveError> {
        if let Some(failure) = &self.failure {
            return Err(InteractiveError::internal(failure.clone()));
        }

        Ok(self.answer.clone().unwrap_or_default())
    }

    fn confirm(&self, _message: &str, _default: bool) -> Result<bool, InteractiveError> {
        Ok(true)
    }

    fn password(&self, _message: &str) -> Result<String, InteractiveError> {
        if let Some(failure) = &self.failure {
            return Err(InteractiveError::internal(failure.clone()));
        }

        Ok(self.answer.clone().unwrap_or_default())
    }

    fn select(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<SelectOption, InteractiveError> {
        Err(InteractiveError::internal("not implemented"))
    }

    fn select_index(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_index: Option<usize>,
    ) -> Result<usize, InteractiveError> {
        Ok(0)
    }

    fn multiselect(
        &self,
        _message: &str,
        _options: &[SelectOption],
        _default_indices: &[usize],
    ) -> Result<Vec<SelectOption>, InteractiveError> {
        Ok(Vec::new())
    }
}

impl Logger for StubPromptService {
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
