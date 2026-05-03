use super::global_constants::GlobalConstants;
use super::validation_utils::ValidationUtils;
use crate::features::entity_generation::errors::entity_generation_error::EntityGenerationError;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    name: String,
    path: PathBuf,
    modules: Vec<String>,
}

impl ServiceInfo {
    pub fn new(name: String, path: PathBuf, modules: Vec<String>) -> Self {
        Self {
            name,
            path,
            modules,
        }
    }

    pub fn try_new(
        name: String,
        path: PathBuf,
        modules: Vec<String>,
    ) -> Result<Self, EntityGenerationError> {
        ValidationUtils::validate_pascal_case(&name, GlobalConstants::SERVICE_LABEL)?;

        if !path.exists() {
            return Err(EntityGenerationError::ConfigError {
                reason: format!("Service directory does not exist: {}", path.display()),
            });
        }

        Ok(Self::new(name, path, modules))
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn modules(&self) -> &[String] {
        &self.modules
    }
}

#[cfg(test)]
#[path = "domain_objects.tests.rs"]
mod tests;
