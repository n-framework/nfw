#[path = "../../../service_add/support.rs"]
mod support;

#[path = "support.rs"]
mod gen_support;

use std::fs;
use std::path::Path;
use std::sync::Mutex;
use std::time::Instant;

static DIR_LOCK: Mutex<()> = Mutex::new(());

fn setup_repository_workspace(
    sandbox: &Path,
    with_persistence: bool,
    with_entity: bool,
    feature: &str,
) {
    let modules_str = if with_persistence {
        r#"
    modules:
      - persistence"#
    } else {
        ""
    };

    fs::write(
        sandbox.join("nfw.yaml"),
        format!(
            r#"
workspace:
  name: Test
  namespace: TestApp
services:
  TestService:
    path: src/TestService
    template:
      id: dotnet-service{}
template_sources:
  local: "templates"
"#,
            modules_str
        ),
    )
    .expect("failed to write nfw.yaml");

    // Scaffold the template configuration
    let root_tpl_dir = sandbox.join("templates").join("dotnet-service");
    fs::create_dir_all(&root_tpl_dir).expect("failed to create root template dir");
    fs::write(
        root_tpl_dir.join("template.yaml"),
        "id: dotnet-service\nname: Dotnet Service\nversion: 1.0.0\ngenerators:\n  repository: ./repository/\n",
    )
    .expect("failed to write root template.yaml");

    let tpl_dir = root_tpl_dir.join("repository");
    fs::create_dir_all(tpl_dir.join("content/interface"))
        .expect("failed to create sub-template dir");
    fs::create_dir_all(tpl_dir.join("content/implementation"))
        .expect("failed to create sub-template dir");
    fs::create_dir_all(tpl_dir.join("content/di-registration"))
        .expect("failed to create sub-template dir");

    fs::write(
        tpl_dir.join("template.yaml"),
        r#"
id: dotnet-service/repository
steps:
  - action: render
    source: "content/interface/IEntityRepository.cs.tera"
    destination: "src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Repositories/I{{ Entity }}Repository.cs"
  - action: render
    source: "content/implementation/EntityRepository.cs.tera"
    destination: "src/infrastructure/{{ Service }}.Infrastructure.Persistence/Features/{{ Feature }}/Repositories/{{ Entity }}Repository.cs"
  - action: inject
    source: "content/di-registration/registration.tera"
    destination: "src/infrastructure/{{ Service }}.Infrastructure.Persistence/ServiceRegistration.cs"
    injection_target:
      type: region
      value: repository-registrations
"#,
    ).expect("failed to write sub-template template.yaml");

    fs::write(
        tpl_dir.join("content/interface/IEntityRepository.cs.tera"),
        "public interface I{{ Entity }}Repository : IReadRepository<{{ Entity }}>, IWriteRepository<{{ Entity }}>",
    ).unwrap();

    fs::write(
        tpl_dir.join("content/implementation/EntityRepository.cs.tera"),
        "public class {{ Entity }}Repository : EFCoreRepository<{{ Entity }}>, I{{ Entity }}Repository",
    ).unwrap();

    fs::write(
        tpl_dir.join("content/di-registration/registration.tera"),
        "services.AddScoped<I{{ Entity }}Repository, {{ Entity }}Repository>();",
    )
    .unwrap();

    // Create the entity if needed
    if with_entity {
        let entities_dir = sandbox.join(format!(
            "src/TestService/src/core/TestService.Core.Domain/Features/{}/Entities",
            feature
        ));
        fs::create_dir_all(&entities_dir).expect("failed to create entities dir");
        fs::write(entities_dir.join("User.cs"), "public class User {}")
            .expect("failed to write entity file");
    }

    // Create ServiceRegistration.cs for injection target
    let di_dir =
        sandbox.join("src/TestService/src/infrastructure/TestService.Infrastructure.Persistence");
    fs::create_dir_all(&di_dir).unwrap();
    fs::write(
        di_dir.join("ServiceRegistration.cs"),
        r#"
public static class ServiceRegistration {
    public static void AddPersistence(this IServiceCollection services) {
        // <nfw:repository-registrations:start>
        // <nfw:repository-registrations:end>
    }
}
"#,
    )
    .unwrap();
}

#[test]
fn generates_repository_successfully_when_valid() {
    let sandbox = support::create_sandbox_directory("gen-repository-integration");
    setup_repository_workspace(&sandbox, true, true, "identity");

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let start = Instant::now();

    // With auto-detection (feature arg is empty string, which command_handler handles or we pass feature explicitly)
    let result = gen_support::execute_non_interactive_gen_repository(&sandbox, "User", "");

    let duration = start.elapsed();
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "gen repository failed: {:?}", result.err());

    // Performance assertion (T028, T030)
    assert!(
        duration.as_secs_f64() < 2.0,
        "Command execution took too long: {} seconds (must be < 2s)",
        duration.as_secs_f64()
    );

    // Check files
    let service_dir = sandbox.join("src/TestService");

    // Interface
    let interface_path = service_dir.join(
        "src/core/TestService.Core.Application/Features/identity/Repositories/IUserRepository.cs",
    );
    assert!(
        interface_path.exists(),
        "Interface IUserRepository.cs was not generated"
    );
    let interface_content = fs::read_to_string(interface_path).unwrap();
    assert!(interface_content.contains(
        "public interface IUserRepository : IReadRepository<User>, IWriteRepository<User>"
    ));

    // Implementation
    let impl_path = service_dir.join("src/infrastructure/TestService.Infrastructure.Persistence/Features/identity/Repositories/UserRepository.cs");
    assert!(
        impl_path.exists(),
        "Implementation UserRepository.cs was not generated"
    );
    let impl_content = fs::read_to_string(impl_path).unwrap();
    assert!(
        impl_content
            .contains("public class UserRepository : EFCoreRepository<User>, IUserRepository")
    );

    // DI Registration
    let di_path = service_dir
        .join("src/infrastructure/TestService.Infrastructure.Persistence/ServiceRegistration.cs");
    assert!(di_path.exists(), "DI Registration file was not generated");
    let di_content = fs::read_to_string(di_path).unwrap();
    assert!(di_content.contains("services.AddScoped<IUserRepository, UserRepository>();"));

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn generates_repository_successfully_with_feature_flag() {
    let sandbox = support::create_sandbox_directory("gen-repository-feature-flag");
    setup_repository_workspace(&sandbox, true, true, "payments");

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let start = Instant::now();
    let result = gen_support::execute_non_interactive_gen_repository(&sandbox, "User", "payments");
    let duration = start.elapsed();

    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "gen repository failed: {:?}", result.err());

    // Performance assertion (T028, T030)
    assert!(
        duration.as_secs_f64() < 2.0,
        "Command execution took too long: {} seconds (must be < 2s)",
        duration.as_secs_f64()
    );

    let service_dir = sandbox.join("src/TestService");

    // Interface should be in payments feature folder
    let interface_path = service_dir.join(
        "src/core/TestService.Core.Application/Features/payments/Repositories/IUserRepository.cs",
    );
    assert!(
        interface_path.exists(),
        "Interface IUserRepository.cs was not generated in payments folder"
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn gen_repository_fails_if_entity_not_found() {
    let sandbox = support::create_sandbox_directory("gen-repository-no-entity");
    setup_repository_workspace(&sandbox, true, false, "identity"); // No entity

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let start = Instant::now();
    let result = gen_support::execute_non_interactive_gen_repository(&sandbox, "User", "identity");
    let duration = start.elapsed();

    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err());

    // Performance assertion (T029, T030)
    assert!(
        duration.as_secs_f64() < 1.0,
        "Error validation took too long: {} seconds (must be < 1s)",
        duration.as_secs_f64()
    );

    let err_str = format!("{:?}", result.err().unwrap());
    assert!(
        err_str.contains("not found in feature")
            || err_str.contains("does not contain an Entities folder")
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn gen_repository_fails_if_invalid_feature_folder() {
    let sandbox = support::create_sandbox_directory("gen-repository-invalid-feature");
    setup_repository_workspace(&sandbox, true, true, "identity");

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let start = Instant::now();
    let result = gen_support::execute_non_interactive_gen_repository(
        &sandbox,
        "User",
        "non_existent_feature",
    );
    let duration = start.elapsed();

    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err());

    // Performance assertion (T029, T030)
    assert!(
        duration.as_secs_f64() < 1.0,
        "Error validation took too long: {} seconds (must be < 1s)",
        duration.as_secs_f64()
    );

    let err_str = format!("{:?}", result.err().unwrap());
    assert!(err_str.contains("does not contain an Entities folder"));

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn gen_repository_fails_if_persistence_not_configured() {
    let sandbox = support::create_sandbox_directory("gen-repository-no-persistence");
    setup_repository_workspace(&sandbox, false, true, "identity"); // No persistence

    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&sandbox).unwrap();

    let start = Instant::now();
    let result = gen_support::execute_non_interactive_gen_repository(&sandbox, "User", "identity");
    let duration = start.elapsed();

    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_err());

    // Performance assertion (T029, T030)
    assert!(
        duration.as_secs_f64() < 1.0,
        "Error validation took too long: {} seconds (must be < 1s)",
        duration.as_secs_f64()
    );

    let err_str = format!("{:?}", result.err().unwrap());
    assert!(err_str.contains("does not have 'persistence' module configured"));

    support::cleanup_sandbox_directory(&sandbox);
}
