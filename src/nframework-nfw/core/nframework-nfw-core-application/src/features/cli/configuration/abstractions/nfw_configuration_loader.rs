use crate::features::cli::configuration::nfw_configuration::NfwConfiguration;

pub trait NfwConfigurationLoader {
    fn load_configuration(&self) -> Result<NfwConfiguration, String>;
    fn save_configuration(&self, configuration: &NfwConfiguration) -> Result<(), String>;
}
