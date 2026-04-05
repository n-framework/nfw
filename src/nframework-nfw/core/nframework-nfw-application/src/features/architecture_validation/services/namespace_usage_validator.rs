use regex::Regex;
use std::collections::BTreeSet;
use std::path::Path;

use crate::features::architecture_validation::models::{
    ArchitectureLayer, ArchitectureRuleSet, FindingType, ValidationFinding,
};
use crate::features::architecture_validation::services::abstractions::NamespaceUsageValidator;
use crate::features::architecture_validation::services::remediation_hint_service::RemediationHintService;

#[derive(Debug, Clone)]
pub struct NamespaceUsageValidatorService {
    import_like_regexes: Vec<Regex>,
    remediation_hint_service: RemediationHintService,
}

impl NamespaceUsageValidatorService {
    pub fn new(remediation_hint_service: RemediationHintService) -> Self {
        Self {
            import_like_regexes: vec![
                Regex::new(r"(?m)^\s*(?:using|namespace)\s+([A-Za-z0-9_.]+)")
                    .expect("csharp namespace regex should compile"),
                Regex::new(r"(?m)^\s*use\s+([A-Za-z0-9_:]+)")
                    .expect("rust use regex should compile"),
                Regex::new(r#"(?m)^\s*import\s+.*?\s+from\s+['"]([^'"]+)['"]"#)
                    .expect("js import-from regex should compile"),
                Regex::new(r#"(?m)^\s*import\s+['"]([^'"]+)['"]"#)
                    .expect("js side effect import regex should compile"),
                Regex::new(r#"(?m)^\s*(?:const|let|var)\s+.*=\s*require\(['"]([^'"]+)['"]\)"#)
                    .expect("js require regex should compile"),
                Regex::new(r"(?m)^\s*from\s+([A-Za-z0-9_.]+)\s+import\s+")
                    .expect("python from-import regex should compile"),
                Regex::new(r"(?m)^\s*import\s+([A-Za-z0-9_.]+)")
                    .expect("python import regex should compile"),
                Regex::new(r#"(?m)^\s*import\s+['"]([^'"]+)['"]"#)
                    .expect("go import regex should compile"),
            ],
            remediation_hint_service,
        }
    }
}

impl Default for NamespaceUsageValidatorService {
    fn default() -> Self {
        Self::new(RemediationHintService::new())
    }
}

impl NamespaceUsageValidator for NamespaceUsageValidatorService {
    fn validate(
        &self,
        source_layer: ArchitectureLayer,
        source_file_path: &Path,
        source_text: &str,
        rules: &ArchitectureRuleSet,
    ) -> Vec<ValidationFinding> {
        let Some(forbidden_prefixes) = rules.forbidden_namespace_prefixes.get(&source_layer) else {
            return Vec::new();
        };

        let mut findings = Vec::new();
        let mut seen = BTreeSet::new();
        let normalized_prefixes = forbidden_prefixes
            .iter()
            .map(|prefix| (prefix.clone(), normalize_symbol(prefix)))
            .collect::<Vec<_>>();

        for regex in &self.import_like_regexes {
            for namespace_value in regex
                .captures_iter(source_text)
                .filter_map(|capture| capture.get(1).map(|value| value.as_str().to_owned()))
            {
                let normalized_namespace = normalize_symbol(&namespace_value);

                for (raw_prefix, normalized_prefix) in &normalized_prefixes {
                    if !normalized_namespace.starts_with(normalized_prefix) {
                        continue;
                    }

                    let dedupe_key = format!(
                        "{namespace_value}|{normalized_prefix}|{}",
                        source_file_path.display()
                    );
                    if !seen.insert(dedupe_key) {
                        continue;
                    }

                    findings.push(ValidationFinding {
                        finding_type: FindingType::NamespaceUsage,
                        location: source_file_path.to_path_buf(),
                        offending_value: namespace_value.clone(),
                        violated_rule_id: Some(format!(
                            "namespace_usage:{source_layer:?}:{raw_prefix}"
                        )),
                        remediation_hint: self
                            .remediation_hint_service
                            .for_namespace_usage(raw_prefix),
                    });
                }
            }
        }

        findings
    }
}

fn normalize_symbol(value: &str) -> String {
    value
        .to_ascii_lowercase()
        .replace("::", ".")
        .replace('/', ".")
        .replace('\\', ".")
        .replace('-', ".")
}
