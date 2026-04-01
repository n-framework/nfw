#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedTemplateId {
    pub source: Option<String>,
    pub template: String,
}

impl QualifiedTemplateId {
    pub fn new(source: String, template: String) -> Self {
        Self {
            source: Some(source),
            template,
        }
    }

    pub fn unqualified(template: String) -> Self {
        Self {
            source: None,
            template,
        }
    }

    pub fn is_qualified(&self) -> bool {
        self.source.is_some()
    }

    pub fn parse(identifier: &str) -> Option<Self> {
        let trimmed_identifier = identifier.trim();
        if trimmed_identifier.is_empty() {
            return None;
        }

        if let Some((source, template)) = trimmed_identifier.split_once('/') {
            let source = source.trim();
            let template = template.trim();
            if source.is_empty() || template.is_empty() {
                return None;
            }

            return Some(Self::new(source.to_owned(), template.to_owned()));
        }

        Some(Self::unqualified(trimmed_identifier.to_owned()))
    }
}
