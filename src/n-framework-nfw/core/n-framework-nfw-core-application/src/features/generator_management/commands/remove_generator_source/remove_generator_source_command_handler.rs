use crate::features::cli::configuration::abstractions::config_store::ConfigStore;
use crate::features::generator_management::commands::remove_generator_source::remove_generator_source_command::{
    RemoveGeneratorSourceCommand, RemoveGeneratorSourceCommandResult,
};
use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use crate::features::generator_management::services::abstractions::generator_source_synchronizer::GeneratorSourceSynchronizer;

/// Command handler for removing a generator source.
///
/// This handler encapsulates the business logic for removing a generator source:
/// 1. Loads current sources
/// 2. Validates the source exists
/// 3. Removes the source and clears cache
/// 4. Persists the updated sources
#[derive(Debug, Clone)]
pub struct RemoveGeneratorSourceCommandHandler<CS, S>
where
    CS: ConfigStore,
    S: GeneratorSourceSynchronizer,
{
    config_store: CS,
    source_synchronizer: S,
}

impl<CS, S> RemoveGeneratorSourceCommandHandler<CS, S>
where
    CS: ConfigStore,
    S: GeneratorSourceSynchronizer,
{
    pub fn new(config_store: CS, source_synchronizer: S) -> Self {
        Self {
            config_store,
            source_synchronizer,
        }
    }

    pub fn handle(
        &self,
        command: &RemoveGeneratorSourceCommand,
    ) -> Result<RemoveGeneratorSourceCommandResult, GeneratorsServiceError> {
        let normalized_name = command.name.trim().to_owned();

        let mut sources = self
            .config_store
            .load_sources()
            .map_err(GeneratorsServiceError::LoadSourcesFailed)?;

        let source_count_before = sources.len();
        sources.retain(|source| source.name != normalized_name);

        if sources.len() == source_count_before {
            return Err(GeneratorsServiceError::SourceNotFound(normalized_name));
        }

        self.source_synchronizer
            .clear_source_cache(&normalized_name)
            .map_err(GeneratorsServiceError::CacheCleanupFailed)?;

        self.config_store
            .save_sources(&sources)
            .map_err(GeneratorsServiceError::SaveSourcesFailed)?;

        Ok(RemoveGeneratorSourceCommandResult {
            source_name: normalized_name,
        })
    }
}
