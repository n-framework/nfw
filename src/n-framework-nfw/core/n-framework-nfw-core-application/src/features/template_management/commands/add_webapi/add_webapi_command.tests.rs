use super::*;
use n_framework_nfw_infrastructure_workspace_metadata::PreservedComments;
use std::path::PathBuf;

#[test]
fn validates_empty_service_name() {
    let nfw_yaml = serde_yaml::from_str("workspace: {}").unwrap();
    let _ctx = WorkspaceContext::new(PathBuf::from("/"), nfw_yaml, PreservedComments::default());
    let svc = ServiceInfo::new("".to_string(), "path".to_string(), "t1".to_string());
    assert!(svc.is_err());
}

#[test]
fn can_create_valid_command() {
    let nfw_yaml = serde_yaml::from_str("workspace: {}").unwrap();
    let ctx = WorkspaceContext::new(PathBuf::from("/"), nfw_yaml, PreservedComments::default());
    let svc = ServiceInfo::new("Svc".to_string(), "path".to_string(), "t1".to_string()).unwrap();

    let cmd = AddWebApiCommand::new(svc.clone(), ctx.clone()).unwrap();
    assert_eq!(cmd.service_info().name(), "Svc");
}

#[test]
fn add_webapi_command_enforces_rules() {
    let nfw_yaml = serde_yaml::from_str("workspace: {}").unwrap();
    let _ctx = WorkspaceContext::new(PathBuf::from("."), nfw_yaml, PreservedComments::default());
    let svc = ServiceInfo::new("Svc".to_string(), "path".to_string(), "t1".to_string()).unwrap();
    let cmd = AddWebApiCommand::new(svc, _ctx).unwrap();

    assert_eq!(cmd.service_info().name(), "Svc");
}
