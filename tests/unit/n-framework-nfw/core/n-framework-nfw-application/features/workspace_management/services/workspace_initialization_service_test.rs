use std::path::PathBuf;

use n_framework_core_cli_abstractions::{PromptError, PromptService, SelectOption};
use n_framework_nfw_core_application::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_catalog_discovery_service::TemplateCatalogDiscoveryService;
use n_framework_nfw_core_application::features::workspace_management::models::new_command_request::NewCommandRequest;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::working_directory_provider::WorkingDirectoryProvider;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::workspace_name_validator::WorkspaceNameValidator;
use n_framework_nfw_core_application::features::workspace_management::services::abstractions::workspace_writer::WorkspaceWriter;
use n_framework_nfw_core_application::features::workspace_management::services::template_selection_for_new_service::TemplateSelectionForNewService;
use n_framework_nfw_core_application::features::workspace_management::services::workspace_initialization_service::WorkspaceInitializationService;
use n_framework_nfw_core_domain::features::template_management::template_catalog::TemplateCatalog;
use n_framework_nfw_core_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;

#[derive(Debug, Clone)]
struct TestPromptService;

impl PromptService for TestPromptService {
    fn is_interactive(&self) -> bool {
        false
    }

    fn text(&self, _message: &str, _default: Option<&str>) -> Result<String, PromptError> {
        Ok("test-value".to_owned())
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

#[derive(Debug, Clone)]
struct TestValidator;

impl WorkspaceNameValidator for TestValidator {
    fn is_valid_workspace_name(&self, name: &str) -> bool {
        name == "valid-workspace"
    }
}

#[derive(Debug, Clone)]
struct TestCatalogDiscoveryService;

impl TemplateCatalogDiscoveryService for TestCatalogDiscoveryService {
    fn discover_catalogs(
        &self,
    ) -> Result<(Vec<TemplateCatalog>, Vec<String>), TemplatesServiceError> {
        Ok((vec![], vec![]))
    }
}

struct TestWorkspaceWriter;

impl WorkspaceWriter for TestWorkspaceWriter {
    fn write_workspace(
        &self,
        _blueprint: &WorkspaceBlueprint,
        _resolution: &n_framework_nfw_core_application::features::workspace_management::models::new_command_resolution::NewCommandResolution,
    ) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct TestWorkingDirectoryProvider {
    pub path: PathBuf,
}

impl WorkingDirectoryProvider for TestWorkingDirectoryProvider {
    fn current_dir(&self) -> Result<PathBuf, String> {
        Ok(self.path.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workspace_initialization_service_with_valid_request() {
        let working_dir = PathBuf::from("/test/dir");
        let working_dir_provider = TestWorkingDirectoryProvider {
            path: working_dir.clone(),
        };

        let service = WorkspaceInitializationService::new(
            TestPromptService,
            TestValidator,
            TemplateSelectionForNewService::new(TestCatalogDiscoveryService, TestPromptService),
            TestWorkspaceWriter,
            working_dir_provider,
        );

        let request = NewCommandRequest::new(Some("valid-workspace".to_owned()), None, true, false);

        // Should fail because no templates are available
        let result = service.execute(request);
        assert!(result.is_err());
    }

    #[test]
    fn test_workspace_initialization_service_with_invalid_name() {
        let working_dir = PathBuf::from("/test/dir");
        let working_dir_provider = TestWorkingDirectoryProvider { path: working_dir };

        let service = WorkspaceInitializationService::new(
            TestPromptService,
            TestValidator,
            TemplateSelectionForNewService::new(TestCatalogDiscoveryService, TestPromptService),
            TestWorkspaceWriter,
            working_dir_provider,
        );

        let request = NewCommandRequest::new(Some("INVALID-NAME".to_owned()), None, true, false);

        let result = service.execute(request);
        assert!(result.is_err());
    }

    #[test]
    fn test_workspace_initialization_service_with_missing_name() {
        let working_dir = PathBuf::from("/test/dir");
        let working_dir_provider = TestWorkingDirectoryProvider { path: working_dir };

        let service = WorkspaceInitializationService::new(
            TestPromptService,
            TestValidator,
            TemplateSelectionForNewService::new(TestCatalogDiscoveryService, TestPromptService),
            TestWorkspaceWriter,
            working_dir_provider,
        );

        let request = NewCommandRequest::new(None, None, true, false);

        let result = service.execute(request);
        assert!(result.is_err());
    }

    #[test]
    fn test_working_directory_provider_returns_correct_path() {
        let working_dir = PathBuf::from("/custom/working/dir");
        let working_dir_provider = TestWorkingDirectoryProvider {
            path: working_dir.clone(),
        };

        let result = working_dir_provider.current_dir();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), working_dir);
    }
}
