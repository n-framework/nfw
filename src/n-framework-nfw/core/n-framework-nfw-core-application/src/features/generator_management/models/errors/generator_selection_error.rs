use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GeneratorSelectionError {
    DiscoverGeneratorsFailed(String),
    GeneratorNotFound {
        identifier: String,
    },
    AmbiguousGeneratorIdentifier {
        identifier: String,
        candidates: Vec<String>,
    },
    /// Internal error indicating a bug in the generator selection logic
    InternalError {
        message: String,
    },
}

impl Display for GeneratorSelectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DiscoverGeneratorsFailed(reason) => {
                write!(f, "failed to discover generators: {reason}")
            }
            Self::GeneratorNotFound { identifier } => {
                write!(
                    f,
                    "generator '{identifier}' was not found; use 'nfw generators list' to view available generators"
                )
            }
            Self::AmbiguousGeneratorIdentifier {
                identifier,
                candidates,
            } => {
                write!(
                    f,
                    "generator '{identifier}' is ambiguous; use a qualified identifier (source/generator). candidates: {}",
                    candidates.join(", ")
                )
            }
            Self::InternalError { message } => {
                write!(f, "internal error during generator selection: {message}")
            }
        }
    }
}

impl std::error::Error for GeneratorSelectionError {}
