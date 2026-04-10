use n_framework_nfw_core_application::features::check::models::errors::CheckError;
use n_framework_nfw_core_application::features::check::models::{ExitOutcome, ValidationSummary};
use n_framework_nfw_core_application::features::cli::exit_codes::ExitCodes;
use n_framework_nfw_core_application::features::service_management::models::errors::add_service_error::AddServiceError;

#[test]
fn maps_add_service_interrupted_to_sigint_exit_code() {
    let exit_code = ExitCodes::from_add_service_error(&AddServiceError::Interrupted) as i32;

    assert_eq!(exit_code, 130);
}

#[test]
fn maps_check_error_to_validation_exit_code() {
    let exit_code = ExitCodes::from_check_error(&CheckError::InvalidWorkspaceContext(
        "missing workspace".to_owned(),
    )) as i32;

    assert_eq!(exit_code, ExitCodes::ValidationError as i32);
}

#[test]
fn maps_check_summary_to_success_exit_code() {
    let summary = ValidationSummary {
        total_findings: 0,
        project_reference_count: 0,
        namespace_usage_count: 0,
        package_usage_count: 0,
        unreadable_artifact_count: 0,
        lint_issue_count: 0,
        test_issue_count: 0,
        exit_outcome: ExitOutcome::Success,
    };

    let exit_code = ExitCodes::from_check_summary(&summary) as i32;

    assert_eq!(exit_code, ExitCodes::Success as i32);
}
