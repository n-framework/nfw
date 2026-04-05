use nframework_nfw_core_application::features::check::models::FindingType;
use nframework_nfw_core_application::features::check::services::remediation_hint_service::RemediationHintService;

#[test]
fn remediation_hint_service_returns_hint_for_each_finding_type() {
    let service = RemediationHintService::new();

    let project_reference_hint = service.for_finding_type(FindingType::ProjectReference);
    let namespace_hint = service.for_finding_type(FindingType::NamespaceUsage);
    let package_hint = service.for_finding_type(FindingType::PackageUsage);
    let unreadable_hint = service.for_finding_type(FindingType::UnreadableArtifact);
    let lint_hint = service.for_finding_type(FindingType::LintIssue);
    let test_hint = service.for_finding_type(FindingType::TestIssue);

    assert!(project_reference_hint.contains("reference"));
    assert!(namespace_hint.contains("namespace"));
    assert!(package_hint.contains("package"));
    assert!(unreadable_hint.contains("permissions"));
    assert!(lint_hint.contains("make lint"));
    assert!(test_hint.contains("make test"));
}
