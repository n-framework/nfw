use std::path::PathBuf;

use crate::features::generator_management::generator_metadata::GeneratorMetadata;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorDescriptor {
    pub metadata: GeneratorMetadata,
    pub cache_path: PathBuf,
}

impl GeneratorDescriptor {
    pub fn new(metadata: GeneratorMetadata, cache_path: PathBuf) -> Self {
        Self {
            metadata,
            cache_path,
        }
    }
}
