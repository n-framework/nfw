use nframework_nfw_application::features::cli::exit_codes::ExitCodes;
use nframework_nfw_application::features::service_management::models::errors::add_service_error::AddServiceError;

#[test]
fn maps_add_service_interrupted_to_sigint_exit_code() {
    let exit_code = ExitCodes::from_add_service_error(&AddServiceError::Interrupted) as i32;

    assert_eq!(exit_code, 130);
}
