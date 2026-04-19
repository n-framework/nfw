use crate::features::template_management::validation::is_kebab_case;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedTemplateId {
    pub source: Option<String>,
    pub template: String,
}

impl QualifiedTemplateId {
    /// Creates a new qualified template identifier without validation.
    ///
    /// # Note
    /// This constructor does not validate the input. For validated construction,
    /// use `try_new()` which returns a `Result`.
    pub fn new(source: String, template: String) -> Self {
        Self {
            source: Some(source),
            template,
        }
    }

    /// Creates a new qualified template identifier with validation.
    ///
    /// # Errors
    /// Returns an error if:
    /// - source is empty
    /// - template is empty
    /// - source contains '/' character
    /// - template is not valid kebab-case
    pub fn try_new(source: String, template: String) -> Result<Self, String> {
        let source = source.trim();
        let template = template.trim();

        if source.is_empty() {
            return Err("source cannot be empty".to_owned());
        }

        if template.is_empty() {
            return Err("template cannot be empty".to_owned());
        }

        if source.contains('/') {
            return Err("source cannot contain '/' character".to_owned());
        }

        if !is_kebab_case(template) {
            return Err(
                "template must use kebab-case (lowercase letters, numbers, hyphens)".to_owned(),
            );
        }

        Ok(Self {
            source: Some(source.to_owned()),
            template: template.to_owned(),
        })
    }

    /// Creates an unqualified template identifier without validation.
    ///
    /// # Note
    /// This constructor does not validate the input. For validated construction,
    /// use `try_unqualified()` which returns a `Result`.
    pub fn unqualified(template: String) -> Self {
        Self {
            source: None,
            template,
        }
    }

    /// Creates an unqualified template identifier with validation.
    ///
    /// # Errors
    /// Returns an error if:
    /// - template is empty
    /// - template is not valid kebab-case
    pub fn try_unqualified(template: String) -> Result<Self, String> {
        let template = template.trim();

        if template.is_empty() {
            return Err("template cannot be empty".to_owned());
        }

        if !is_kebab_case(template) {
            return Err(
                "template must use kebab-case (lowercase letters, numbers, hyphens)".to_owned(),
            );
        }

        Ok(Self {
            source: None,
            template: template.to_owned(),
        })
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

#[cfg(test)]
#[path = "qualified_template_id.tests.rs"]
mod tests;
