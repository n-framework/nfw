use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use regex::Regex;
use serde_json::Value as JsonValue;
use serde_yaml::Value as YamlValue;

use crate::features::architecture_validation::models::check_command_request::CheckCommandRequest;
use crate::features::architecture_validation::models::errors::ArchitectureValidationError;
use crate::features::architecture_validation::models::{
    ArchitectureLayer, CheckCommandResult, FindingType, ValidationFinding,
};
use crate::features::architecture_validation::services::abstractions::{
    NamespaceUsageValidator, PackageUsageValidator, ProjectReferenceValidator, RuleSetLoader,
};
use crate::features::architecture_validation::services::finding_aggregation_service::FindingAggregationService;
use crate::features::architecture_validation::services::namespace_usage_validator::NamespaceUsageValidatorService;
use crate::features::architecture_validation::services::package_usage_validator::PackageUsageValidatorService;
use crate::features::architecture_validation::services::project_reference_validator::ProjectReferenceValidatorService;
use crate::features::architecture_validation::services::remediation_hint_service::RemediationHintService;
use crate::features::architecture_validation::services::rule_set_loader::RuleSetLoaderService;

const WORKSPACE_METADATA_FILE: &str = "nfw.yaml";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectManifestKind {
    CSharp,
    Rust,
    Node,
}

#[derive(Debug, Clone)]
struct ProjectManifest {
    path: PathBuf,
    kind: ProjectManifestKind,
}

#[derive(Debug, Clone)]
pub struct CheckCommandHandler {
    rule_set_loader: RuleSetLoaderService,
    project_reference_validator: ProjectReferenceValidatorService,
    namespace_usage_validator: NamespaceUsageValidatorService,
    package_usage_validator: PackageUsageValidatorService,
    finding_aggregation_service: FindingAggregationService,
    remediation_hint_service: RemediationHintService,
    csharp_project_reference_regex: Regex,
    csharp_package_reference_regex: Regex,
    cargo_path_dependency_regex: Regex,
}

impl CheckCommandHandler {
    pub fn new(
        rule_set_loader: RuleSetLoaderService,
        project_reference_validator: ProjectReferenceValidatorService,
        namespace_usage_validator: NamespaceUsageValidatorService,
        package_usage_validator: PackageUsageValidatorService,
        finding_aggregation_service: FindingAggregationService,
        remediation_hint_service: RemediationHintService,
    ) -> Self {
        Self {
            rule_set_loader,
            project_reference_validator,
            namespace_usage_validator,
            package_usage_validator,
            finding_aggregation_service,
            remediation_hint_service,
            csharp_project_reference_regex: Regex::new(r#"<ProjectReference\s+Include="([^"]+)""#)
                .expect("csharp project reference regex should compile"),
            csharp_package_reference_regex: Regex::new(r#"<PackageReference\s+Include="([^"]+)""#)
                .expect("csharp package regex should compile"),
            cargo_path_dependency_regex: Regex::new(r#"(?m)^\s*([A-Za-z0-9_-]+)\s*=\s*\{[^}]*\bpath\s*=\s*"([^"]+)""#)
                .expect("cargo path dependency regex should compile"),
        }
    }

    pub fn handle(
        &self,
        command: &CheckCommandRequest,
    ) -> Result<CheckCommandResult, ArchitectureValidationError> {
        self.execute(command)
    }

    pub fn execute(
        &self,
        command: &CheckCommandRequest,
    ) -> Result<CheckCommandResult, ArchitectureValidationError> {
        let workspace_root = resolve_workspace_root(&command.start_directory)?;
        let rules = self.rule_set_loader.load();
        let project_manifests = collect_project_manifests(&workspace_root);

        let mut findings = Vec::new();

        for manifest in project_manifests {
            findings.extend(self.validate_manifest(&manifest, &rules));
        }

        if let Some(lint_finding) = self.run_make_lint_check(&workspace_root) {
            findings.push(lint_finding);
        }
        findings.extend(self.run_service_make_test_checks(&workspace_root));

        let (deduplicated, summary) = self
            .finding_aggregation_service
            .deduplicate_and_summarize(findings, false);

        Ok(CheckCommandResult {
            workspace_root,
            findings: deduplicated,
            summary,
        })
    }

    fn validate_manifest(
        &self,
        manifest: &ProjectManifest,
        rules: &crate::features::architecture_validation::models::ArchitectureRuleSet,
    ) -> Vec<ValidationFinding> {
        let source_layer = ArchitectureLayer::from_path(&manifest.path.to_string_lossy());
        let mut findings = Vec::new();

        let project_content = match fs::read_to_string(&manifest.path) {
            Ok(value) => value,
            Err(error) => {
                findings.push(self.unreadable_artifact_finding(
                    manifest.path.clone(),
                    format!("failed to read manifest file: {error}"),
                ));
                return findings;
            }
        };

        let project_references = self.extract_project_references(manifest, &project_content);
        findings.extend(self.project_reference_validator.validate(
            source_layer,
            &manifest.path,
            &project_references,
            rules,
        ));

        let direct_package_references = self.extract_direct_package_references(manifest, &project_content);
        findings.extend(self.package_usage_validator.validate(
            source_layer,
            &manifest.path,
            &direct_package_references,
            rules,
        ));

        let Some(project_directory) = manifest.path.parent() else {
            return findings;
        };

        let source_files = collect_source_files(project_directory);
        for source_file in source_files {
            match fs::read_to_string(&source_file) {
                Ok(content) => findings.extend(self.namespace_usage_validator.validate(
                    source_layer,
                    &source_file,
                    &content,
                    rules,
                )),
                Err(error) => findings.push(self.unreadable_artifact_finding(
                    source_file,
                    format!("failed to read source file: {error}"),
                )),
            }
        }

        findings
    }

    fn run_make_lint_check(&self, workspace_root: &Path) -> Option<ValidationFinding> {
        let output = Command::new("make")
            .arg("lint")
            .current_dir(workspace_root)
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    return None;
                }

                let stderr = String::from_utf8_lossy(&result.stderr).trim().to_owned();
                let stdout = String::from_utf8_lossy(&result.stdout).trim().to_owned();
                let message = if !stderr.is_empty() {
                    stderr
                } else if !stdout.is_empty() {
                    stdout
                } else {
                    "make lint failed without output".to_owned()
                };

                Some(ValidationFinding {
                    finding_type: FindingType::LintIssue,
                    location: workspace_root.to_path_buf(),
                    offending_value: message,
                    violated_rule_id: Some("lint:make-lint".to_owned()),
                    remediation_hint: self.remediation_hint_service.for_lint_issue(),
                })
            }
            Err(error) => Some(ValidationFinding {
                finding_type: FindingType::LintIssue,
                location: workspace_root.to_path_buf(),
                offending_value: format!("failed to execute `make lint`: {error}"),
                violated_rule_id: Some("lint:make-lint-execution".to_owned()),
                remediation_hint: self.remediation_hint_service.for_lint_issue(),
            }),
        }
    }

    fn run_service_make_test_checks(&self, workspace_root: &Path) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();
        let service_roots = match resolve_service_roots(workspace_root) {
            Ok(roots) => roots,
            Err(error) => {
                findings.push(self.unreadable_artifact_finding(
                    workspace_root.join(WORKSPACE_METADATA_FILE),
                    error,
                ));
                return findings;
            }
        };

        for service_root in service_roots {
            if let Some(test_finding) = self.run_make_test_check(&service_root) {
                findings.push(test_finding);
            }
        }

        findings
    }

    fn run_make_test_check(&self, service_root: &Path) -> Option<ValidationFinding> {
        let output = Command::new("make")
            .arg("test")
            .current_dir(service_root)
            .output();

        match output {
            Ok(result) => {
                if result.status.success() {
                    return None;
                }

                let stderr = String::from_utf8_lossy(&result.stderr).trim().to_owned();
                let stdout = String::from_utf8_lossy(&result.stdout).trim().to_owned();
                let message = if !stderr.is_empty() {
                    stderr
                } else if !stdout.is_empty() {
                    stdout
                } else {
                    "make test failed without output".to_owned()
                };

                Some(ValidationFinding {
                    finding_type: FindingType::TestIssue,
                    location: service_root.to_path_buf(),
                    offending_value: message,
                    violated_rule_id: Some("test:make-test".to_owned()),
                    remediation_hint: self.remediation_hint_service.for_test_issue(),
                })
            }
            Err(error) => Some(ValidationFinding {
                finding_type: FindingType::TestIssue,
                location: service_root.to_path_buf(),
                offending_value: format!("failed to execute `make test`: {error}"),
                violated_rule_id: Some("test:make-test-execution".to_owned()),
                remediation_hint: self.remediation_hint_service.for_test_issue(),
            }),
        }
    }

    fn extract_project_references(
        &self,
        manifest: &ProjectManifest,
        content: &str,
    ) -> Vec<String> {
        match manifest.kind {
            ProjectManifestKind::CSharp => self
                .csharp_project_reference_regex
                .captures_iter(content)
                .filter_map(|capture| capture.get(1).map(|value| value.as_str().to_owned()))
                .collect(),
            ProjectManifestKind::Rust => self
                .cargo_path_dependency_regex
                .captures_iter(content)
                .filter_map(|capture| capture.get(2).map(|value| value.as_str().to_owned()))
                .collect(),
            ProjectManifestKind::Node => {
                let Some(root) = parse_json(content) else {
                    return Vec::new();
                };
                extract_local_node_references(&root)
            }
        }
    }

    fn extract_direct_package_references(
        &self,
        manifest: &ProjectManifest,
        content: &str,
    ) -> Vec<String> {
        match manifest.kind {
            ProjectManifestKind::CSharp => self
                .csharp_package_reference_regex
                .captures_iter(content)
                .filter_map(|capture| capture.get(1).map(|value| value.as_str().to_owned()))
                .collect(),
            ProjectManifestKind::Rust => extract_cargo_direct_dependencies(content),
            ProjectManifestKind::Node => {
                let Some(root) = parse_json(content) else {
                    return Vec::new();
                };
                extract_node_direct_dependencies(&root)
            }
        }
    }

    fn unreadable_artifact_finding(&self, path: PathBuf, message: String) -> ValidationFinding {
        ValidationFinding {
            finding_type: FindingType::UnreadableArtifact,
            location: path,
            offending_value: message,
            violated_rule_id: None,
            remediation_hint: self.remediation_hint_service.for_unreadable_artifact(),
        }
    }
}

impl Default for CheckCommandHandler {
    fn default() -> Self {
        let remediation_hint_service = RemediationHintService::new();

        Self::new(
            RuleSetLoaderService::new(),
            ProjectReferenceValidatorService::new(remediation_hint_service),
            NamespaceUsageValidatorService::new(remediation_hint_service),
            PackageUsageValidatorService::new(remediation_hint_service),
            FindingAggregationService::new(),
            remediation_hint_service,
        )
    }
}

fn resolve_workspace_root(start_directory: &Path) -> Result<PathBuf, ArchitectureValidationError> {
    let mut candidate = start_directory.to_path_buf();

    loop {
        let workspace_file = candidate.join(WORKSPACE_METADATA_FILE);
        if workspace_file.is_file() {
            return Ok(candidate);
        }

        let Some(parent) = candidate.parent() else {
            break;
        };

        candidate = parent.to_path_buf();
    }

    Err(ArchitectureValidationError::InvalidWorkspaceContext(
        "could not find nfw.yaml in current directory or parent directories".to_owned(),
    ))
}

fn resolve_service_roots(workspace_root: &Path) -> Result<Vec<PathBuf>, String> {
    let workspace_metadata_path = workspace_root.join(WORKSPACE_METADATA_FILE);
    let metadata_content = fs::read_to_string(&workspace_metadata_path)
        .map_err(|error| format!("failed to read workspace metadata file: {error}"))?;
    let metadata = serde_yaml::from_str::<YamlValue>(&metadata_content)
        .map_err(|error| format!("failed to parse workspace metadata file: {error}"))?;

    let Some(services) = metadata.get("services").and_then(|value| value.as_mapping()) else {
        return Ok(Vec::new());
    };

    let mut service_roots = Vec::new();
    for service_definition in services.values() {
        let Some(service_mapping) = service_definition.as_mapping() else {
            continue;
        };
        let Some(path_value) = service_mapping
            .get(&YamlValue::String("path".to_owned()))
            .and_then(|value| value.as_str())
        else {
            continue;
        };

        let normalized = path_value.trim();
        if normalized.is_empty() {
            continue;
        }

        service_roots.push(workspace_root.join(normalized));
    }

    service_roots.sort();
    service_roots.dedup();
    Ok(service_roots)
}

fn collect_project_manifests(root: &Path) -> Vec<ProjectManifest> {
    let mut manifests = Vec::new();
    collect_project_manifests_recursive(root, &mut manifests);
    manifests.sort_by(|a, b| a.path.cmp(&b.path));
    manifests
}

fn collect_project_manifests_recursive(root: &Path, output: &mut Vec<ProjectManifest>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if should_skip_path(&path) {
            continue;
        }

        if path.is_dir() {
            collect_project_manifests_recursive(&path, output);
            continue;
        }

        if let Some(kind) = detect_manifest_kind(&path) {
            output.push(ProjectManifest { path, kind });
        }
    }
}

fn detect_manifest_kind(path: &Path) -> Option<ProjectManifestKind> {
    let file_name = path.file_name().and_then(|value| value.to_str())?;

    if file_name.ends_with(".csproj") {
        return Some(ProjectManifestKind::CSharp);
    }

    if file_name.eq_ignore_ascii_case("Cargo.toml") {
        return Some(ProjectManifestKind::Rust);
    }

    if file_name.eq_ignore_ascii_case("package.json") {
        return Some(ProjectManifestKind::Node);
    }

    None
}

fn collect_source_files(root: &Path) -> Vec<PathBuf> {
    let mut results = Vec::new();
    collect_source_files_recursive(root, &mut results);
    results.sort();
    results
}

fn collect_source_files_recursive(root: &Path, output: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if should_skip_path(&path) {
            continue;
        }

        if path.is_dir() {
            collect_source_files_recursive(&path, output);
            continue;
        }

        let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
            continue;
        };

        let supported = matches!(
            extension.to_ascii_lowercase().as_str(),
            "cs" | "rs" | "js" | "jsx" | "ts" | "tsx" | "py" | "go"
        );

        if supported {
            output.push(path);
        }
    }
}

fn should_skip_path(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };

    matches!(
        file_name,
        ".git"
            | "bin"
            | "obj"
            | "target"
            | "node_modules"
            | ".idea"
            | ".vscode"
            | ".venv"
            | "venv"
            | "__pycache__"
    )
}

fn parse_json(content: &str) -> Option<JsonValue> {
    serde_json::from_str::<JsonValue>(content).ok()
}

fn extract_node_direct_dependencies(root: &JsonValue) -> Vec<String> {
    let mut dependencies = Vec::new();

    for section in ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"] {
        let Some(mapping) = root.get(section).and_then(|value| value.as_object()) else {
            continue;
        };

        dependencies.extend(mapping.keys().cloned());
    }

    dependencies.sort();
    dependencies.dedup();
    dependencies
}

fn extract_local_node_references(root: &JsonValue) -> Vec<String> {
    let mut references = Vec::new();

    for section in ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"] {
        let Some(mapping) = root.get(section).and_then(|value| value.as_object()) else {
            continue;
        };

        for value in mapping.values() {
            let Some(specifier) = value.as_str() else {
                continue;
            };

            let normalized = specifier.to_ascii_lowercase();
            let is_local = normalized.starts_with("file:")
                || normalized.starts_with("workspace:")
                || normalized.starts_with("../")
                || normalized.starts_with("./");
            if is_local {
                references.push(specifier.to_owned());
            }
        }
    }

    references.sort();
    references.dedup();
    references
}

fn extract_cargo_direct_dependencies(content: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    let mut inside_dependencies_section = false;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.starts_with('[') && line.ends_with(']') {
            let section_name = line.trim_start_matches('[').trim_end_matches(']');
            inside_dependencies_section = matches!(
                section_name,
                "dependencies" | "dev-dependencies" | "build-dependencies"
            );
            continue;
        }

        if !inside_dependencies_section || line.is_empty() || line.starts_with('#') {
            continue;
        }

        let Some((name, _)) = line.split_once('=') else {
            continue;
        };
        let dependency_name = name.trim();
        if dependency_name.is_empty() {
            continue;
        }

        dependencies.push(dependency_name.to_owned());
    }

    dependencies.sort();
    dependencies.dedup();
    dependencies
}
