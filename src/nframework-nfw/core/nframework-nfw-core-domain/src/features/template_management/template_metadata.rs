use crate::features::template_management::errors::TemplateMetadataValidationError;
use crate::features::template_management::language::Language;
use crate::features::template_management::validation::is_kebab_case;
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

#[derive(Default)]
pub struct TemplateMetadataBuilder {
    id: Option<String>,
    name: Option<String>,
    description: Option<String>,
    version: Option<Version>,
    language: Option<Language>,
    tags: Option<Vec<String>>,
    author: Option<String>,
    min_cli_version: Option<Version>,
    source_url: Option<String>,
}

impl TemplateMetadataBuilder {
    pub fn id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }

    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    pub fn language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    pub fn tags(mut self, tags: Vec<String>) -> Self {
        self.tags = Some(tags);
        self
    }

    pub fn author(mut self, author: Option<String>) -> Self {
        self.author = author;
        self
    }

    pub fn min_cli_version(mut self, min_cli_version: Option<Version>) -> Self {
        self.min_cli_version = min_cli_version;
        self
    }

    pub fn source_url(mut self, source_url: Option<String>) -> Self {
        self.source_url = source_url;
        self
    }

    pub fn build(self) -> Result<TemplateMetadata, TemplateMetadataValidationError> {
        let metadata = TemplateMetadata {
            id: self
                .id
                .ok_or_else(|| TemplateMetadataValidationError::missing_field("id"))?,
            name: self
                .name
                .ok_or_else(|| TemplateMetadataValidationError::missing_field("name"))?,
            description: self
                .description
                .ok_or_else(|| TemplateMetadataValidationError::missing_field("description"))?,
            version: self
                .version
                .ok_or_else(|| TemplateMetadataValidationError::missing_field("version"))?,
            language: self.language.unwrap_or(Language::Neutral),
            tags: self.tags.unwrap_or_default(),
            author: self.author,
            min_cli_version: self.min_cli_version,
            source_url: self.source_url,
        };
        metadata.validate()?;
        Ok(metadata)
    }
}

impl TemplateMetadata {
    pub fn builder() -> TemplateMetadataBuilder {
        TemplateMetadataBuilder::default()
    }

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
