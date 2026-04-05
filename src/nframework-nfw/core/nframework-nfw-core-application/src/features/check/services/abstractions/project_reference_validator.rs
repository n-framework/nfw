use crate::features::check::models::{CheckLayer, CheckRuleSet, ValidationFinding};

pub trait ProjectReferenceValidator {
    fn validate(
        &self,
        source_layer: CheckLayer,
        source_project_path: &std::path::Path,
        project_references: &[String],
        rules: &CheckRuleSet,
    ) -> Vec<ValidationFinding>;
}
