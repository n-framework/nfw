use std::path::Path;
use std::process::Command;

use crate::features::check::models::{FindingType, ValidationFinding};
use crate::features::check::services::abstractions::{ExternalToolResult, ExternalToolRunner};
use crate::features::check::services::remediation_hint_service::RemediationHintService;

/// Process-based implementation of ExternalToolRunner.
pub struct ProcessExternalToolRunner {
    remediation_hint_service: RemediationHintService,
}

impl ProcessExternalToolRunner {
    pub fn new(remediation_hint_service: RemediationHintService) -> Self {
        Self {
            remediation_hint_service,
        }
    }
}

impl ExternalToolRunner for ProcessExternalToolRunner {
    fn run_make_lint(&self, workspace_root: &Path) -> Option<ValidationFinding> {
        let output = Command::new("make")
            .arg("lint")
            .current_dir(workspace_root)
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    return None;
                }

                let stderr = String::from_utf8_lossy(&result.stderr).trim().to_owned();
                let stdout = String::from_utf8_lossy(&result.stdout).trim().to_owned();
                let message = if !stderr.is_empty() {
                    stderr
                } else if !stdout.is_empty() {
                    stdout
                } else {
                    "make lint failed without output".to_owned()
                };

                Some(ValidationFinding {
                    finding_type: FindingType::LintIssue,
                    location: workspace_root.to_path_buf(),
                    offending_value: message,
                    violated_rule_id: Some("lint:make-lint".to_owned()),
                    remediation_hint: self.remediation_hint_service.for_lint_issue(),
                })
            }
            Err(error) => Some(ValidationFinding {
                finding_type: FindingType::LintIssue,
                location: workspace_root.to_path_buf(),
                offending_value: format!(
                    "failed to execute `make lint` in '{}': {error} (make may not be installed or not in PATH)",
                    workspace_root.display()
                ),
                violated_rule_id: Some("lint:make-lint-execution".to_owned()),
                remediation_hint: self.remediation_hint_service.for_lint_issue(),
            }),
        }
    }

    fn run_make_test(&self, service_root: &Path) -> Option<ValidationFinding> {
        let output = Command::new("make")
            .arg("test")
            .current_dir(service_root)
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    return None;
                }

                let stderr = String::from_utf8_lossy(&result.stderr).trim().to_owned();
                let stdout = String::from_utf8_lossy(&result.stdout).trim().to_owned();
                let message = if !stderr.is_empty() {
                    stderr
                } else if !stdout.is_empty() {
                    stdout
                } else {
                    "make test failed without output".to_owned()
                };

                Some(ValidationFinding {
                    finding_type: FindingType::TestIssue,
                    location: service_root.to_path_buf(),
                    offending_value: message,
                    violated_rule_id: Some("test:make-test".to_owned()),
                    remediation_hint: self.remediation_hint_service.for_test_issue(),
                })
            }
            Err(error) => Some(ValidationFinding {
                finding_type: FindingType::TestIssue,
                location: service_root.to_path_buf(),
                offending_value: format!(
                    "failed to execute `make test` in '{}': {error} (make may not be installed or not in PATH)",
                    service_root.display()
                ),
                violated_rule_id: Some("test:make-test-execution".to_owned()),
                remediation_hint: self.remediation_hint_service.for_test_issue(),
            }),
        }
    }

    fn run_command(
        &self,
        command: &str,
        args: &[&str],
        working_dir: &Path,
    ) -> Result<ExternalToolResult, String> {
        let output = Command::new(command)
            .args(args)
            .current_dir(working_dir)
            .output()
            .map_err(|error| format!("failed to execute '{command}': {error}"))?;

        Ok(ExternalToolResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).trim().to_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
            exit_code: output.status.code(),
        })
    }
}
