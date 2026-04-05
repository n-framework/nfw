use std::path::Path;

use crate::features::architecture_validation::models::{
    ArchitectureLayer, ArchitectureRuleSet, FindingType, ValidationFinding,
};
use crate::features::architecture_validation::services::abstractions::ProjectReferenceValidator;
use crate::features::architecture_validation::services::remediation_hint_service::RemediationHintService;

#[derive(Debug, Default, Clone, Copy)]
pub struct ProjectReferenceValidatorService {
    remediation_hint_service: RemediationHintService,
}

impl ProjectReferenceValidatorService {
    pub fn new(remediation_hint_service: RemediationHintService) -> Self {
        Self {
            remediation_hint_service,
        }
    }
}

impl ProjectReferenceValidator for ProjectReferenceValidatorService {
    fn validate(
        &self,
        source_layer: ArchitectureLayer,
        source_project_path: &Path,
        project_references: &[String],
        rules: &ArchitectureRuleSet,
    ) -> Vec<ValidationFinding> {
        let Some(forbidden_targets) = rules.forbidden_project_references.get(&source_layer) else {
            return Vec::new();
        };

        project_references
            .iter()
            .filter_map(|project_reference| {
                let target_layer = ArchitectureLayer::from_path(project_reference);
                if !forbidden_targets.contains(&target_layer) {
                    return None;
                }

                Some(ValidationFinding {
                    finding_type: FindingType::ProjectReference,
                    location: source_project_path.to_path_buf(),
                    offending_value: project_reference.clone(),
                    violated_rule_id: Some(format!(
                        "project_reference:{source_layer:?}->{target_layer:?}"
                    )),
                    remediation_hint: self
                        .remediation_hint_service
                        .for_project_reference(source_layer, target_layer),
                })
            })
            .collect()
    }
}
