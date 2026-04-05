use std::path::PathBuf;

use crate::features::architecture_validation::models::finding_type::FindingType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationFinding {
    pub finding_type: FindingType,
    pub location: PathBuf,
    pub offending_value: String,
    pub violated_rule_id: Option<String>,
    pub remediation_hint: String,
}

impl ValidationFinding {
    pub fn dedupe_key(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.finding_type,
            self.location.display(),
            self.offending_value,
            self.violated_rule_id.clone().unwrap_or_default(),
        )
    }
}
