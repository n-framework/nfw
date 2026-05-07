// ... existing code ...
    let nfw_yaml = serde_yaml::from_str("workspace:\n  namespace: MyProj\nservices:\n  Svc1:\n    path: src/Svc1\n    template:\n      id: t1").unwrap();
    let cmd = AddMediatorCommand::new(
        ServiceInfo::new("Svc1".to_string(), "src/Svc1".to_string(), "t1".to_string()).unwrap(),
        WorkspaceContext::new(
            PathBuf::from("/mock/workspace"),
            nfw_yaml,
            n_framework_nfw_infrastructure_workspace_metadata::PreservedComments::default(),
        ),
        "WebApi".to_string(),
    )
    .unwrap();

    let result = handler.handle(&cmd);
// ... existing code ...