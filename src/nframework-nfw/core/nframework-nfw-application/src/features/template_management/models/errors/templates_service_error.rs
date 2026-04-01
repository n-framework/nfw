use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplatesServiceError {
    LoadSourcesFailed(String),
}

impl Display for TemplatesServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LoadSourcesFailed(reason) => {
                write!(f, "failed to load template sources: {reason}")
            }
        }
    }
}

impl std::error::Error for TemplatesServiceError {}
