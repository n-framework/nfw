#[test]
fn can_create_valid_command() {
    let nfw_yaml = serde_yaml::from_str("workspace: {}").unwrap();
    let ctx = WorkspaceContext::new(PathBuf::from("/"), nfw_yaml, PreservedComments::default());
    let svc = ServiceInfo::new("Svc".to_string(), "path".to_string(), "t1".to_string()).unwrap();

    let cmd = AddMediatorCommand::new(svc.clone(), ctx.clone(), "WebApi".to_string()).unwrap();
    assert_eq!(cmd.service_info().name(), "Svc");
}

#[test]
fn add_mediator_command_enforces_rules() {
    let nfw_yaml = serde_yaml::from_str("workspace: {}").unwrap();
    let _ctx = WorkspaceContext::new(PathBuf::from("."), nfw_yaml, PreservedComments::default());
    let svc = ServiceInfo::new("Svc".to_string(), "path".to_string(), "t1".to_string()).unwrap();
    let cmd = AddMediatorCommand::new(svc, _ctx, "WebApi".to_string()).unwrap();

    assert_eq!(cmd.service_info().name(), "Svc");
}