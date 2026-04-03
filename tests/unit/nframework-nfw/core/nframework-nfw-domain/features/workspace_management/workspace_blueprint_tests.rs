use nframework_nfw_domain::features::workspace_management::namespace_convention::NamespaceConvention;
use nframework_nfw_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;

#[test]
fn creates_layered_workspace_blueprint_with_solution_files() {
    let blueprint = WorkspaceBlueprint::new("BillingPlatform");

    assert_eq!(blueprint.workspace_name, "BillingPlatform");
    assert_eq!(
        blueprint.root_directories,
        vec!["src".to_owned(), "tests".to_owned(), "docs".to_owned(),]
    );
}

#[test]
fn derives_namespace_convention_from_workspace_name() {
    let convention = NamespaceConvention::from_workspace_name("billing-platform");

    assert_eq!(convention.workspace_base_namespace, "BillingPlatform");
    assert_eq!(
        convention.service_namespace("orders-service"),
        "BillingPlatform.OrdersService.Service"
    );
}
