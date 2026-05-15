use std::fmt::{Display, Formatter};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorCatalogSourceResolverError {
    SourceScanFailed {
        source_name: String,
        reason: String,
    },
    MetadataReadFailed {
        generator_path: PathBuf,
        reason: String,
    },
    InvalidGeneratorMetadata {
        generator_path: PathBuf,
        reason: String,
    },
}

impl Display for GeneratorCatalogSourceResolverError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SourceScanFailed {
                source_name,
                reason,
            } => {
                write!(
                    f,
                    "failed to scan generator source '{source_name}': {reason}"
                )
            }
            Self::MetadataReadFailed {
                generator_path,
                reason,
            } => {
                write!(
                    f,
                    "failed to read generator metadata at '{}': {reason}",
                    generator_path.display()
                )
            }
            Self::InvalidGeneratorMetadata {
                generator_path,
                reason,
            } => {
                write!(
                    f,
                    "generator metadata at '{}' is invalid: {reason}",
                    generator_path.display()
                )
            }
        }
    }
}

impl std::error::Error for GeneratorCatalogSourceResolverError {}
