use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceNewError {
    InvalidWorkspaceName(String),
    MissingWorkspaceName,
    MissingRequiredInput(String),
    TemplateNotFound(String),
    AmbiguousTemplate(String),
    InvalidOptionCombination(String),
    TargetDirectoryNotEmpty(String),
    PromptFailed(String),
    WriteFailed(String),
    Internal(String),
}

impl Display for WorkspaceNewError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidWorkspaceName(name) => write!(
                f,
                "workspace name '{name}' is invalid; use lowercase kebab-case or alphanumeric"
            ),
            Self::MissingWorkspaceName => {
                write!(f, "workspace name is required for `nfw new`")
            }
            Self::MissingRequiredInput(field) => {
                write!(
                    f,
                    "required input '{field}' is missing in non-interactive mode"
                )
            }
            Self::TemplateNotFound(template) => {
                write!(
                    f,
                    "template '{template}' was not found in discovered templates"
                )
            }
            Self::AmbiguousTemplate(template) => {
                write!(f, "template identifier '{template}' is ambiguous")
            }
            Self::InvalidOptionCombination(message) => {
                write!(f, "invalid option combination: {message}")
            }
            Self::TargetDirectoryNotEmpty(path) => {
                write!(
                    f,
                    "target directory '{path}' already exists and is not empty"
                )
            }
            Self::PromptFailed(reason) => write!(f, "interactive prompt failed: {reason}"),
            Self::WriteFailed(reason) => write!(f, "failed to write workspace artifacts: {reason}"),
            Self::Internal(reason) => write!(f, "workspace initialization failed: {reason}"),
        }
    }
}

impl std::error::Error for WorkspaceNewError {}
