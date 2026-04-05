use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use nframework_nfw_core_application::features::cli::configuration::abstractions::path_resolver::PathResolver;
use nframework_nfw_core_application::features::template_management::services::abstractions::validator::Validator;
use nframework_nfw_core_application::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use nframework_nfw_core_application::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use nframework_nfw_core_application::features::template_management::services::templates_service::TemplatesService;
use nframework_nfw_core_application::features::versioning::version_provider::VersionProvider;
use nframework_nfw_core_application::features::versioning::version_resolver::VersionResolver;
use nframework_nfw_core_domain::features::versioning::version_info::VersionInfo;
use nframework_nfw_infrastructure_filesystem::features::cli::configuration::nfw_configuration_loader::NfwFileSystemConfigurationLoader;
use nframework_nfw_infrastructure_filesystem::features::template_management::services::file_system_config_store::FileSystemWorkspaceArtifactWriter;
use nframework_nfw_infrastructure_filesystem::features::template_management::services::local_templates_catalog_source::LocalTemplatesCatalogSource;
use nframework_nfw_infrastructure_filesystem::features::template_management::services::placeholder_detector::PlaceholderDetector;
use nframework_nfw_infrastructure_git::features::template_management::services::cli_git_repository::CliGitRepository;
use nframework_nfw_infrastructure_git::features::template_management::services::git_template_catalog_source::GitTemplateCatalogSource;
use nframework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use nframework_nfw_infrastructure_yaml::features::template_management::services::serde_yaml_parser::SerdeYamlParser;

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
fn runs_template_source_registration_discovery_listing_and_version_resolution() {
    let sandbox = create_sandbox_directory();
    let remote_repository = sandbox.join("remote-catalog.git");
    let seed_repository = sandbox.join("seed-repository");
    let cache_directory = sandbox.join("cache");
    let config_directory = sandbox.join("config");

    run_git_command(&sandbox, &["init", "--bare", path_text(&remote_repository)]);
    write_catalog_repository(
        &seed_repository,
        &[
            ("web-api-v1", "web-api", "1.0.0", "Web API"),
            ("web-api-v2", "web-api", "1.1.0", "Web API"),
        ],
    );
    run_git_command(
        &seed_repository,
        &["remote", "add", "origin", path_text(&remote_repository)],
    );
    run_git_command(&seed_repository, &["branch", "-M", "main"]);
    run_git_command(&seed_repository, &["push", "-u", "origin", "main"]);

    let service = create_templates_service(cache_directory, config_directory);
    service
        .add_source("local-catalog", path_text(&remote_repository))
        .expect("source registration should succeed");

    let (templates, warnings) = service.list_templates().expect("listing should succeed");
    assert!(warnings.is_empty());

    let mut versions = templates
        .iter()
        .filter(|template| template.id == "web-api")
        .map(|template| VersionInfo::new(template.version.clone()))
        .collect::<Vec<_>>();

    versions.sort_by(|left, right| left.version.to_string().cmp(&right.version.to_string()));
    assert_eq!(versions.len(), 2);

    let resolver = VersionResolver::new(VersionProvider::new(SemverVersionComparator::new()));
    let latest = resolver
        .resolve_latest_stable(&versions)
        .expect("version resolution should succeed")
        .expect("latest stable version should be resolved");

    assert_eq!(latest.version.to_string(), "1.1.0");
}

type E2eTemplatesService = TemplatesService<
    GitTemplateCatalogSource<CliGitRepository, TestPathResolver>,
    LocalTemplatesCatalogSource,
    SerdeYamlParser,
    TestValidator,
    SemverVersionComparator,
    FileSystemWorkspaceArtifactWriter<NfwFileSystemConfigurationLoader<TestPathResolver>>,
    CliGitRepository,
>;

fn create_templates_service(
    cache_directory: PathBuf,
    config_directory: PathBuf,
) -> E2eTemplatesService {
    fs::create_dir_all(&cache_directory).expect("cache directory should be created");
    fs::create_dir_all(&config_directory).expect("config directory should be created");

    let path_resolver = TestPathResolver {
        cache_directory,
        config_directory,
    };
    let git_repository = CliGitRepository::new();
    let source_synchronizer = GitTemplateCatalogSource::new(git_repository, path_resolver.clone());

    let catalog_source = LocalTemplatesCatalogSource::new(PlaceholderDetector::new());
    let catalog_parser = TemplateCatalogParser::new(
        SerdeYamlParser::new(),
        TestValidator,
        SemverVersionComparator::new(),
    );
    let catalog_resolver = TemplateCatalogSourceResolver::new(catalog_source, catalog_parser);

    let config_loader = NfwFileSystemConfigurationLoader::new(path_resolver);
    let config_store = FileSystemWorkspaceArtifactWriter::new(config_loader);

    TemplatesService::new(
        source_synchronizer,
        catalog_resolver,
        config_store,
        TestValidator,
        git_repository,
    )
}

fn write_catalog_repository(root: &Path, templates: &[(&str, &str, &str, &str)]) {
    if root.exists() {
        fs::remove_dir_all(root).expect("existing repository should be removed");
    }
    fs::create_dir_all(root).expect("repository root should be created");

    run_git_command(root, &["init"]);
    run_git_command(root, &["config", "user.name", "nfw-test"]);
    run_git_command(root, &["config", "user.email", "nfw-test@example.com"]);

    for (directory_name, template_id, version, template_name) in templates {
        write_template(root, directory_name, template_id, version, template_name);
    }

    run_git_command(root, &["add", "."]);
    run_git_command(root, &["commit", "-m", "seed templates"]);
}

fn write_template(
    root: &Path,
    directory_name: &str,
    template_id: &str,
    version: &str,
    template_name: &str,
) {
    let template_directory = root.join(directory_name);
    let content_directory = template_directory.join("content");
    fs::create_dir_all(&content_directory).expect("template content directory should be created");

    fs::write(
        template_directory.join("template.yaml"),
        format!(
            "id: {template_id}\nname: {template_name}\ndescription: {template_name} template\nversion: {version}\nlanguage: rust\n"
        ),
    )
    .expect("template metadata should be written");
    fs::write(content_directory.join("README.md"), "# Template\n")
        .expect("template content should be written");
}

fn create_sandbox_directory() -> PathBuf {
    let unix_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("nfw-phase8-e2e-{unix_timestamp}"));
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
            "git command failed in '{}': git {}\nstdout: {}\nstderr: {}",
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
