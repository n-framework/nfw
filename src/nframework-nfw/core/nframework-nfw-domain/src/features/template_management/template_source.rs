#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateSource {
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

impl TemplateSource {
    pub fn new(name: String, url: String, enabled: bool) -> Self {
        Self { name, url, enabled }
    }
}
