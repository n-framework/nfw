use nframework_nfw_core_application::features::check::models::CheckLayer;

#[test]
fn architecture_layer_from_path_handles_domain_layer() {
    assert_eq!(
        CheckLayer::from_path("src/domain/services.rs"),
        CheckLayer::Domain
    );
    assert_eq!(
        CheckLayer::from_path("src/infrastructure-nfw/domain/entities.rs"),
        CheckLayer::Domain
    );
    assert_eq!(
        CheckLayer::from_path("core/domain/value_objects.rs"),
        CheckLayer::Domain
    );
}

#[test]
fn architecture_layer_from_path_handles_application_layer() {
    assert_eq!(
        CheckLayer::from_path("src/application/commands.rs"),
        CheckLayer::Application
    );
    assert_eq!(
        CheckLayer::from_path("core/application/handlers.rs"),
        CheckLayer::Application
    );
}

#[test]
fn architecture_layer_from_path_handles_infrastructure_layer() {
    assert_eq!(
        CheckLayer::from_path("src/infrastructure/data.rs"),
        CheckLayer::Infrastructure
    );
    assert_eq!(
        CheckLayer::from_path("core/infrastructure/repositories.rs"),
        CheckLayer::Infrastructure
    );
}

#[test]
fn architecture_layer_from_path_handles_presentation_layer() {
    assert_eq!(
        CheckLayer::from_path("src/presentation/controllers.rs"),
        CheckLayer::Presentation
    );
    assert_eq!(
        CheckLayer::from_path("core/presentation/views.rs"),
        CheckLayer::Presentation
    );
}

#[test]
fn architecture_layer_from_path_handles_unknown_layer() {
    assert_eq!(
        CheckLayer::from_path("src/my-domain-specific-logic.rs"),
        CheckLayer::Unknown
    );
    assert_eq!(
        CheckLayer::from_path("src/applicationlogic/handlers.rs"),
        CheckLayer::Unknown
    );
    assert_eq!(
        CheckLayer::from_path("src/shared/utils.rs"),
        CheckLayer::Unknown
    );
    assert_eq!(CheckLayer::from_path("src/lib.rs"), CheckLayer::Unknown);
}

#[test]
fn architecture_layer_from_path_case_insensitive() {
    assert_eq!(
        CheckLayer::from_path("src/Domain/services.rs"),
        CheckLayer::Domain
    );
    assert_eq!(
        CheckLayer::from_path("src/APPLICATION/commands.rs"),
        CheckLayer::Application
    );
    assert_eq!(
        CheckLayer::from_path("src/INFRASTRUCTURE/data.rs"),
        CheckLayer::Infrastructure
    );
}

#[test]
fn architecture_layer_from_path_with_special_prefixes() {
    // These should be recognized as layer components
    assert_eq!(
        CheckLayer::from_path("src/.domain/services.rs"),
        CheckLayer::Domain
    );
    assert_eq!(
        CheckLayer::from_path("src/_application/commands.rs"),
        CheckLayer::Application
    );
    assert_eq!(
        CheckLayer::from_path("src/-infrastructure/data.rs"),
        CheckLayer::Infrastructure
    );
}

#[test]
fn architecture_layer_from_path_substring_does_not_match() {
    // "mydomainlogic" should NOT match as Domain layer
    assert_eq!(
        CheckLayer::from_path("src/mydomainlogic/services.rs"),
        CheckLayer::Unknown
    );
    // "applicationlayer" should NOT match as Application layer
    assert_eq!(
        CheckLayer::from_path("src/applicationlayer/handlers.rs"),
        CheckLayer::Unknown
    );
}

#[test]
fn architecture_layer_from_path_windows_paths() {
    assert_eq!(
        CheckLayer::from_path("core\\domain\\services.rs"),
        CheckLayer::Domain
    );
    assert_eq!(
        CheckLayer::from_path("core\\application\\commands.rs"),
        CheckLayer::Application
    );
}

#[test]
fn architecture_layer_from_path_core_compound_names() {
    assert_eq!(
        CheckLayer::from_path("src/nframework-nfw-core-domain/Cargo.toml"),
        CheckLayer::Domain
    );
    assert_eq!(
        CheckLayer::from_path("src/nframework-nfw-core-application/Cargo.toml"),
        CheckLayer::Application
    );
    assert_eq!(
        CheckLayer::from_path("src/MyProject.Core.Domain/MyProject.Core.Domain.csproj"),
        CheckLayer::Domain
    );
    assert_eq!(
        CheckLayer::from_path("src/MyProject.Core.Application/MyProject.Core.Application.csproj"),
        CheckLayer::Application
    );
    assert_eq!(
        CheckLayer::from_path(
            "src/MyProject.Core.Infrastructure/MyProject.Core.Infrastructure.csproj"
        ),
        CheckLayer::Infrastructure
    );
    assert_eq!(
        CheckLayer::from_path("src/MyProject.Core.Presentation/MyProject.Core.Presentation.csproj"),
        CheckLayer::Presentation
    );
}

#[test]
fn architecture_layer_from_path_presentation_webapi_compound_names() {
    assert_eq!(
        CheckLayer::from_path(
            "src/__ServiceName__.Presentation.WebApi/__ServiceName__.Presentation.WebApi.csproj"
        ),
        CheckLayer::Presentation
    );
    assert_eq!(
        CheckLayer::from_path("src/MyProject.Presentation.WebApi/Program.cs"),
        CheckLayer::Presentation
    );
    assert_eq!(
        CheckLayer::from_path("src/MyProject.Presentation.WebApi/Shared/OpenApi/Extensions/OpenApiConfigurationExtensions.cs"),
        CheckLayer::Presentation
    );
    assert_eq!(
        CheckLayer::from_path("presentation/__ServiceName__.Presentation.WebApi/__ServiceName__.Presentation.WebApi.csproj"),
        CheckLayer::Presentation
    );
}
