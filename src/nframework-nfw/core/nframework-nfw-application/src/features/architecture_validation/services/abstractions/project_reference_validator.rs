use crate::features::architecture_validation::models::{
    ArchitectureLayer, ArchitectureRuleSet, ValidationFinding,
};

pub trait ProjectReferenceValidator {
    fn validate(
        &self,
        source_layer: ArchitectureLayer,
        source_project_path: &std::path::Path,
        project_references: &[String],
        rules: &ArchitectureRuleSet,
    ) -> Vec<ValidationFinding>;
}
