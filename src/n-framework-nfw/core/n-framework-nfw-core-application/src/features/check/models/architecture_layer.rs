#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CheckLayer {
    Domain,
    Application,
    Infrastructure,
    Presentation,
    Unknown,
}

impl CheckLayer {
    /// Classifies a file path into an architecture layer.
    /// The layer marker must be a complete path component (directory name), not a substring.
    /// For example:
    /// - "src/domain/services.rs" -> Domain
    /// - "src/my-domain-specific-logic.rs" -> Unknown (layer marker not a complete component)
    /// - "src/infrastructure/data.rs" -> Infrastructure
    /// - "src/n-framework-nfw-core-domain/Cargo.toml" -> Domain (contains "-core-domain")
    /// - "src/MyProject.Core.Application/MyProject.Core.Application.csproj" -> Application
    pub fn from_path(path: &str) -> Self {
        let normalized = path.to_ascii_lowercase();

        // Check each path component for exact layer matches
        for component in normalized.split(&['/', '\\']) {
            // Remove common prefixes/suffixes to isolate the layer name
            let cleaned = component
                .trim_start_matches('.')
                .trim_start_matches('_')
                .trim_start_matches('-')
                .trim_end_matches('-')
                .trim_end_matches('_');

            // Check for exact layer matches (not substrings)
            if cleaned == "domain" {
                return Self::Domain;
            }
            if cleaned == "application" {
                return Self::Application;
            }
            if cleaned == "infrastructure" {
                return Self::Infrastructure;
            }
            if cleaned == "presentation" {
                return Self::Presentation;
            }

            // Check for hyphenated compound names like "nframework-nfw-core-domain"
            // or "MyProject.Core.Domain" (dots replaced by hyphens in normalized form)
            if cleaned.contains("-core-domain") || cleaned.contains(".core.domain") {
                return Self::Domain;
            }
            if cleaned.contains("-core-application") || cleaned.contains(".core.application") {
                return Self::Application;
            }
            if cleaned.contains("-core-infrastructure") || cleaned.contains(".core.infrastructure")
            {
                return Self::Infrastructure;
            }
            if cleaned.contains("-core-presentation")
                || cleaned.contains(".core.presentation")
                || cleaned.contains(".presentation.")
                || cleaned.starts_with("presentation")
            {
                return Self::Presentation;
            }
        }

        Self::Unknown
    }
}
