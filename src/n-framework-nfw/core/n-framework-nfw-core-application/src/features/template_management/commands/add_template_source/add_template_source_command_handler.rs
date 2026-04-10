use crate::features::cli::configuration::abstractions::config_store::ConfigStore;
use crate::features::template_management::commands::add_template_source::add_template_source_command::{
    AddTemplateSourceCommand, AddTemplateSourceCommandResult,
};
use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::services::abstractions::git_repository::GitRepository;
use crate::features::template_management::services::abstractions::validator::Validator;
use n_framework_nfw_core_domain::features::template_management::template_source::TemplateSource;

/// Command handler for adding a template source.
///
/// This handler encapsulates the business logic for adding a template source:
/// 1. Validates the source name (kebab case)
/// 2. Validates the URL format
/// 3. Checks for duplicates
/// 4. Persists the new source
#[derive(Debug, Clone)]
pub struct AddTemplateSourceCommandHandler<CS, V, G>
where
    CS: ConfigStore,
    V: Validator,
    G: GitRepository,
{
    config_store: CS,
    validator: V,
    git_repository: G,
}

impl<CS, V, G> AddTemplateSourceCommandHandler<CS, V, G>
where
    CS: ConfigStore,
    V: Validator,
    G: GitRepository,
{
    pub fn new(config_store: CS, validator: V, git_repository: G) -> Self {
        Self {
            config_store,
            validator,
            git_repository,
        }
    }

    pub fn handle(
        &self,
        command: &AddTemplateSourceCommand,
    ) -> Result<AddTemplateSourceCommandResult, TemplatesServiceError> {
        let normalized_name = command.name.trim().to_owned();
        let normalized_url = command.url.trim().to_owned();

        if !self.validator.is_kebab_case(&normalized_name) {
            return Err(TemplatesServiceError::InvalidSourceName(
                command.name.clone(),
            ));
        }

        if normalized_url.is_empty()
            || !self.git_repository.is_valid_git_url_format(&normalized_url)
        {
            return Err(TemplatesServiceError::InvalidSourceUrl(command.url.clone()));
        }

        let mut sources = self
            .config_store
            .load_sources()
            .map_err(TemplatesServiceError::LoadSourcesFailed)?;

        if sources.iter().any(|source| source.name == normalized_name) {
            return Err(TemplatesServiceError::SourceAlreadyExists(
                normalized_name.clone(),
            ));
        }

        sources.push(TemplateSource::new(normalized_name.clone(), normalized_url));
        sources.sort_by(|left, right| left.name.cmp(&right.name));
        self.config_store
            .save_sources(&sources)
            .map_err(TemplatesServiceError::SaveSourcesFailed)?;

        Ok(AddTemplateSourceCommandResult {
            source_name: normalized_name,
        })
    }
}
