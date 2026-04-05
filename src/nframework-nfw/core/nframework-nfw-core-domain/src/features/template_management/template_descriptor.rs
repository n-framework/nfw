use std::path::PathBuf;

use crate::features::template_management::template_metadata::TemplateMetadata;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateDescriptor {
    pub metadata: TemplateMetadata,
    pub cache_path: PathBuf,
}

impl TemplateDescriptor {
    pub fn new(metadata: TemplateMetadata, cache_path: PathBuf) -> Self {
        Self {
            metadata,
            cache_path,
        }
    }
}
