use n_framework_nfw_core_application::features::cli::configuration::abstractions::config_store::ConfigStore;
use n_framework_nfw_core_application::features::cli::configuration::abstractions::nfw_configuration_loader::NfwConfigurationLoader;
use n_framework_nfw_core_domain::features::generator_management::generator_source::GeneratorSource;

#[derive(Debug, Clone)]
pub struct FileSystemWorkspaceArtifactWriter<L>
where
    L: NfwConfigurationLoader,
{
    configuration_loader: L,
}

impl<L> FileSystemWorkspaceArtifactWriter<L>
where
    L: NfwConfigurationLoader,
{
    pub fn new(configuration_loader: L) -> Self {
        Self {
            configuration_loader,
        }
    }
}

impl<L> ConfigStore for FileSystemWorkspaceArtifactWriter<L>
where
    L: NfwConfigurationLoader,
{
    fn load_sources(&self) -> Result<Vec<GeneratorSource>, String> {
        self.configuration_loader
            .load_configuration()
            .map(|configuration| configuration.generator_sources)
    }

    fn save_sources(&self, sources: &[GeneratorSource]) -> Result<(), String> {
        let mut configuration = self.configuration_loader.load_configuration()?;
        configuration.generator_sources = sources.to_vec();
        self.configuration_loader.save_configuration(&configuration)
    }
}
