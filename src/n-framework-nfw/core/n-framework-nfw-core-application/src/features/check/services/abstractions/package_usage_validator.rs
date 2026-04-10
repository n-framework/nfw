use crate::features::check::models::{CheckLayer, CheckRuleSet, ValidationFinding};

pub trait PackageUsageValidator {
    fn validate(
        &self,
        source_layer: CheckLayer,
        source_project_path: &std::path::Path,
        direct_package_references: &[String],
        rules: &CheckRuleSet,
    ) -> Vec<ValidationFinding>;
}
