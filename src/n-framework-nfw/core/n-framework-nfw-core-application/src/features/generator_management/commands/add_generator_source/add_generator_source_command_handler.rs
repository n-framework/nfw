use crate::features::cli::configuration::abstractions::config_store::ConfigStore;
use crate::features::generator_management::commands::add_generator_source::add_generator_source_command::{
    AddGeneratorSourceCommand, AddGeneratorSourceCommandResult,
};
use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use crate::features::generator_management::services::abstractions::git_repository::GitRepository;
use crate::features::generator_management::services::abstractions::validator::Validator;
use n_framework_nfw_core_domain::features::generator_management::generator_source::GeneratorSource;

/// Command handler for adding a generator source.
///
/// This handler encapsulates the business logic for adding a generator source:
/// 1. Validates the source name (kebab case)
/// 2. Validates the URL format
/// 3. Checks for duplicates
/// 4. Persists the new source
#[derive(Debug, Clone)]
pub struct AddGeneratorSourceCommandHandler<CS, V, G>
where
    CS: ConfigStore,
    V: Validator,
    G: GitRepository,
{
    config_store: CS,
    validator: V,
    git_repository: G,
}

impl<CS, V, G> AddGeneratorSourceCommandHandler<CS, V, G>
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
        command: &AddGeneratorSourceCommand,
    ) -> Result<AddGeneratorSourceCommandResult, GeneratorsServiceError> {
        let normalized_name = command.name.trim().to_owned();
        let normalized_url = command.url.trim().to_owned();

        if !self.validator.is_kebab_case(&normalized_name) {
            return Err(GeneratorsServiceError::InvalidSourceName(
                command.name.clone(),
            ));
        }

        if normalized_url.is_empty()
            || !self.git_repository.is_valid_git_url_format(&normalized_url)
        {
            return Err(GeneratorsServiceError::InvalidSourceUrl(
                command.url.clone(),
            ));
        }

        let mut sources = self
            .config_store
            .load_sources()
            .map_err(GeneratorsServiceError::LoadSourcesFailed)?;

        if sources.iter().any(|source| source.name == normalized_name) {
            return Err(GeneratorsServiceError::SourceAlreadyExists(
                normalized_name.clone(),
            ));
        }

        sources.push(GeneratorSource::new(
            normalized_name.clone(),
            normalized_url,
        ));
        sources.sort_by(|left, right| left.name.cmp(&right.name));
        self.config_store
            .save_sources(&sources)
            .map_err(GeneratorsServiceError::SaveSourcesFailed)?;

        Ok(AddGeneratorSourceCommandResult {
            source_name: normalized_name,
        })
    }
}
