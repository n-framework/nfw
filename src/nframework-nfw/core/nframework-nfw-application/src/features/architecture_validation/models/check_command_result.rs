use std::path::PathBuf;

use crate::features::architecture_validation::models::validation_finding::ValidationFinding;
use crate::features::architecture_validation::models::validation_summary::ValidationSummary;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckCommandResult {
    pub workspace_root: PathBuf,
    pub findings: Vec<ValidationFinding>,
    pub summary: ValidationSummary,
}
