use super::*;
use crate::features::entity_generation::errors::entity_generation_error::EntityGenerationError;
use crate::features::entity_generation::value_objects::workspace_context::WorkspaceContext;
use std::path::PathBuf;
use tempfile;

#[test]
fn service_info_try_new_validates_name() {
    let temp = tempfile::tempdir().unwrap();
    let path = temp.path().to_path_buf();

    let result = ServiceInfo::try_new("invalid name".to_owned(), path.clone(), vec![]);
    assert!(result.is_err());

    let result = ServiceInfo::try_new("ValidService".to_owned(), path, vec![]);
    assert!(result.is_ok());
}

#[test]
fn service_info_try_new_validates_path_existence() {
    let path = PathBuf::from("non_existent_path_12345");
    let result = ServiceInfo::try_new("ValidService".to_owned(), path, vec![]);
    assert!(result.is_err());
    if let Err(EntityGenerationError::ConfigError { reason }) = result {
        assert!(reason.contains("Service directory does not exist"));
    } else {
        panic!("Expected ConfigError for missing path");
    }
}

#[test]
fn workspace_context_try_new_validates_root_existence() {
    let s1 = ServiceInfo::new("S1".to_owned(), PathBuf::from("p1"), vec![]);
    let root = PathBuf::from("non_existent_root_12345");
    let result = WorkspaceContext::try_new(root, vec![s1]);

    assert!(result.is_err());
    if let Err(EntityGenerationError::WorkspaceError { reason }) = result {
        assert!(reason.contains("Workspace root directory does not exist"));
    } else {
        panic!("Expected WorkspaceError for missing root");
    }
}

#[test]
fn workspace_context_try_new_validates_non_empty_services() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().to_path_buf();
    let result = WorkspaceContext::try_new(root, vec![]);

    assert!(result.is_err());
    assert!(matches!(
        result,
        Err(EntityGenerationError::NoServicesInWorkspace)
    ));
}

#[test]
fn workspace_context_try_new_detects_duplicate_services() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().to_path_buf();
    let s1 = ServiceInfo::new("S1".to_owned(), PathBuf::from("p1"), vec![]);
    let s2 = ServiceInfo::new("S1".to_owned(), PathBuf::from("p2"), vec![]);

    let result = WorkspaceContext::try_new(root, vec![s1, s2]);

    assert!(result.is_err());
    if let Err(EntityGenerationError::InvalidEntityName { reason, .. }) = result {
        assert!(reason.contains("duplicate service name"));
    } else {
        panic!("Expected duplicate service name error");
    }
}
