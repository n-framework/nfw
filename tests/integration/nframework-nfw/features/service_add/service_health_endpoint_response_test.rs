#[path = "support.rs"]
mod support;

use nframework_nfw_infrastructure_filesystem::features::service_management::services::generated_api_contract_inspector::FileSystemGeneratedApiContractInspector;
use nframework_nfw_application::features::service_management::services::abstraction::generated_api_contract_inspector::GeneratedApiContractInspector;

#[test]
fn generated_api_health_contract_inspection_passes() {
    let workspace_root = support::create_workspace_root("service-health-response");
    let template_root = support::create_service_template(
        &workspace_root,
        "dotnet-service-template",
        "service",
        true,
        true,
    );
    let template_resolution =
        support::create_template_resolution(&template_root, "official", "dotnet-service");
    let orchestration = support::build_default_orchestration(&workspace_root, template_resolution);

    support::execute_non_interactive_add_service(
        &orchestration,
        "Orders",
        "official/dotnet-service",
    )
    .expect("service generation should succeed");

    let inspector = FileSystemGeneratedApiContractInspector::new();
    inspector
        .assert_health_endpoints(&workspace_root.join("src/Orders"))
        .expect("health contract inspection should pass");

    support::cleanup_sandbox_directory(&workspace_root);
}

#[test]
fn generated_web_api_health_contract_inspection_passes() {
    let workspace_root = support::create_workspace_root("service-health-response-webapi");
    let service_root = workspace_root.join("src/Orders");
    let web_api_root = service_root.join("presentation/Orders.WebApi");
    std::fs::create_dir_all(&web_api_root).expect("web api directory should be created");
    std::fs::write(
        web_api_root.join("Program.cs"),
        "var app = builder.Build();\napp.MapGet(\"/health/live\", () => Results.Ok());\napp.MapGet(\"/health/ready\", () => Results.Ok());\napp.Run();\n",
    )
    .expect("program should be written");

    let inspector = FileSystemGeneratedApiContractInspector::new();
    inspector
        .assert_health_endpoints(&service_root)
        .expect("health contract inspection should pass");

    support::cleanup_sandbox_directory(&workspace_root);
}

#[test]
fn generated_web_api_health_contract_inspection_passes_with_extension_mapping() {
    let workspace_root = support::create_workspace_root("service-health-response-webapi-extension");
    let service_root = workspace_root.join("src/Orders");
    let web_api_root = service_root.join("presentation/Orders.WebApi");
    std::fs::create_dir_all(web_api_root.join("Shared/HealthCheck/Extensions"))
        .expect("web api extension directory should be created");
    std::fs::write(
        web_api_root.join("Program.cs"),
        "var app = builder.Build();\napp.MapHealthCheckEndpoints();\napp.Run();\n",
    )
    .expect("program should be written");
    std::fs::write(
        web_api_root.join("Shared/HealthCheck/Extensions/HealthCheckExtensions.cs"),
        "private const string HEALTH_GROUP_ENDPOINT = \"/health\";\npublic static void MapHealthCheckEndpoints(this IEndpointRouteBuilder endpoints)\n{\n\t_ = endpoints.MapHealthChecks($\"{HEALTH_GROUP_ENDPOINT}/live\");\n\t_ = endpoints.MapHealthChecks($\"{HEALTH_GROUP_ENDPOINT}/ready\");\n}\n",
    )
    .expect("health extension should be written");

    let inspector = FileSystemGeneratedApiContractInspector::new();
    inspector
        .assert_health_endpoints(&service_root)
        .expect("health contract inspection should pass");

    support::cleanup_sandbox_directory(&workspace_root);
}
