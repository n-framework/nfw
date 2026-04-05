#[path = "support.rs"]
mod support;

#[test]
fn generates_service_under_src_with_expected_layers() {
    let workspace_root = support::create_workspace_root("service-layout");
    let template_root =
        support::create_service_template(&workspace_root, "dotnet-service-template", "service");
    let template_resolution =
        support::create_template_resolution(&template_root, "official", "dotnet-service");
    let orchestration = support::build_default_orchestration(&workspace_root, template_resolution);

    let result = support::execute_non_interactive_add_service(
        &orchestration,
        "Orders",
        "official/dotnet-service",
    )
    .expect("service generation should succeed");

    let service_root = workspace_root.join("src/Orders");
    assert_eq!(result.output_path, service_root);
    assert!(service_root.join("Domain").is_dir());
    assert!(service_root.join("Application").is_dir());
    assert!(service_root.join("Infrastructure").is_dir());
    assert!(service_root.join("Api").is_dir());

    support::cleanup_sandbox_directory(&workspace_root);
}
