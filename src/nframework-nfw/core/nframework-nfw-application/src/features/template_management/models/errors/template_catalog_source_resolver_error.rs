use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateCatalogSourceResolverError {
    SourceScanFailed {
        source_name: String,
        reason: String,
    },
    MetadataReadFailed {
        template_path: PathBuf,
        reason: String,
    },
    InvalidTemplateMetadata {
        template_path: PathBuf,
        reason: String,
    },
}

impl Display for TemplateCatalogSourceResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceScanFailed {
                source_name,
                reason,
            } => {
                write!(
                    f,
                    "failed to scan template source '{source_name}': {reason}"
                )
            }
            Self::MetadataReadFailed {
                template_path,
                reason,
            } => {
                write!(
                    f,
                    "failed to read template metadata at '{}': {reason}",
                    template_path.display()
                )
            }
            Self::InvalidTemplateMetadata {
                template_path,
                reason,
            } => {
                write!(
                    f,
                    "template metadata at '{}' is invalid: {reason}",
                    template_path.display()
                )
            }
        }
    }
}

impl std::error::Error for TemplateCatalogSourceResolverError {}
