use crate::features::cli::configuration::abstractions::config_store::ConfigStore;
use crate::features::template_management::commands::ensure_default_source::ensure_default_source_command::{
    EnsureDefaultSourceCommand, EnsureDefaultSourceCommandResult,
};
use crate::features::template_management::constants::source;
use crate::features::template_management::models::errors::templates_service_error::TemplatesServiceError;
use nframework_nfw_domain::features::template_management::template_source::TemplateSource;

/// Command handler for ensuring the default template source is registered.
///
/// This handler encapsulates the bootstrap logic:
/// 1. Loads current sources
/// 2. Checks if the official source is already registered
/// 3. Adds the official source if missing
/// 4. Persists the updated sources
#[derive(Debug, Clone)]
pub struct EnsureDefaultSourceCommandHandler<CS>
where
    CS: ConfigStore,
{
    config_store: CS,
}

impl<CS> EnsureDefaultSourceCommandHandler<CS>
where
    CS: ConfigStore,
{
    pub fn new(config_store: CS) -> Self {
        Self { config_store }
    }

    pub fn handle(
        &self,
        _command: &EnsureDefaultSourceCommand,
    ) -> Result<EnsureDefaultSourceCommandResult, TemplatesServiceError> {
        let mut sources = self
            .config_store
            .load_sources()
            .map_err(TemplatesServiceError::LoadSourcesFailed)?;

        if sources.iter().any(|s| s.name == source::OFFICIAL_NAME) {
            return Ok(EnsureDefaultSourceCommandResult);
        }

        sources.push(TemplateSource::new(
            source::OFFICIAL_NAME.to_owned(),
            source::OFFICIAL_URL.to_owned(),
        ));
        self.config_store
            .save_sources(&sources)
            .map_err(TemplatesServiceError::SaveSourcesFailed)?;

        Ok(EnsureDefaultSourceCommandResult)
    }
}
