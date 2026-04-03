use crate::features::cli::configuration::abstraction::config_store::ConfigStore;
use crate::features::template_management::commands::remove_template_source::remove_template_source_command::{
    RemoveTemplateSourceCommand, RemoveTemplateSourceCommandResult,
};
use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use crate::features::template_management::services::abstraction::template_source_synchronizer::TemplateSourceSynchronizer;

/// Command handler for removing a template source.
///
/// This handler encapsulates the business logic for removing a template source:
/// 1. Loads current sources
/// 2. Validates the source exists
/// 3. Removes the source and clears cache
/// 4. Persists the updated sources
#[derive(Debug, Clone)]
pub struct RemoveTemplateSourceCommandHandler<CS, S>
where
    CS: ConfigStore,
    S: TemplateSourceSynchronizer,
{
    config_store: CS,
    source_synchronizer: S,
}

impl<CS, S> RemoveTemplateSourceCommandHandler<CS, S>
where
    CS: ConfigStore,
    S: TemplateSourceSynchronizer,
{
    pub fn new(config_store: CS, source_synchronizer: S) -> Self {
        Self {
            config_store,
            source_synchronizer,
        }
    }

    pub fn handle(
        &self,
        command: &RemoveTemplateSourceCommand,
    ) -> Result<RemoveTemplateSourceCommandResult, TemplatesServiceError> {
        let normalized_name = command.name.trim().to_owned();

        let mut sources = self
            .config_store
            .load_sources()
            .map_err(TemplatesServiceError::LoadSourcesFailed)?;

        let source_count_before = sources.len();
        sources.retain(|source| source.name != normalized_name);

        if sources.len() == source_count_before {
            return Err(TemplatesServiceError::SourceNotFound(normalized_name));
        }

        self.source_synchronizer
            .clear_source_cache(&normalized_name)
            .map_err(TemplatesServiceError::CacheCleanupFailed)?;

        self.config_store
            .save_sources(&sources)
            .map_err(TemplatesServiceError::SaveSourcesFailed)?;

        Ok(RemoveTemplateSourceCommandResult {
            source_name: normalized_name,
        })
    }
}
