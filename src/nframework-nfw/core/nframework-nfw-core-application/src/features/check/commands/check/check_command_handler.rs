use std::path::{Path, PathBuf};

use regex::Regex;

use crate::features::check::models::check_command_request::CheckCommandRequest;
use crate::features::check::models::{
    CheckCommandResult, CheckLayer, CheckRuleSet, FindingType, ValidationFinding,
    ValidationServices,
};
use crate::features::check::services::abstractions::RuleSetLoader;
use crate::features::check::services::rule_set_loader::RuleSetLoaderService;

const WORKSPACE_METADATA_FILE: &str = "nfw.yaml";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectManifestKind {
    CSharp,
    Rust,
    Go,
}

#[derive(Debug, Clone)]
struct ProjectManifest {
    path: PathBuf,
    kind: ProjectManifestKind,
}

/// Refactored CheckCommandHandler that uses abstractions for I/O operations.
pub struct CheckCommandHandler {
    services: ValidationServices,
    rule_set_loader: RuleSetLoaderService,
    csharp_project_reference_regex: Regex,
    csharp_package_reference_regex: Regex,
    cargo_path_dependency_regex: Regex,
}

impl CheckCommandHandler {
    /// Creates a new CheckCommandHandler with the provided services.
    /// Returns Result to avoid .expect() panics from regex compilation.
    pub fn new(
        services: ValidationServices,
        rule_set_loader: RuleSetLoaderService,
    ) -> Result<Self, String> {
        Ok(Self {
            services,
            rule_set_loader,
            csharp_project_reference_regex: Regex::new(r#"<ProjectReference\s+Include="([^"]+)""#)
                .map_err(|e| format!("invalid regex for C# project references: {e}"))?,
            csharp_package_reference_regex: Regex::new(r#"<PackageReference\s+Include="([^"]+)""#)
                .map_err(|e| format!("invalid regex for C# package references: {e}"))?,
            cargo_path_dependency_regex: Regex::new(
                r#"(?m)^\s*([A-Za-z0-9_-]+)\s*=\s*\{[^}]*\bpath\s*=\s*"([^"]+)""#,
            )
            .map_err(|e| format!("invalid regex for Cargo path dependencies: {e}"))?,
        })
    }

    /// Executes the check command with the provided request.
    pub fn execute(&self, command: &CheckCommandRequest) -> Result<CheckCommandResult, String> {
        let workspace_root = self
            .services
            .workspace_metadata_reader
            .resolve_workspace_root(&command.start_directory)
            .map_err(|error| error.to_string())?;

        let rules = self.rule_set_loader.load();
        let mut errors = Vec::new();
        let manifest_paths = self
            .services
            .manifest_collector
            .collect_manifests(&workspace_root, &mut errors);

        let mut findings = Vec::new();
        findings.extend(errors); // Add collection errors to findings

        // Convert PathBuf to ProjectManifest by detecting kind
        let project_manifests: Vec<ProjectManifest> = manifest_paths
            .into_iter()
            .filter_map(|path| {
                let kind = detect_manifest_kind(&path)?;
                Some(ProjectManifest { path, kind })
            })
            .collect();

        for manifest in project_manifests {
            findings.extend(self.validate_manifest(&manifest, &rules));
        }

        findings.extend(self.run_service_make_checks(&workspace_root));

        let (deduplicated, summary) = self
            .services
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
        rules: &CheckRuleSet,
    ) -> Vec<ValidationFinding> {
        let source_layer = CheckLayer::from_path(&manifest.path.to_string_lossy());
        let mut findings = Vec::new();

        let project_content = match self
            .services
            .workspace_metadata_reader
            .read_manifest_content(&manifest.path)
        {
            Ok(value) => value,
            Err(error) => {
                findings.push(self.unreadable_artifact_finding(manifest.path.clone(), error));
                return findings;
            }
        };

        let project_references = self.extract_project_references(manifest, &project_content);
        findings.extend(self.services.project_reference_validator.validate(
            source_layer,
            &manifest.path,
            &project_references,
            rules,
        ));

        let direct_package_references =
            self.extract_direct_package_references(manifest, &project_content);
        findings.extend(self.services.package_usage_validator.validate(
            source_layer,
            &manifest.path,
            &direct_package_references,
            rules,
        ));

        let Some(project_directory) = manifest.path.parent() else {
            return findings;
        };

        let mut errors = Vec::new();
        let source_files = self
            .services
            .source_file_collector
            .collect_source_files(project_directory, &mut errors);
        findings.extend(errors); // Add collection errors to findings

        for source_file in source_files {
            match self
                .services
                .workspace_metadata_reader
                .read_source_file(&source_file)
            {
                Ok(content) => findings.extend(self.services.namespace_usage_validator.validate(
                    source_layer,
                    &source_file,
                    &content,
                    rules,
                )),
                Err(error) => findings.push(self.unreadable_artifact_finding(source_file, error)),
            }
        }

        findings
    }

    fn run_service_make_checks(&self, workspace_root: &Path) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();
        let service_roots = match self
            .services
            .workspace_metadata_reader
            .resolve_service_roots(workspace_root)
        {
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
            if let Some(lint_finding) = self
                .services
                .external_tool_runner
                .run_make_lint(&service_root)
            {
                findings.push(lint_finding);
            }

            if let Some(test_finding) = self
                .services
                .external_tool_runner
                .run_make_test(&service_root)
            {
                findings.push(test_finding);
            }
        }

        findings
    }

    fn extract_project_references(&self, manifest: &ProjectManifest, content: &str) -> Vec<String> {
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
            ProjectManifestKind::Go => extract_go_replace_directives(content),
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
            ProjectManifestKind::Go => extract_go_direct_dependencies(content),
        }
    }

    fn unreadable_artifact_finding(&self, path: PathBuf, message: String) -> ValidationFinding {
        ValidationFinding {
            finding_type: FindingType::UnreadableArtifact,
            location: path,
            offending_value: message,
            violated_rule_id: None,
            remediation_hint: self
                .services
                .remediation_hint_service
                .for_unreadable_artifact(),
        }
    }
}

/// Parses JSON and returns None, with errors already reported as findings elsewhere.
fn extract_go_replace_directives(content: &str) -> Vec<String> {
    let mut references = Vec::new();

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.starts_with("replace ") || line.starts_with("replace\t") {
            if let Some((_, rest)) = line.split_once("=>") {
                let rest = rest.trim();
                let path = rest
                    .trim_start_matches('"')
                    .trim_end_matches('"')
                    .split_whitespace()
                    .next()
                    .unwrap_or(rest);
                if path.starts_with("./") || path.starts_with("../") {
                    references.push(path.to_owned());
                }
            }
        }
    }

    references.sort();
    references.dedup();
    references
}

fn extract_go_direct_dependencies(content: &str) -> Vec<String> {
    let mut dependencies = Vec::new();
    let mut inside_require_block = false;

    for raw_line in content.lines() {
        let line = raw_line.trim();

        if line.starts_with("require (") {
            inside_require_block = true;
            continue;
        }

        if inside_require_block {
            if line == ")" {
                inside_require_block = false;
                continue;
            }
            if line.is_empty() || line.starts_with("//") {
                continue;
            }
            let parts: Vec<&str> = line.split_whitespace().collect();
            if !parts.is_empty() {
                dependencies.push(parts[0].to_owned());
            }
            continue;
        }

        if line.starts_with("require ") || line.starts_with("require\t") {
            let rest = line.trim_start_matches("require").trim();
            if !rest.starts_with('(') {
                let parts: Vec<&str> = rest.split_whitespace().collect();
                if !parts.is_empty() {
                    dependencies.push(parts[0].to_owned());
                }
            }
        }
    }

    dependencies.sort();
    dependencies.dedup();
    dependencies
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

impl Default for CheckCommandHandler {
    fn default() -> Self {
        use crate::features::check::services::{
            filesystem_project_manifest_collector::FilesystemProjectManifestCollector,
            filesystem_source_file_collector::FilesystemSourceFileCollector,
            filesystem_workspace_metadata_reader::FilesystemWorkspaceMetadataReader,
            namespace_usage_validator::NamespaceUsageValidatorService,
            package_usage_validator::PackageUsageValidatorService,
            process_external_tool_runner::ProcessExternalToolRunner,
            project_reference_validator::ProjectReferenceValidatorService,
        };

        let remediation_hint_service =
            crate::features::check::services::remediation_hint_service::RemediationHintService::new(
            );

        let services = ValidationServices::builder()
            .project_reference_validator(Box::new(ProjectReferenceValidatorService::new(
                remediation_hint_service,
            )))
            .namespace_usage_validator(Box::new(NamespaceUsageValidatorService::new(
                remediation_hint_service,
            )))
            .package_usage_validator(Box::new(PackageUsageValidatorService::new(
                remediation_hint_service,
            )))
            .manifest_collector(Box::new(FilesystemProjectManifestCollector::new()))
            .source_file_collector(Box::new(FilesystemSourceFileCollector::new()))
            .workspace_metadata_reader(Box::new(FilesystemWorkspaceMetadataReader::new()))
            .external_tool_runner(Box::new(ProcessExternalToolRunner::new(
                remediation_hint_service,
            )))
            .finding_aggregation_service(
                crate::features::check::services::finding_aggregation_service::FindingAggregationService::new(),
            )
            .remediation_hint_service(remediation_hint_service)
            .build();

        Self::new(services, RuleSetLoaderService::new()).expect("regex patterns should be valid")
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

    if file_name.eq_ignore_ascii_case("go.mod") {
        return Some(ProjectManifestKind::Go);
    }

    None
}
