use crate::features::cli::configuration::abstractions::config_store::ConfigStore;
use crate::features::generator_management::commands::ensure_default_source::ensure_default_source_command::{
    EnsureDefaultSourceCommand, EnsureDefaultSourceCommandResult,
};
use crate::features::generator_management::constants::source;
use crate::features::generator_management::models::errors::generators_service_error::GeneratorsServiceError;
use n_framework_nfw_core_domain::features::generator_management::generator_source::GeneratorSource;

/// Command handler for ensuring the default generator source is registered.
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
    ) -> Result<EnsureDefaultSourceCommandResult, GeneratorsServiceError> {
        let mut sources = self
            .config_store
            .load_sources()
            .map_err(GeneratorsServiceError::LoadSourcesFailed)?;

        if sources.iter().any(|s| s.name == source::OFFICIAL_NAME) {
            return Ok(EnsureDefaultSourceCommandResult);
        }

        sources.push(GeneratorSource::new(
            source::OFFICIAL_NAME.to_owned(),
            source::OFFICIAL_URL.to_owned(),
        ));
        self.config_store
            .save_sources(&sources)
            .map_err(GeneratorsServiceError::SaveSourcesFailed)?;

        Ok(EnsureDefaultSourceCommandResult)
    }
}
