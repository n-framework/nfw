use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use n_framework_nfw_core_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use n_framework_nfw_core_application::features::cli::configuration::abstractions::path_resolver::PathResolver;
use n_framework_nfw_core_application::features::generator_management::services::abstractions::validator::Validator;
use n_framework_nfw_core_application::features::generator_management::services::generator_catalog_parser::GeneratorCatalogParser;
use n_framework_nfw_core_application::features::generator_management::services::generator_catalog_source_resolver::GeneratorCatalogSourceResolver;
use n_framework_nfw_core_application::features::generator_management::services::generators_service::GeneratorsService;
use n_framework_nfw_core_domain::features::generator_management::generator_source::GeneratorSource;
use n_framework_nfw_infrastructure_filesystem::features::generator_management::services::local_generators_catalog_source::LocalGeneratorsCatalogSource;
use n_framework_nfw_infrastructure_git::features::generator_management::services::cli_git_repository::CliGitRepository;
use n_framework_nfw_infrastructure_git::features::generator_management::services::git_generator_catalog_source::GitGeneratorCatalogSource;
use n_framework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use n_framework_nfw_infrastructure_yaml::features::generator_management::services::serde_yaml_parser::SerdeYamlParser;

#[derive(Debug, Clone)]
struct TestConfigStore {
    sources: Vec<GeneratorSource>,
}

impl ConfigStore for TestConfigStore {
    fn load_sources(&self) -> Result<Vec<GeneratorSource>, String> {
        Ok(self.sources.clone())
    }

    fn save_sources(&self, _sources: &[GeneratorSource]) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct TestPathResolver {
    cache_directory: PathBuf,
    config_directory: PathBuf,
}

impl PathResolver for TestPathResolver {
    fn cache_dir(&self) -> Result<PathBuf, String> {
        Ok(self.cache_directory.clone())
    }

    fn config_dir(&self) -> Result<PathBuf, String> {
        Ok(self.config_directory.clone())
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct TestValidator;

impl Validator for TestValidator {
    fn is_kebab_case(&self, value: &str) -> bool {
        if value.starts_with('-') || value.ends_with('-') || value.contains("--") {
            return false;
        }

        value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    }

    fn is_git_url(&self, value: &str) -> bool {
        value.starts_with("http://")
            || value.starts_with("https://")
            || value.starts_with("git@")
            || Path::new(value).exists()
    }
}

#[test]
fn discovers_and_refreshes_generators_from_git_source() {
    let sandbox = create_sandbox_directory();
    let remote_repository = sandbox.join("remote-catalog.git");
    let seed_repository = sandbox.join("seed-repository");
    let update_repository = sandbox.join("update-repository");
    let cache_directory = sandbox.join("cache");
    let config_directory = sandbox.join("config");

    run_git_command(&sandbox, &["init", "--bare", path_text(&remote_repository)]);

    write_catalog_repository(
        &seed_repository,
        &[("web-api", "Web API"), ("worker-service", "Worker Service")],
    );
    run_git_command(
        &seed_repository,
        &["remote", "add", "origin", path_text(&remote_repository)],
    );
    run_git_command(&seed_repository, &["branch", "-M", "main"]);
    run_git_command(&seed_repository, &["push", "-u", "origin", "main"]);

    let service = create_generators_service(
        vec![GeneratorSource::new(
            "official".to_owned(),
            remote_repository.to_string_lossy().to_string(),
        )],
        cache_directory,
        config_directory,
    );

    let (first_discovery, first_warnings) =
        service.list_generators().expect("discovery should succeed");
    assert!(first_warnings.is_empty());
    assert_eq!(first_discovery.len(), 2);
    assert!(
        first_discovery
            .iter()
            .any(|generator| generator.id == "web-api")
    );
    assert!(
        first_discovery
            .iter()
            .any(|generator| generator.id == "worker-service")
    );

    run_git_command(
        &sandbox,
        &[
            "clone",
            path_text(&remote_repository),
            path_text(&update_repository),
        ],
    );
    write_generator(&update_repository, "grpc-service", "Grpc Service");
    run_git_command(&update_repository, &["add", "."]);
    run_git_command(
        &update_repository,
        &["commit", "-m", "add grpc-service generator"],
    );
    run_git_command(&update_repository, &["push", "origin", "main"]);

    let (second_discovery, second_warnings) = service
        .list_generators()
        .expect("refresh discovery should succeed");

    assert!(second_warnings.is_empty());
    assert_eq!(second_discovery.len(), 3);
    assert!(
        second_discovery
            .iter()
            .any(|generator| generator.id == "grpc-service")
    );
}

fn create_generators_service(
    sources: Vec<GeneratorSource>,
    cache_directory: PathBuf,
    config_directory: PathBuf,
) -> GeneratorsService<
    GitGeneratorCatalogSource<CliGitRepository, TestPathResolver>,
    LocalGeneratorsCatalogSource,
    SerdeYamlParser,
    TestValidator,
    SemverVersionComparator,
    TestConfigStore,
    CliGitRepository,
> {
    fs::create_dir_all(&cache_directory).expect("cache directory should be created");
    fs::create_dir_all(&config_directory).expect("config directory should be created");

    let git_repository = CliGitRepository::new();
    let path_resolver = TestPathResolver {
        cache_directory,
        config_directory,
    };
    let source_synchronizer = GitGeneratorCatalogSource::new(git_repository, path_resolver);
    let catalog_source = LocalGeneratorsCatalogSource::new();
    let catalog_parser = GeneratorCatalogParser::new(
        SerdeYamlParser::new(),
        TestValidator,
        SemverVersionComparator::new(),
    );
    let catalog_resolver = GeneratorCatalogSourceResolver::new(catalog_source, catalog_parser);
    let config_store = TestConfigStore { sources };

    GeneratorsService::new(
        source_synchronizer,
        catalog_resolver,
        config_store,
        TestValidator,
        git_repository,
    )
}

fn write_catalog_repository(root: &Path, generators: &[(&str, &str)]) {
    if root.exists() {
        fs::remove_dir_all(root).expect("existing repository should be removed");
    }
    fs::create_dir_all(root).expect("repository root should be created");

    run_git_command(root, &["init"]);
    run_git_command(root, &["config", "user.name", "nfw-test"]);
    run_git_command(root, &["config", "user.email", "nfw-test@example.com"]);

    for (generator_id, generator_name) in generators {
        write_generator(root, generator_id, generator_name);
    }

    run_git_command(root, &["add", "."]);
    run_git_command(root, &["commit", "-m", "seed generators"]);
}

fn write_generator(root: &Path, generator_id: &str, generator_name: &str) {
    let generator_directory = root.join("src").join(generator_id);
    let content_directory = generator_directory.join("content");
    fs::create_dir_all(&content_directory).expect("generator content directory should be created");

    fs::write(
        generator_directory.join("nfw.generator.yaml"),
        format!(
            "id: {generator_id}\nname: {generator_name}\ndescription: {generator_name} generator\nversion: 1.0.0\nlanguage: rust\n"
        ),
    )
    .expect("generator metadata should be written");
    fs::write(content_directory.join("README.md"), "# Generator\n")
        .expect("generator content should be written");
}

fn create_sandbox_directory() -> PathBuf {
    let unix_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("nfw-phase5-it-{unix_timestamp}"));
    fs::create_dir_all(&path).expect("sandbox directory should be created");
    path
}

fn run_git_command(working_directory: &Path, arguments: &[&str]) {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(working_directory)
        .output()
        .expect("git command should execute");

    if !output.status.success() {
        panic!(
            "git command failed in '{}': git {} \nstdout: {}\nstderr: {}",
            working_directory.display(),
            arguments.join(" "),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn path_text(path: &Path) -> &str {
    path.to_str().expect("path should be valid UTF-8 for tests")
}
