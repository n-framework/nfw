#[path = "support.rs"]
mod support;

use std::fs;

#[test]
fn generated_service_contains_build_ready_project_files() {
    let workspace_root = support::create_workspace_root("service-first-build");
    let template_root =
        support::create_service_template(&workspace_root, "dotnet-service-template", "service");
    let template_resolution =
        support::create_template_resolution(&template_root, "official", "dotnet-service");
    let orchestration = support::build_default_orchestration(&workspace_root, template_resolution);

    support::execute_non_interactive_add_service(
        &orchestration,
        "Orders",
        "official/dotnet-service",
    )
    .expect("service generation should succeed");

    let service_root = workspace_root.join("src/Orders");
    let project_files = [
        service_root.join("Domain/Orders.Domain.csproj"),
        service_root.join("Application/Orders.Application.csproj"),
        service_root.join("Infrastructure/Orders.Infrastructure.csproj"),
        service_root.join("Api/Orders.WebApi.csproj"),
    ];

    for project_file in project_files {
        let content =
            fs::read_to_string(&project_file).expect("generated project file should exist");
        assert!(
            content.contains("<Project"),
            "project file '{}' must contain an XML project root",
            project_file.display()
        );
    }

    support::cleanup_sandbox_directory(&workspace_root);
}
