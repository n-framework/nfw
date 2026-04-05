use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use nframework_nfw_core_application::features::cli::configuration::abstractions::nfw_configuration_loader::NfwConfigurationLoader;
use nframework_nfw_core_application::features::cli::configuration::abstractions::path_resolver::PathResolver;
use nframework_nfw_core_domain::features::template_management::template_source::TemplateSource;
use nframework_nfw_infrastructure_filesystem::features::cli::configuration::nfw_configuration_loader::NfwFileSystemConfigurationLoader;

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

#[test]
fn loads_empty_sources_when_file_is_missing() {
    let sandbox = create_sandbox_directory();
    let cache_directory = sandbox.join("cache");
    let config_directory = sandbox.join("config");
    fs::create_dir_all(&cache_directory).expect("cache directory should be created");
    fs::create_dir_all(&config_directory).expect("config directory should be created");

    let loader = NfwFileSystemConfigurationLoader::new(TestPathResolver {
        cache_directory: cache_directory.clone(),
        config_directory: config_directory.clone(),
    });

    let configuration = loader
        .load_configuration()
        .expect("load should succeed when sources file is missing");

    assert!(configuration.template_sources.is_empty());
    assert_eq!(configuration.cache_directory, cache_directory);
    assert_eq!(configuration.config_directory, config_directory);
}

#[test]
fn saves_and_loads_sources_file() {
    let sandbox = create_sandbox_directory();
    let cache_directory = sandbox.join("cache");
    let config_directory = sandbox.join("config");
    fs::create_dir_all(&cache_directory).expect("cache directory should be created");

    let loader = NfwFileSystemConfigurationLoader::new(TestPathResolver {
        cache_directory: cache_directory.clone(),
        config_directory: config_directory.clone(),
    });

    let input_configuration = nframework_nfw_core_application::features::cli::configuration::nfw_configuration::NfwConfiguration::new(
        vec![
            TemplateSource::new(
                "official".to_owned(),
                "https://github.com/n-framework/nfw-templates".to_owned(),
            ),
            TemplateSource::new_disabled(
                "my-team".to_owned(),
                "https://example.com/my-team.git".to_owned(),
            ),
        ],
        cache_directory.clone(),
        config_directory.clone(),
    );

    loader
        .save_configuration(&input_configuration)
        .expect("save should succeed");

    let sources_file_path = config_directory.join("sources.yaml");
    assert!(sources_file_path.is_file());
    let content = fs::read_to_string(&sources_file_path).expect("sources file should be readable");
    assert!(content.contains("name: official"));
    assert!(content.contains("name: my-team"));

    let loaded_configuration = loader
        .load_configuration()
        .expect("load should succeed after save");
    assert_eq!(
        loaded_configuration.template_sources,
        input_configuration.template_sources
    );
}

fn create_sandbox_directory() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("nfw-config-loader-tests-{timestamp}"));
    fs::create_dir_all(&path).expect("sandbox directory should be created");
    path
}
