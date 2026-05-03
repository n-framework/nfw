use std::path::Path;

use n_framework_nfw_core_domain::features::entity_generation::entities::entity_schema::EntitySchema;
use n_framework_nfw_core_domain::features::entity_generation::errors::entity_generation_error::EntityGenerationError;

/// Infrastructure abstraction for reading and writing entity schema YAML files.
pub trait EntitySchemaStore {
    fn write_schema(
        &self,
        specs_dir: &Path,
        schema: &EntitySchema,
    ) -> Result<(), EntityGenerationError>;

    fn read_schema(&self, schema_path: &Path) -> Result<EntitySchema, EntityGenerationError>;

    fn schema_exists(&self, schema_path: &Path) -> bool;
}
