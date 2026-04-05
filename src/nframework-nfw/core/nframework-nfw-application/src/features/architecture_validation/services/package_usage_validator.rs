use std::path::Path;

use crate::features::architecture_validation::models::{
    ArchitectureLayer, ArchitectureRuleSet, FindingType, ValidationFinding,
};
use crate::features::architecture_validation::services::abstractions::PackageUsageValidator;
use crate::features::architecture_validation::services::remediation_hint_service::RemediationHintService;

#[derive(Debug, Default, Clone, Copy)]
pub struct PackageUsageValidatorService {
    remediation_hint_service: RemediationHintService,
}

impl PackageUsageValidatorService {
    pub fn new(remediation_hint_service: RemediationHintService) -> Self {
        Self {
            remediation_hint_service,
        }
    }
}

impl PackageUsageValidator for PackageUsageValidatorService {
    fn validate(
        &self,
        source_layer: ArchitectureLayer,
        source_project_path: &Path,
        direct_package_references: &[String],
        rules: &ArchitectureRuleSet,
    ) -> Vec<ValidationFinding> {
        let Some(forbidden_packages) = rules.forbidden_direct_packages.get(&source_layer) else {
            return Vec::new();
        };

        direct_package_references
            .iter()
            .filter_map(|package_name| {
                let is_forbidden = forbidden_packages
                    .iter()
                    .any(|forbidden| forbidden.eq_ignore_ascii_case(package_name));

                if !is_forbidden {
                    return None;
                }

                Some(ValidationFinding {
                    finding_type: FindingType::PackageUsage,
                    location: source_project_path.to_path_buf(),
                    offending_value: package_name.clone(),
                    violated_rule_id: Some(format!(
                        "package_usage:{source_layer:?}:{}",
                        package_name.to_ascii_lowercase()
                    )),
                    remediation_hint: self.remediation_hint_service.for_package_usage(package_name),
                })
            })
            .collect()
    }
}
