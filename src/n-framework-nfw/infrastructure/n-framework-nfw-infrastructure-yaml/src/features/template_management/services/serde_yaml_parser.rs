use serde::Serialize;
use serde::de::DeserializeOwned;

use n_framework_nfw_core_application::features::template_management::services::abstractions::yaml_parser::YamlParser;

#[derive(Debug, Default, Clone, Copy)]
pub struct SerdeYamlParser;

impl SerdeYamlParser {
    pub fn new() -> Self {
        Self
    }
}

impl YamlParser for SerdeYamlParser {
    fn parse<T>(&self, content: &str) -> Result<T, String>
    where
        T: DeserializeOwned,
    {
        serde_yaml::from_str(content).map_err(|error| error.to_string())
    }

    fn serialize<T>(&self, value: &T) -> Result<String, String>
    where
        T: Serialize,
    {
        serde_yaml::to_string(value).map_err(|error| error.to_string())
    }
}

#[cfg(test)]
#[path = "serde_yaml_parser.tests.rs"]
mod tests;
