use crate::features::template_management::errors::TemplateMetadataValidationError;
use crate::features::template_management::language::Language;
use crate::features::versioning::version::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: Version,
    pub language: Language,
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub min_cli_version: Option<Version>,
    pub source_url: Option<String>,
}

impl TemplateMetadata {
    pub fn validate(&self) -> Result<(), TemplateMetadataValidationError> {
        if self.id.trim().is_empty() {
            return Err(TemplateMetadataValidationError::missing_field("id"));
        }

        if !is_kebab_case(&self.id) {
            return Err(TemplateMetadataValidationError::invalid_field(
                "id",
                "id must use kebab-case (lowercase letters, numbers, hyphens)",
            ));
        }

        if self.name.trim().is_empty() {
            return Err(TemplateMetadataValidationError::missing_field("name"));
        }

        if self.description.trim().is_empty() {
            return Err(TemplateMetadataValidationError::missing_field(
                "description",
            ));
        }

        if self.tags.iter().any(|tag| tag.trim().is_empty()) {
            return Err(TemplateMetadataValidationError::invalid_field(
                "tags",
                "tags cannot include empty values",
            ));
        }

        Ok(())
    }
}

fn is_kebab_case(value: &str) -> bool {
    if value.starts_with('-') || value.ends_with('-') || value.contains("--") {
        return false;
    }

    value.chars().all(|character| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
    })
}
