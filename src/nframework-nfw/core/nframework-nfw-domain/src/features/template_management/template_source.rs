#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateSource {
    pub name: String,
    pub url: String,
    pub enabled: bool,
}

impl TemplateSource {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            enabled: true,
        }
    }

    pub fn new_disabled(name: String, url: String) -> Self {
        Self {
            name,
            url,
            enabled: false,
        }
    }
}
