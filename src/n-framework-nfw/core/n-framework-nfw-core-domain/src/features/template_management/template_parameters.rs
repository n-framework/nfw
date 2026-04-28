use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

/// A type-safe container for template parameters used during rendering.
/// Standardizes keys like 'Name', 'Namespace', and 'Feature' while allowing
/// arbitrary custom parameters. Validates that keys are valid identifiers.
///
/// # Security
///
/// This container enforces that all keys are valid identifiers (alphanumeric, dots,
/// hyphens, underscores, or curly braces for interpolation). This prevents
/// injection attacks where malicious keys could manipulate template logic.
/// Standard keys like `Name` and `Namespace` are protected with dedicated setters
/// that enforce non-empty invariants.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TemplateParameters {
    inner: BTreeMap<String, Value>,
}

impl TemplateParameters {
    /// Reserved key for the project/item name.
    pub const KEY_NAME: &'static str = "Name";
    /// Reserved key for the feature name.
    pub const KEY_FEATURE: &'static str = "Feature";
    /// Reserved key for the workspace namespace.
    pub const KEY_NAMESPACE: &'static str = "Namespace";
    /// Reserved key for the service name.
    pub const KEY_SERVICE: &'static str = "Service";

    /// Creates a new, empty set of template parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the 'Name' parameter.
    pub fn with_name(mut self, name: impl Into<String>) -> Result<Self, String> {
        let name_val = name.into();
        if name_val.trim().is_empty() {
            return Err("name parameter cannot be empty".to_string());
        }
        self.inner
            .insert(Self::KEY_NAME.to_string(), Value::String(name_val));
        Ok(self)
    }

    /// Sets the 'Feature' parameter.
    pub fn with_feature(mut self, feature: impl Into<String>) -> Result<Self, String> {
        let feature_val = feature.into();
        if feature_val.trim().is_empty() {
            return Err("feature parameter cannot be empty".to_string());
        }
        self.inner
            .insert(Self::KEY_FEATURE.to_string(), Value::String(feature_val));
        Ok(self)
    }

    /// Sets the 'Namespace' parameter.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Result<Self, String> {
        let namespace_val = namespace.into();
        if namespace_val.trim().is_empty() {
            return Err("namespace parameter cannot be empty".to_string());
        }
        self.inner.insert(
            Self::KEY_NAMESPACE.to_string(),
            Value::String(namespace_val),
        );
        Ok(self)
    }

    /// Sets the 'Service' parameter.
    pub fn with_service(mut self, service: impl Into<String>) -> Result<Self, String> {
        let service_val = service.into();
        if service_val.trim().is_empty() {
            return Err("service parameter cannot be empty".to_string());
        }
        self.inner
            .insert(Self::KEY_SERVICE.to_string(), Value::String(service_val));
        Ok(self)
    }

    /// Inserts a custom string parameter. Validates the key is a valid identifier.
    ///
    /// # Errors
    /// Returns an error if the key contains invalid characters or is empty.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), String> {
        self.insert_value(key, Value::String(value.into()))
    }

    /// Inserts a custom JSON parameter. Validates the key is a valid identifier.
    pub fn insert_value(&mut self, key: impl Into<String>, value: Value) -> Result<(), String> {
        let k = key.into();

        if k.trim().is_empty() {
            return Err("parameter key cannot be empty".to_string());
        }

        let re = get_parameter_key_regex();
        if !re.is_match(&k) {
            return Err(format!(
                "invalid parameter key '{}'. Names must be alphanumeric or standard placeholder formats.",
                k
            ));
        }

        self.inner.insert(k, value);
        Ok(())
    }

    /// Gets a string parameter by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.inner.get(key).and_then(|v| v.as_str())
    }

    /// Gets a json parameter by key.
    pub fn get_value(&self, key: &str) -> Option<&Value> {
        self.inner.get(key)
    }

    /// Returns the name parameter if set.
    pub fn name(&self) -> Option<&str> {
        self.get(Self::KEY_NAME)
    }

    /// Returns the feature parameter if set.
    pub fn feature(&self) -> Option<&str> {
        self.get(Self::KEY_FEATURE)
    }

    /// Returns the namespace parameter if set.
    pub fn namespace(&self) -> Option<&str> {
        self.get(Self::KEY_NAMESPACE)
    }

    /// Returns the service parameter if set.
    pub fn service(&self) -> Option<&str> {
        self.get(Self::KEY_SERVICE)
    }

    /// Returns a reference to the underlying map.
    pub fn as_map(&self) -> &BTreeMap<String, Value> {
        &self.inner
    }
}

fn get_parameter_key_regex() -> &'static regex::Regex {
    use std::sync::OnceLock;
    static KEY_REGEX: OnceLock<regex::Regex> = OnceLock::new();
    KEY_REGEX.get_or_init(|| {
        regex::Regex::new("^[a-zA-Z0-9_.\\-{}]+$").expect("invalid parameter key regex")
    })
}

impl TryFrom<BTreeMap<String, String>> for TemplateParameters {
    type Error = String;

    fn try_from(inner: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut params = Self::new();
        for (k, v) in inner {
            params.insert(k, v)?;
        }
        Ok(params)
    }
}

impl TryFrom<BTreeMap<String, Value>> for TemplateParameters {
    type Error = String;

    fn try_from(inner: BTreeMap<String, Value>) -> Result<Self, Self::Error> {
        let mut params = Self::new();
        for (k, v) in inner {
            params.insert_value(k, v)?;
        }
        Ok(params)
    }
}

impl TryFrom<serde_json::Value> for TemplateParameters {
    type Error = String;

    fn try_from(value: serde_json::Value) -> Result<Self, Self::Error> {
        match value {
            Value::Object(map) => {
                let mut params = Self::new();
                for (k, v) in map {
                    params.insert_value(k, v)?;
                }
                Ok(params)
            }
            _ => Err("TemplateParameters must be constructed from a JSON object".to_string()),
        }
    }
}

#[cfg(test)]
#[path = "template_parameters.tests.rs"]
mod tests;
