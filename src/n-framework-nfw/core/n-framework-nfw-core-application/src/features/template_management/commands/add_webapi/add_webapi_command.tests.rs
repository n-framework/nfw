use super::*;
use n_framework_nfw_infrastructure_workspace_metadata::PreservedComments;
use std::path::PathBuf;

#[test]
fn validates_empty_service_name() {
    let service_info = ServiceInfo::new("".to_string(), "path".to_string(), "t1".to_string());
    assert!(service_info.is_err());
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
    let nfw_yaml: serde_yaml::Value = serde_yaml::from_str("workspace:\n  namespace: TestApp\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1").unwrap();
    let ctx = WorkspaceContext::new(
        PathBuf::from("/tmp"),
        nfw_yaml,
        n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
    );
    let cmd = AddWebApiCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        ctx,
        WebApiConfig::default(),
    );

    assert_eq!(cmd.service_info().name(), "Svc1");
    assert_eq!(cmd.workspace_context().workspace_root(), PathBuf::from("/tmp"));
    assert!(cmd.config().use_openapi);
}
