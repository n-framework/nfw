use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A type-safe container for template parameters used during rendering.
/// Standardizes keys like 'Name', 'Namespace', and 'Feature' while allowing
/// arbitrary custom parameters.
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
        self.inner.insert("Name".to_string(), name.into());
        self
    }

    /// Sets the 'Feature' parameter.
    pub fn with_feature(mut self, feature: impl Into<String>) -> Self {
        self.inner.insert("Feature".to_string(), feature.into());
        self
    }

    /// Sets the 'Namespace' parameter.
    pub fn with_namespace(mut self, namespace: impl Into<String>) -> Self {
        self.inner.insert("Namespace".to_string(), namespace.into());
        self
    }

    /// Inserts a custom parameter.
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.inner.insert(key.into(), value.into());
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
    /// This is currently used for compatibility with the Tera engine.
    pub fn as_map(&self) -> &BTreeMap<String, String> {
        &self.inner
    }
}

impl From<BTreeMap<String, String>> for TemplateParameters {
    fn from(inner: BTreeMap<String, String>) -> Self {
        Self { inner }
    }
}
