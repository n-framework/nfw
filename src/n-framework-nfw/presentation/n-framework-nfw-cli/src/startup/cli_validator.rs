use std::path::Path;

use n_framework_nfw_core_application::features::template_management::services::abstractions::validator::Validator;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::workspace_name_validator::WorkspaceNameValidator;
use n_framework_nfw_core_application::validation::is_kebab_case;

#[derive(Debug, Default, Clone, Copy)]
pub struct CliValidator;

impl Validator for CliValidator {
    fn is_kebab_case(&self, value: &str) -> bool {
        is_kebab_case(value)
    }

    fn is_git_url(&self, value: &str) -> bool {
        Self::has_valid_git_url_format(value)
    }
}

impl WorkspaceNameValidator for CliValidator {
    fn is_valid_workspace_name(&self, value: &str) -> bool {
        Self::has_valid_workspace_name_format(value)
    }
}

impl CliValidator {
    fn has_valid_git_url_format(value: &str) -> bool {
        value.starts_with("http://")
            || value.starts_with("https://")
            || value.starts_with("git@")
            || Path::new(value).exists()
    }

    fn has_valid_workspace_name_format(value: &str) -> bool {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return false;
        }

        trimmed.chars().all(|character| {
            character.is_ascii_alphanumeric() || character == '-' || character == '_'
        })
    }
}
