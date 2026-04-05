use std::path::PathBuf;

use crate::features::check::models::finding_type::FindingType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationFinding {
    pub finding_type: FindingType,
    pub location: PathBuf,
    pub offending_value: String,
    pub violated_rule_id: Option<String>,
    pub remediation_hint: String,
}

impl ValidationFinding {
    /// Creates a new ValidationFinding with the provided details.
    /// Returns an error if any required field is empty.
    pub fn new(
        finding_type: FindingType,
        location: PathBuf,
        offending_value: String,
        violated_rule_id: Option<String>,
        remediation_hint: String,
    ) -> Result<Self, String> {
        if offending_value.trim().is_empty() {
            return Err("offending_value cannot be empty".to_owned());
        }
        if remediation_hint.trim().is_empty() {
            return Err("remediation_hint cannot be empty".to_owned());
        }

        Ok(Self {
            finding_type,
            location,
            offending_value,
            violated_rule_id,
            remediation_hint,
        })
    }

    pub fn builder() -> ValidationFindingBuilder {
        ValidationFindingBuilder::default()
    }

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

#[derive(Debug, Default)]
pub struct ValidationFindingBuilder {
    finding_type: Option<FindingType>,
    location: Option<PathBuf>,
    offending_value: Option<String>,
    violated_rule_id: Option<String>,
    remediation_hint: Option<String>,
}

impl ValidationFindingBuilder {
    pub fn finding_type(mut self, value: FindingType) -> Self {
        self.finding_type = Some(value);
        self
    }

    pub fn location(mut self, value: PathBuf) -> Self {
        self.location = Some(value);
        self
    }

    pub fn offending_value(mut self, value: String) -> Self {
        self.offending_value = Some(value);
        self
    }

    pub fn violated_rule_id(mut self, value: Option<String>) -> Self {
        self.violated_rule_id = value;
        self
    }

    pub fn remediation_hint(mut self, value: String) -> Self {
        self.remediation_hint = Some(value);
        self
    }

    pub fn build(self) -> Result<ValidationFinding, String> {
        let finding_type = self
            .finding_type
            .ok_or_else(|| "finding_type is required".to_owned())?;
        let location = self
            .location
            .ok_or_else(|| "location is required".to_owned())?;
        let offending_value = self
            .offending_value
            .ok_or_else(|| "offending_value is required".to_owned())?;
        let remediation_hint = self
            .remediation_hint
            .ok_or_else(|| "remediation_hint is required".to_owned())?;

        ValidationFinding::new(
            finding_type,
            location,
            offending_value,
            self.violated_rule_id,
            remediation_hint,
        )
    }
}
