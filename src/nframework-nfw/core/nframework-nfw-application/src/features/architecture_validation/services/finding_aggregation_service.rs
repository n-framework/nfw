use std::collections::BTreeMap;

use crate::features::architecture_validation::models::{ValidationFinding, ValidationSummary};

#[derive(Debug, Default, Clone, Copy)]
pub struct FindingAggregationService;

impl FindingAggregationService {
    pub fn new() -> Self {
        Self
    }

    pub fn deduplicate_and_summarize(
        &self,
        findings: Vec<ValidationFinding>,
        interrupted: bool,
    ) -> (Vec<ValidationFinding>, ValidationSummary) {
        let mut map = BTreeMap::new();
        for finding in findings {
            map.entry(finding.dedupe_key()).or_insert(finding);
        }

        let deduplicated = map.into_values().collect::<Vec<_>>();
        let summary = ValidationSummary::from_findings(&deduplicated, interrupted);

        (deduplicated, summary)
    }
}
