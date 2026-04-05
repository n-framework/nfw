use std::collections::BTreeMap;

use crate::features::architecture_validation::models::exit_outcome::ExitOutcome;
use crate::features::architecture_validation::models::finding_type::FindingType;
use crate::features::architecture_validation::models::validation_finding::ValidationFinding;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationSummary {
    pub total_findings: usize,
    pub project_reference_count: usize,
    pub namespace_usage_count: usize,
    pub package_usage_count: usize,
    pub unreadable_artifact_count: usize,
    pub lint_issue_count: usize,
    pub test_issue_count: usize,
    pub exit_outcome: ExitOutcome,
}

impl ValidationSummary {
    pub fn from_findings(findings: &[ValidationFinding], interrupted: bool) -> Self {
        let grouped_counts = findings.iter().fold(BTreeMap::new(), |mut acc, finding| {
            let current = acc.entry(finding.finding_type).or_insert(0usize);
            *current += 1;
            acc
        });

        let total_findings = findings.len();
        let exit_outcome = if interrupted {
            ExitOutcome::ExecutionInterrupted
        } else if total_findings > 0 {
            ExitOutcome::ViolationFound
        } else {
            ExitOutcome::Success
        };

        Self {
            total_findings,
            project_reference_count: *grouped_counts.get(&FindingType::ProjectReference).unwrap_or(&0),
            namespace_usage_count: *grouped_counts.get(&FindingType::NamespaceUsage).unwrap_or(&0),
            package_usage_count: *grouped_counts.get(&FindingType::PackageUsage).unwrap_or(&0),
            unreadable_artifact_count: *grouped_counts.get(&FindingType::UnreadableArtifact).unwrap_or(&0),
            lint_issue_count: *grouped_counts.get(&FindingType::LintIssue).unwrap_or(&0),
            test_issue_count: *grouped_counts.get(&FindingType::TestIssue).unwrap_or(&0),
            exit_outcome,
        }
    }
}
