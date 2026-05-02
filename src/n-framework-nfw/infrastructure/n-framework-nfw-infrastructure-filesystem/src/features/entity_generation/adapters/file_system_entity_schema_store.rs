use std::fs;
use std::path::Path;

use n_framework_nfw_core_application::features::entity_generation::abstractions::entity_schema_store::EntitySchemaStore;
use n_framework_nfw_core_domain::features::entity_generation::entities::entity_schema::EntitySchema;
use n_framework_nfw_core_domain::features::entity_generation::errors::entity_generation_error::EntityGenerationError;

/// File system implementation of `EntitySchemaStore` for reading/writing YAML schema files.
#[derive(Debug, Clone)]
pub struct FileSystemEntitySchemaStore;

impl FileSystemEntitySchemaStore {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FileSystemEntitySchemaStore {
    fn default() -> Self {
        Self::new()
    }
}

impl EntitySchemaStore for FileSystemEntitySchemaStore {
    fn write_schema(
        &self,
        specs_dir: &Path,
        schema: &EntitySchema,
    ) -> Result<(), EntityGenerationError> {
        std::fs::create_dir_all(specs_dir).map_err(|e| {
            EntityGenerationError::DirectoryCreationError {
                path: specs_dir.to_path_buf(),
                reason: e.to_string(),
            }
        })?;

        let schema_path = specs_dir.join(format!("{}.yaml", schema.entity()));
        let yaml =
            serde_yaml::to_string(schema).map_err(|e| EntityGenerationError::ConfigError {
                reason: format!("failed to serialize schema: {e}"),
            })?;

        fs::write(&schema_path, &yaml).map_err(|e| EntityGenerationError::SchemaWriteError {
            path: schema_path,
            reason: e.to_string(),
        })?;

        Ok(())
    }

    fn read_schema(&self, schema_path: &Path) -> Result<EntitySchema, EntityGenerationError> {
        let content = fs::read_to_string(schema_path).map_err(|e| {
            EntityGenerationError::SchemaReadError {
                path: schema_path.to_path_buf(),
                reason: e.to_string(),
            }
        })?;

        let schema: EntitySchema = serde_yaml::from_str(&content).map_err(|e| {
            EntityGenerationError::InvalidSchemaContent {
                path: schema_path.to_path_buf(),
                reason: e.to_string(),
            }
        })?;

        Ok(schema)
    }

    fn schema_exists(&self, schema_path: &Path) -> bool {
        schema_path.is_file()
    }
}

#[cfg(test)]
#[path = "file_system_entity_schema_store.tests.rs"]
mod tests;
