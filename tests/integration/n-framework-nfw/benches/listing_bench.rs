use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use n_framework_nfw_core_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use n_framework_nfw_core_application::features::template_management::services::abstractions::git_repository::GitRepository;
use n_framework_nfw_core_application::features::template_management::services::abstractions::template_source_synchronizer::TemplateSourceSynchronizer;
use n_framework_nfw_core_application::features::template_management::services::abstractions::validator::Validator;
use n_framework_nfw_core_application::features::template_management::services::template_catalog_parser::TemplateCatalogParser;
use n_framework_nfw_core_application::features::template_management::services::template_catalog_source_resolver::TemplateCatalogSourceResolver;
use n_framework_nfw_core_application::features::template_management::services::templates_service::TemplatesService;
use n_framework_nfw_core_domain::features::template_management::template_source::TemplateSource;
use n_framework_nfw_infrastructure_filesystem::features::template_management::services::local_templates_catalog_source::LocalTemplatesCatalogSource;
use n_framework_nfw_infrastructure_versioning::features::versioning::services::semver_version_comparator::SemverVersionComparator;
use n_framework_nfw_infrastructure_yaml::features::template_management::services::serde_yaml_parser::SerdeYamlParser;

#[derive(Debug, Clone)]
struct StaticConfigStore {
    sources: Vec<TemplateSource>,
}

impl ConfigStore for StaticConfigStore {
    fn load_sources(&self) -> Result<Vec<TemplateSource>, String> {
        Ok(self.sources.clone())
    }

    fn save_sources(&self, _sources: &[TemplateSource]) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct LocalSourceSynchronizer {
    source_root: PathBuf,
}

impl TemplateSourceSynchronizer for LocalSourceSynchronizer {
    fn sync_source(&self, _source: &TemplateSource) -> Result<(PathBuf, Option<String>), String> {
        Ok((self.source_root.clone(), None))
    }

    fn clear_source_cache(&self, _source_name: &str) -> Result<(), String> {
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct BenchmarkValidator;

impl Validator for BenchmarkValidator {
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

#[derive(Debug, Default, Clone, Copy)]
struct NoopGitRepository;

impl GitRepository for NoopGitRepository {
    fn clone(&self, _url: &str, _destination: &Path) -> Result<(), String> {
        Ok(())
    }

    fn fetch(&self, _repository_path: &Path) -> Result<(), String> {
        Ok(())
    }

    fn pull(&self, _repository_path: &Path) -> Result<(), String> {
        Ok(())
    }

    fn current_branch(&self, _repository_path: &Path) -> Result<String, String> {
        Ok("main".to_owned())
    }

    fn is_valid_repo(&self, _repository_path: &Path) -> Result<bool, String> {
        Ok(true)
    }

    fn is_valid_git_url_format(&self, url: &str) -> bool {
        // Simple format validation for tests
        url.starts_with("http://")
            || url.starts_with("https://")
            || url.starts_with("git@")
            || url.starts_with('/')
    }

    fn is_remote_url_reachable(&self, _url: &str) -> bool {
        true
    }
}

#[test]
fn listing_fifty_templates_stays_under_target_threshold() {
    let source_root = create_sandbox_directory();
    create_templates(&source_root, 50);

    let source_name = "benchmark-source";
    let source_url = "https://example.com/benchmark-source.git";

    let service = create_templates_service(source_root.clone(), source_name, source_url);

    let started_at = Instant::now();
    let (templates, warnings) = service.list_templates().expect("list should succeed");
    let elapsed = started_at.elapsed();

    assert!(warnings.is_empty());
    assert_eq!(templates.len(), 50);
    assert!(
        elapsed < Duration::from_millis(500),
        "listing 50 templates took {:?}, expected < 500ms",
        elapsed
    );
}

fn create_templates_service(
    source_root: PathBuf,
    source_name: &str,
    source_url: &str,
) -> TemplatesService<
    LocalSourceSynchronizer,
    LocalTemplatesCatalogSource,
    SerdeYamlParser,
    BenchmarkValidator,
    SemverVersionComparator,
    StaticConfigStore,
    NoopGitRepository,
> {
    let source_synchronizer = LocalSourceSynchronizer { source_root };
    let catalog_source = LocalTemplatesCatalogSource::new();
    let catalog_parser = TemplateCatalogParser::new(
        SerdeYamlParser::new(),
        BenchmarkValidator,
        SemverVersionComparator::new(),
    );
    let catalog_resolver = TemplateCatalogSourceResolver::new(catalog_source, catalog_parser);
    let config_store = StaticConfigStore {
        sources: vec![TemplateSource::new(
            source_name.to_owned(),
            source_url.to_owned(),
        )],
    };

    TemplatesService::new(
        source_synchronizer,
        catalog_resolver,
        config_store,
        BenchmarkValidator,
        NoopGitRepository,
    )
}

fn create_templates(root: &Path, count: usize) {
    for index in 0..count {
        let template_id = format!("template-{index}");
        let template_directory = root.join(&template_id);
        let content_directory = template_directory.join("content");
        fs::create_dir_all(&content_directory).expect("content directory should be created");

        fs::write(
            template_directory.join("template.yaml"),
            format!(
                "id: {template_id}\nname: Template {index}\ndescription: Template {index}\nversion: 1.0.0\nlanguage: rust\n"
            ),
        )
        .expect("template metadata should be written");
        fs::write(content_directory.join("README.md"), "# Template\n")
            .expect("template content should be written");
    }
}

fn create_sandbox_directory() -> PathBuf {
    let unix_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("nfw-phase8-listing-bench-{unix_timestamp}"));
    fs::create_dir_all(&path).expect("sandbox directory should be created");
    path
}
