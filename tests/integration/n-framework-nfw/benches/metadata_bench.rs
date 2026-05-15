use std::path::Path;
use std::time::{Duration, Instant};

use n_framework_nfw_core_application::features::generator_management::services::abstractions::validator::Validator;
use n_framework_nfw_core_application::features::generator_management::services::generator_catalog_parser::GeneratorCatalogParser;
use n_framework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use n_framework_nfw_infrastructure_yaml::features::generator_management::services::serde_yaml_parser::SerdeYamlParser;

#[derive(Debug, Default, Clone, Copy)]
struct BenchmarkValidator;

impl Validator for BenchmarkValidator {
    fn is_kebab_case(&self, value: &str) -> bool {
        if value.starts_with('-') || value.ends_with('-') || value.contains("--") {
            return false;
        }

        value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    }

    fn is_git_url(&self, value: &str) -> bool {
        value.starts_with("http://")
            || value.starts_with("https://")
            || value.starts_with("git@")
            || Path::new(value).exists()
    }
}

#[test]
fn metadata_validation_stays_under_target_threshold() {
    let parser = GeneratorCatalogParser::new(
        SerdeYamlParser::new(),
        BenchmarkValidator,
        SemverVersionComparator::new(),
    );
    let metadata = r#"
id: web-api
name: Web API
description: Standalone web api generator
version: 1.0.0
language: rust
tags:
  - api
  - service
author: nframework
minCliVersion: 0.1.0
sourceUrl: https://github.com/n-framework/nfw-generators
"#;

    let iterations = 100u32;
    let started_at = Instant::now();
    for _ in 0..iterations {
        parser
            .parse_generator_metadata(metadata)
            .expect("metadata should parse");
    }
    let elapsed = started_at.elapsed();
    let average_duration = elapsed / iterations;

    assert!(
        average_duration < Duration::from_millis(50),
        "average metadata validation took {:?}, expected < 50ms",
        average_duration
    );
}
