use crate::features::check::models::{CheckLayer, CheckRuleSet, ValidationFinding};

pub trait NamespaceUsageValidator {
    fn validate(
        &self,
        source_layer: CheckLayer,
        source_file_path: &std::path::Path,
        source_text: &str,
        rules: &CheckRuleSet,
    ) -> Vec<ValidationFinding>;
}
