use super::*;
use crate::features::entity_generation::errors::entity_generation_error::EntityGenerationError;
use crate::features::entity_generation::value_objects::workspace_context::WorkspaceContext;
use std::path::PathBuf;

#[test]
fn service_info_try_new_validates_name() {
    let result = ServiceInfo::try_new("invalid name".to_owned(), PathBuf::from("path"), vec![]);
    assert!(result.is_err());

    let result = ServiceInfo::try_new("ValidService".to_owned(), PathBuf::from("path"), vec![]);
    assert!(result.is_ok());
}

#[test]
fn workspace_context_try_new_detects_duplicate_services() {
    let s1 = ServiceInfo::new("S1".to_owned(), PathBuf::from("p1"), vec![]);
    let s2 = ServiceInfo::new("S1".to_owned(), PathBuf::from("p2"), vec![]);

    let result = WorkspaceContext::try_new(PathBuf::from("root"), vec![s1, s2]);

    assert!(result.is_err());
    if let Err(EntityGenerationError::InvalidEntityName { reason, .. }) = result {
        assert!(reason.contains("duplicate service name"));
    } else {
        panic!("Expected duplicate service name error");
    }
}
