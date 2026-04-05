use crate::features::service_management::models::add_service_command_request::AddServiceCommandRequest;
use crate::features::service_management::models::errors::add_service_error::AddServiceError;

#[derive(Debug, Default, Clone, Copy)]
pub struct AddServiceRequestValidator;

impl AddServiceRequestValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate_request(
        &self,
        request: &AddServiceCommandRequest,
    ) -> Result<(), AddServiceError> {
        if let Some(service_name) = request.service_name.as_deref() {
            self.validate_service_name(service_name)?;
        } else if request.is_non_interactive() {
            return Err(AddServiceError::MissingRequiredInput("name".to_owned()));
        }

        if request.is_non_interactive() && request.template_id.is_none() {
            return Err(AddServiceError::MissingRequiredInput("template".to_owned()));
        }

        Ok(())
    }

    pub fn validate_service_name(&self, service_name: &str) -> Result<(), AddServiceError> {
        if !is_valid_service_name(service_name) {
            return Err(AddServiceError::InvalidServiceName(service_name.to_owned()));
        }

        Ok(())
    }
}

fn is_valid_service_name(value: &str) -> bool {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return false;
    }

    let mut characters = trimmed.chars();
    let Some(first_character) = characters.next() else {
        return false;
    };

    if !first_character.is_ascii_alphabetic() {
        return false;
    }

    trimmed
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '-' || character == '_')
}
