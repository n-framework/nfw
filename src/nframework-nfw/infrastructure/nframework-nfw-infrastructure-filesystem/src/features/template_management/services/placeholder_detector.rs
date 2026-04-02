use std::collections::BTreeSet;
use std::path::Path;
use std::sync::OnceLock;

use regex::Regex;

#[derive(Debug, Default, Clone, Copy)]
pub struct PlaceholderDetector;

impl PlaceholderDetector {
    pub fn new() -> Self {
        Self
    }

    pub fn detect_in_name(&self, entry_name: &str) -> Vec<String> {
        placeholder_regex()
            .find_iter(entry_name)
            .map(|placeholder| placeholder.as_str().to_owned())
            .collect()
    }

    pub fn detect_in_path(&self, path: &Path) -> Vec<String> {
        let mut placeholders = BTreeSet::new();

        for component in path.components() {
            let component_name = component.as_os_str().to_string_lossy();
            for placeholder in self.detect_in_name(&component_name) {
                placeholders.insert(placeholder);
            }
        }

        placeholders.into_iter().collect()
    }
}

/// Returns a compiled regex for matching placeholder patterns.
///
/// The regex pattern `__[A-Z][A-Za-z0-9]*__` matches placeholders like `__NAME__`, `__VERSION__`, etc.
///
/// # Panics
/// This function will panic on first call if the hardcoded regex pattern is invalid.
/// Since the pattern is a compile-time constant and has been validated, this should never happen.
fn placeholder_regex() -> &'static Regex {
    static PLACEHOLDER_REGEX: OnceLock<Regex> = OnceLock::new();
    PLACEHOLDER_REGEX.get_or_init(|| {
        Regex::new(r"__[A-Z][A-Za-z0-9]*__")
            .expect("hardcoded placeholder regex pattern is invalid - this is a bug in the code")
    })
}
