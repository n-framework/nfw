use crate::features::architecture_validation::models::{
    ArchitectureLayer, ArchitectureRuleSet, ValidationFinding,
};

pub trait PackageUsageValidator {
    fn validate(
        &self,
        source_layer: ArchitectureLayer,
        source_project_path: &std::path::Path,
        direct_package_references: &[String],
        rules: &ArchitectureRuleSet,
    ) -> Vec<ValidationFinding>;
}
