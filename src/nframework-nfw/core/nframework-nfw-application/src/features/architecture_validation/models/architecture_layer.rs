#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArchitectureLayer {
    Domain,
    Application,
    Infrastructure,
    Presentation,
    Unknown,
}

impl ArchitectureLayer {
    pub fn from_path(path: &str) -> Self {
        let normalized = path.to_ascii_lowercase();

        if contains_layer_marker(&normalized, "domain") {
            return Self::Domain;
        }

        if contains_layer_marker(&normalized, "application") {
            return Self::Application;
        }

        if contains_layer_marker(&normalized, "infrastructure") {
            return Self::Infrastructure;
        }

        if contains_layer_marker(&normalized, "presentation") {
            return Self::Presentation;
        }

        Self::Unknown
    }
}

fn contains_layer_marker(path: &str, layer: &str) -> bool {
    let markers = [
        format!(".{layer}"),
        format!("-{layer}"),
        format!("/{layer}"),
        format!("\\{layer}"),
        format!("_{layer}"),
    ];

    markers.iter().any(|marker| path.contains(marker))
}
