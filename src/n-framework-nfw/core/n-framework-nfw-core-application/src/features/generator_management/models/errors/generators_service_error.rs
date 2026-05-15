use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorsServiceError {
    LoadSourcesFailed(String),
    SaveSourcesFailed(String),
    InvalidSourceName(String),
    SourceAlreadyExists(String),
    SourceNotFound(String),
    InvalidSourceUrl(String),
    CacheCleanupFailed(String),
    /// Critical error: failed to synchronize a generator source
    SourceSyncFailed {
        source: String,
        reason: String,
    },
    /// Critical error: failed to discover generators from a source
    SourceDiscoveryFailed {
        source: String,
        reason: String,
    },
}

impl Display for GeneratorsServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadSourcesFailed(reason) => {
                write!(f, "failed to load generator sources: {reason}")
            }
            Self::SaveSourcesFailed(reason) => {
                write!(f, "failed to save generator sources: {reason}")
            }
            Self::InvalidSourceName(name) => {
                write!(f, "generator source name '{name}' must be kebab-case")
            }
            Self::SourceAlreadyExists(name) => {
                write!(f, "generator source '{name}' is already registered")
            }
            Self::SourceNotFound(name) => {
                write!(f, "generator source '{name}' is not registered")
            }
            Self::InvalidSourceUrl(url) => {
                write!(
                    f,
                    "generator source URL '{url}' is not a valid git repository"
                )
            }
            Self::CacheCleanupFailed(reason) => {
                write!(f, "failed to cleanup generator source cache: {reason}")
            }
            Self::SourceSyncFailed { source, reason } => {
                write!(
                    f,
                    "failed to synchronize generator source '{source}': {reason}"
                )
            }
            Self::SourceDiscoveryFailed { source, reason } => {
                write!(
                    f,
                    "failed to discover generators from source '{source}': {reason}"
                )
            }
        }
    }
}

impl std::error::Error for GeneratorsServiceError {}
