use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait YamlParser {
    fn parse<T>(&self, content: &str) -> Result<T, String>
    where
        T: DeserializeOwned;

    fn serialize<T>(&self, value: &T) -> Result<String, String>
    where
        T: Serialize;
}
