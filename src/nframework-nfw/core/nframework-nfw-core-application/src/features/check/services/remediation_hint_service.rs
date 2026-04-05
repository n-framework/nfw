use crate::features::check::models::{CheckLayer, FindingType};

#[derive(Debug, Default, Clone, Copy)]
pub struct RemediationHintService;

impl RemediationHintService {
    pub fn new() -> Self {
        Self
    }

    pub fn for_project_reference(
        &self,
        source_layer: CheckLayer,
        target_layer: CheckLayer,
    ) -> String {
        format!(
            "remove the reference and move shared contracts to an allowed boundary ({source_layer:?} -> {target_layer:?})"
        )
    }

    pub fn for_namespace_usage(&self, namespace_prefix: &str) -> String {
        format!(
            "replace forbidden namespace usage '{namespace_prefix}' with an allowed abstraction"
        )
    }

    pub fn for_package_usage(&self, package_name: &str) -> String {
        format!("remove direct package reference '{package_name}' from this layer")
    }

    pub fn for_unreadable_artifact(&self) -> String {
        "fix file permissions or file format and rerun `nfw check`".to_owned()
    }

    pub fn for_lint_issue(&self) -> String {
        "run `make lint`, fix reported lint violations, then rerun `nfw check`".to_owned()
    }

    pub fn for_test_issue(&self) -> String {
        "run `make test` in the reported service, fix failing tests, then rerun `nfw check`"
            .to_owned()
    }

    pub fn for_finding_type(&self, finding_type: FindingType) -> String {
        match finding_type {
            FindingType::ProjectReference => {
                "remove forbidden project reference or refactor dependency direction".to_owned()
            }
            FindingType::NamespaceUsage => {
                "replace forbidden namespace usage with an allowed abstraction".to_owned()
            }
            FindingType::PackageUsage => {
                "remove forbidden direct package and keep only allowed dependencies".to_owned()
            }
            FindingType::UnreadableArtifact => self.for_unreadable_artifact(),
            FindingType::LintIssue => self.for_lint_issue(),
            FindingType::TestIssue => self.for_test_issue(),
        }
    }
}
