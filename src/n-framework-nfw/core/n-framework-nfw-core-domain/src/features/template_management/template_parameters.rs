use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A type-safe container for template parameters used during rendering.
/// Standardizes keys like 'Name', 'Namespace', and 'Feature' while allowing
/// arbitrary custom parameters. Validates that keys are valid identifiers.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct TemplateParameters {
    inner: BTreeMap<String, String>,
}

impl TemplateParameters {
    /// Creates a new, empty set of template parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the 'Name' parameter.
    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        let name_val = name.into();
        if !name_val.trim().is_empty() {
            self.inner.insert("Name".to_string(), name_val);
        }
        self
    }

    /// Sets the 'Feature' parameter.
    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        let feature_val = feature.into();
        if !feature_val.trim().is_empty() {
            self.inner.insert("Feature".to_string(), feature_val);
        }
        self
    }

    /// Sets the 'Namespace' parameter.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        let namespace_val = namespace.into();
        if !namespace_val.trim().is_empty() {
            self.inner.insert("Namespace".to_string(), namespace_val);
        }
        self
    }

    /// Inserts a custom parameter. Validates the key is a valid identifier.
    ///
    /// # Errors
    /// Returns an error if the key contains invalid characters or is empty.
    pub fn insert(
        &mut self,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> Result<(), String> {
        let k = key.into();
        let v = value.into();

        if k.trim().is_empty() {
            return Err("parameter key cannot be empty".to_string());
        }

        // Validate key format (alphanumeric, underscores, dots, hyphens, and braces/underscores for legacy)
        // We allow {{TOKEN}} and __TOKEN__ format keys for backward compatibility but encourage clean names.
        let re = get_parameter_key_regex();
        if !re.is_match(&k) {
            return Err(format!(
                "invalid parameter key '{}'. Names must be alphanumeric or standard placeholder formats.",
                k
            ));
        }

        self.inner.insert(k, v);
        Ok(())
    }

    /// Gets a parameter by key.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.inner.get(key).map(|s| s.as_str())
    }

    /// Returns the name parameter if set.
    pub fn name(&self) -> Option<&str> {
        self.get("Name")
    }

    /// Returns the feature parameter if set.
    pub fn feature(&self) -> Option<&str> {
        self.get("Feature")
    }

    /// Returns the namespace parameter if set.
    pub fn namespace(&self) -> Option<&str> {
        self.get("Namespace")
    }

    /// Returns a reference to the underlying map.
    pub fn as_map(&self) -> &BTreeMap<String, String> {
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


impl From<BTreeMap<String, String>> for TemplateParameters {
    fn from(inner: BTreeMap<String, String>) -> Self {
        Self { inner }
    }
}
