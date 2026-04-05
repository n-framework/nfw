use crate::features::architecture_validation::models::{
    ArchitectureLayer, ArchitectureRuleSet, ValidationFinding,
};

pub trait NamespaceUsageValidator {
    fn validate(
        &self,
        source_layer: ArchitectureLayer,
        source_file_path: &std::path::Path,
        source_text: &str,
        rules: &ArchitectureRuleSet,
    ) -> Vec<ValidationFinding>;
}
