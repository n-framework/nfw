use crate::features::generator_management::validation::is_kebab_case;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedGeneratorId {
    pub source: Option<String>,
    pub generator: String,
}

impl QualifiedGeneratorId {
    /// Creates a new qualified generator identifier without validation.
    ///
    /// # Note
    /// This constructor does not validate the input. For validated construction,
    /// use `try_new()` which returns a `Result`.
    pub fn new(source: String, generator: String) -> Self {
        Self {
            source: Some(source),
            generator,
        }
    }

    /// Creates a new qualified generator identifier with validation.
    ///
    /// # Errors
    /// Returns an error if:
    /// - source is empty
    /// - generator is empty
    /// - source contains '/' character
    /// - generator is not valid kebab-case
    pub fn try_new(source: String, generator: String) -> Result<Self, String> {
        let source = source.trim();
        let generator = generator.trim();

        if source.is_empty() {
            return Err("source cannot be empty".to_owned());
        }

        if generator.is_empty() {
            return Err("generator cannot be empty".to_owned());
        }

        if source.contains('/') {
            return Err("source cannot contain '/' character".to_owned());
        }

        if !is_kebab_case(generator) {
            return Err(
                "generator must use kebab-case (lowercase letters, numbers, hyphens)".to_owned(),
            );
        }

        Ok(Self {
            source: Some(source.to_owned()),
            generator: generator.to_owned(),
        })
    }

    /// Creates an unqualified generator identifier without validation.
    ///
    /// # Note
    /// This constructor does not validate the input. For validated construction,
    /// use `try_unqualified()` which returns a `Result`.
    pub fn unqualified(generator: String) -> Self {
        Self {
            source: None,
            generator,
        }
    }

    /// Creates an unqualified generator identifier with validation.
    ///
    /// # Errors
    /// Returns an error if:
    /// - generator is empty
    /// - generator is not valid kebab-case
    pub fn try_unqualified(generator: String) -> Result<Self, String> {
        let generator = generator.trim();

        if generator.is_empty() {
            return Err("generator cannot be empty".to_owned());
        }

        if !is_kebab_case(generator) {
            return Err(
                "generator must use kebab-case (lowercase letters, numbers, hyphens)".to_owned(),
            );
        }

        Ok(Self {
            source: None,
            generator: generator.to_owned(),
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

        if let Some((source, generator)) = trimmed_identifier.split_once('/') {
            let source = source.trim();
            let generator = generator.trim();
            if source.is_empty() || generator.is_empty() {
                return None;
            }

            return Some(Self::new(source.to_owned(), generator.to_owned()));
        }

        Some(Self::unqualified(trimmed_identifier.to_owned()))
    }
}

#[cfg(test)]
#[path = "qualified_generator_id.tests.rs"]
mod tests;
