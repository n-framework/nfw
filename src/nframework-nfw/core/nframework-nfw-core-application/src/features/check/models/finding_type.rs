use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FindingType {
    ProjectReference,
    NamespaceUsage,
    PackageUsage,
    UnreadableArtifact,
    LintIssue,
    TestIssue,
}

impl Display for FindingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProjectReference => write!(f, "project_reference"),
            Self::NamespaceUsage => write!(f, "namespace_usage"),
            Self::PackageUsage => write!(f, "package_usage"),
            Self::UnreadableArtifact => write!(f, "unreadable_artifact"),
            Self::LintIssue => write!(f, "lint_issue"),
            Self::TestIssue => write!(f, "test_issue"),
        }
    }
}
