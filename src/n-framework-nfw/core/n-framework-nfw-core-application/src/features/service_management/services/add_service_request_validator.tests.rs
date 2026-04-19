use super::*;
use crate::features::service_management::models::add_service_command_request::AddServiceCommandRequest;
use crate::features::service_management::models::errors::add_service_error::AddServiceError;

#[test]
fn rejects_missing_template_in_non_interactive_mode() {
    let validator = AddServiceRequestValidator::new();
    let request = AddServiceCommandRequest::new(Some("Orders".to_owned()), None, true, false);

    let error = validator
        .validate_request(&request)
        .expect_err("missing template should fail in non-interactive mode");
    assert_eq!(
        error,
        AddServiceError::MissingRequiredInput("template".to_owned())
    );
}

#[test]
fn rejects_missing_name_in_non_interactive_mode() {
    let validator = AddServiceRequestValidator::new();
    let request = AddServiceCommandRequest::new(
        None,
        Some("official/dotnet-service".to_owned()),
        true,
        false,
    );

    let error = validator
        .validate_request(&request)
        .expect_err("missing name should fail in non-interactive mode");
    assert_eq!(
        error,
        AddServiceError::MissingRequiredInput("name".to_owned())
    );
}

#[test]
fn rejects_invalid_service_name() {
    let validator = AddServiceRequestValidator::new();
    let request = AddServiceCommandRequest::new(
        Some("invalid service".to_owned()),
        Some("official/dotnet-service".to_owned()),
        true,
        false,
    );

    let error = validator
        .validate_request(&request)
        .expect_err("invalid service name should fail");
    assert_eq!(
        error,
        AddServiceError::InvalidServiceName("invalid service".to_owned())
    );
}

#[test]
fn accepts_valid_request() {
    let validator = AddServiceRequestValidator::new();
    let request = AddServiceCommandRequest::new(
        Some("Orders".to_owned()),
        Some("official/dotnet-service".to_owned()),
        true,
        false,
    );

    validator
        .validate_request(&request)
        .expect("valid request should pass");
}

#[test]
fn accepts_missing_name_when_interactive_mode_can_prompt() {
    let validator = AddServiceRequestValidator::new();
    let request = AddServiceCommandRequest::new(
        None,
        Some("official/dotnet-service".to_owned()),
        false,
        true,
    );

    validator
        .validate_request(&request)
        .expect("interactive mode should allow prompting for missing name");
}
