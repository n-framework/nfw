use nframework_nfw_application::features::cli::configuration::abstraction::config_store::ConfigStore;
use nframework_nfw_application::features::cli::configuration::abstraction::nfw_configuration_loader::NfwConfigurationLoader;
use nframework_nfw_domain::features::template_management::template_source::TemplateSource;

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
    fn load_sources(&self) -> Result<Vec<TemplateSource>, String> {
        self.configuration_loader
            .load_configuration()
            .map(|configuration| configuration.template_sources)
    }

    fn save_sources(&self, sources: &[TemplateSource]) -> Result<(), String> {
        let mut configuration = self.configuration_loader.load_configuration()?;
        configuration.template_sources = sources.to_vec();
        self.configuration_loader.save_configuration(&configuration)
    }
}
