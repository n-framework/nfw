use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplatesServiceError {
    LoadSourcesFailed(String),
    SaveSourcesFailed(String),
    InvalidSourceName(String),
    SourceAlreadyExists(String),
    SourceNotFound(String),
    InvalidSourceUrl(String),
    CacheCleanupFailed(String),
}

impl Display for TemplatesServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadSourcesFailed(reason) => {
                write!(f, "failed to load template sources: {reason}")
            }
            Self::SaveSourcesFailed(reason) => {
                write!(f, "failed to save template sources: {reason}")
            }
            Self::InvalidSourceName(name) => {
                write!(f, "template source name '{name}' must be kebab-case")
            }
            Self::SourceAlreadyExists(name) => {
                write!(f, "template source '{name}' is already registered")
            }
            Self::SourceNotFound(name) => {
                write!(f, "template source '{name}' is not registered")
            }
            Self::InvalidSourceUrl(url) => {
                write!(
                    f,
                    "template source URL '{url}' is not a valid git repository"
                )
            }
            Self::CacheCleanupFailed(reason) => {
                write!(f, "failed to cleanup template source cache: {reason}")
            }
        }
    }
}

impl std::error::Error for TemplatesServiceError {}
