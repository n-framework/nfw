use super::*;
use crate::features::check::models::FindingType;

#[test]
fn provides_hint_for_namespace_usage() {
    let service = RemediationHintService;
    let hint = service.for_namespace_usage("NFramework.Internal");
    assert!(hint.contains("replace forbidden namespace usage"));
    assert!(hint.contains("'NFramework.Internal'"));
}

#[test]
fn provides_hint_for_finding_type() {
    let service = RemediationHintService;
    let hint = service.for_finding_type(FindingType::LintIssue);
    assert!(hint.contains("run `make lint`"));
}

#[test]
fn provides_hint_for_unreadable_artifact() {
    let service = RemediationHintService;
    let hint = service.for_unreadable_artifact();
    assert!(hint.contains("fix file permissions"));
}
