use nframework_nfw_core_application::features::check::models::{
    CheckCommandResult, ValidationFinding,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct CheckOutputFormatter;

impl CheckOutputFormatter {
    pub fn new() -> Self {
        Self
    }

    pub fn success_message(&self, result: &CheckCommandResult) -> String {
        format!(
            "architecture validation passed in '{}': no forbidden project references, namespaces, direct packages, lint issues, or service test issues found",
            result.workspace_root.display()
        )
    }

    pub fn failure_message(&self, result: &CheckCommandResult) -> String {
        let mut lines = Vec::new();

        lines.push(format!(
            "architecture validation found {} issue(s)",
            result.summary.total_findings
        ));

        for finding in &result.findings {
            lines.push(self.format_finding(finding));
        }

        lines.push(format!(
            "summary: project_reference={}, namespace_usage={}, package_usage={}, unreadable_artifact={}, lint_issue={}, test_issue={}",
            result.summary.project_reference_count,
            result.summary.namespace_usage_count,
            result.summary.package_usage_count,
            result.summary.unreadable_artifact_count,
            result.summary.lint_issue_count,
            result.summary.test_issue_count,
        ));

        lines.join("\n")
    }

    fn format_finding(&self, finding: &ValidationFinding) -> String {
        format!(
            "- type={} location={} offending={} hint={}",
            finding.finding_type,
            finding.location.display(),
            finding.offending_value,
            finding.remediation_hint,
        )
    }
}
