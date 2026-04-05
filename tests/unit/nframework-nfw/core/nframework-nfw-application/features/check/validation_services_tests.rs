use std::path::Path;

use nframework_nfw_application::features::check::models::{CheckLayer, FindingType};
use nframework_nfw_application::features::check::services::abstractions::{
    NamespaceUsageValidator, PackageUsageValidator, ProjectReferenceValidator, RuleSetLoader,
};
use nframework_nfw_application::features::check::services::namespace_usage_validator::NamespaceUsageValidatorService;
use nframework_nfw_application::features::check::services::package_usage_validator::PackageUsageValidatorService;
use nframework_nfw_application::features::check::services::project_reference_validator::ProjectReferenceValidatorService;
use nframework_nfw_application::features::check::services::remediation_hint_service::RemediationHintService;
use nframework_nfw_application::features::check::services::rule_set_loader::RuleSetLoaderService;

#[test]
fn project_reference_validator_detects_forbidden_reference() {
    let rules = RuleSetLoaderService::new().load();
    let validator = ProjectReferenceValidatorService::new(RemediationHintService::new());

    let findings = validator.validate(
        CheckLayer::Domain,
        Path::new("/tmp/NFramework.Domain.csproj"),
        &["../application/NFramework.Application.csproj".to_owned()],
        &rules,
    );

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].finding_type, FindingType::ProjectReference);
}

#[test]
fn namespace_validator_detects_forbidden_namespace_prefix() {
    let rules = RuleSetLoaderService::new().load();
    let validator = NamespaceUsageValidatorService::new(RemediationHintService::new());

    let findings = validator.validate(
        CheckLayer::Domain,
        Path::new("/tmp/DomainModel.cs"),
        "using NFramework.Infrastructure.Persistence;\nnamespace NFramework.Domain;",
        &rules,
    );

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].finding_type, FindingType::NamespaceUsage);
}

#[test]
fn package_validator_detects_forbidden_direct_package_only() {
    let rules = RuleSetLoaderService::new().load();
    let validator = PackageUsageValidatorService::new(RemediationHintService::new());

    let findings = validator.validate(
        CheckLayer::Domain,
        Path::new("/tmp/NFramework.Domain.csproj"),
        &[
            "Microsoft.AspNetCore.App".to_owned(),
            "Some.Safe.Package".to_owned(),
        ],
        &rules,
    );

    assert_eq!(findings.len(), 1);
    assert_eq!(findings[0].finding_type, FindingType::PackageUsage);
    assert!(
        findings[0]
            .offending_value
            .contains("Microsoft.AspNetCore.App")
    );
}
