use std::str::FromStr;

use n_framework_nfw_core_domain::features::template_management::language::Language;
use n_framework_nfw_core_domain::features::template_management::template_metadata::TemplateMetadata;
use n_framework_nfw_core_domain::features::versioning::version::Version;

use crate::features::template_management::models::errors::TemplateCatalogError;
use crate::features::template_management::models::raw_template_metadata::RawTemplateMetadata;
use crate::features::template_management::services::abstractions::validator::Validator;
use crate::features::template_management::services::abstractions::yaml_parser::YamlParser;
use crate::features::versioning::abstractions::version_comparator::VersionComparator;

#[derive(Debug, Clone)]
pub struct TemplateCatalogParser<Y, V, C>
where
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
{
    yaml_parser: Y,
    validator: V,
    version_comparator: C,
}

impl<Y, V, C> TemplateCatalogParser<Y, V, C>
where
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
{
    pub fn new(yaml_parser: Y, validator: V, version_comparator: C) -> Self {
        Self {
            yaml_parser,
            validator,
            version_comparator,
        }
    }

    pub fn parse_template_metadata(
        &self,
        yaml_content: &str,
    ) -> Result<TemplateMetadata, TemplateCatalogError> {
        let raw = self
            .yaml_parser
            .parse::<RawTemplateMetadata>(yaml_content)
            .map_err(TemplateCatalogError::InvalidYaml)?;

        let id = self.validate_and_parse_id(raw.id)?;
        let name = required_non_empty(raw.name, "name")?;
        let description = required_non_empty(raw.description, "description")?;
        let parsed_version = self.parse_and_validate_version(raw.version)?;
        let language = self.parse_optional_language(raw.language)?;
        let min_cli_version = self.parse_min_cli_version(raw.min_cli_version)?;
        let source_url = self.validate_source_url(raw.source_url)?;
        let tags = self.filter_tags(raw.tags);

        let metadata = TemplateMetadata {
            id,
            name,
            description,
            version: parsed_version,
            language,
            tags,
            author: normalize_optional(raw.author),
            min_cli_version,
            source_url,
        };

        metadata
            .validate()
            .map_err(|error| TemplateCatalogError::InvalidField {
                field: error.field(),
                reason: error.to_string(),
            })?;

        Ok(metadata)
    }
}

impl<Y, V, C> TemplateCatalogParser<Y, V, C>
where
    Y: YamlParser,
    V: Validator,
    C: VersionComparator,
{
    fn validate_and_parse_id(&self, id: Option<String>) -> Result<String, TemplateCatalogError> {
        let id = required_non_empty(id, "id")?;
        if !self.validator.is_kebab_case(&id) {
            return Err(TemplateCatalogError::InvalidField {
                field: "id",
                reason: "must use kebab-case (example: web-api)".to_owned(),
            });
        }
        Ok(id)
    }

    fn parse_and_validate_version(
        &self,
        version: Option<String>,
    ) -> Result<Version, TemplateCatalogError> {
        let version = required_non_empty(version, "version")?;
        self.version_comparator.parse(&version).map_err(|error| {
            TemplateCatalogError::InvalidField {
                field: "version",
                reason: error,
            }
        })?;
        Version::from_str(&version).map_err(|error| TemplateCatalogError::InvalidField {
            field: "version",
            reason: error.to_string(),
        })
    }

    fn parse_min_cli_version(
        &self,
        min_cli_version: Option<String>,
    ) -> Result<Option<Version>, TemplateCatalogError> {
        match min_cli_version {
            Some(value) => {
                let value = required_non_empty(Some(value), "min_cli_version")?;
                self.version_comparator.parse(&value).map_err(|error| {
                    TemplateCatalogError::InvalidField {
                        field: "min_cli_version",
                        reason: error,
                    }
                })?;
                Ok(Some(Version::from_str(&value).map_err(|error| {
                    TemplateCatalogError::InvalidField {
                        field: "min_cli_version",
                        reason: error.to_string(),
                    }
                })?))
            }
            None => Ok(None),
        }
    }

    fn validate_source_url(
        &self,
        source_url: Option<String>,
    ) -> Result<Option<String>, TemplateCatalogError> {
        let url = normalize_optional(source_url);
        if let Some(url_value) = url.as_deref()
            && !self.validator.is_git_url(url_value)
        {
            return Err(TemplateCatalogError::InvalidField {
                field: "source_url",
                reason: "must be a valid git URL".to_owned(),
            });
        }
        Ok(url)
    }

    fn parse_optional_language(
        &self,
        language: Option<String>,
    ) -> Result<Language, TemplateCatalogError> {
        let normalized = normalize_optional(language);
        match normalized {
            Some(value) => parse_language(value),
            None => Ok(Language::Neutral),
        }
    }

    fn filter_tags(&self, tags: Option<Vec<String>>) -> Vec<String> {
        tags.unwrap_or_default()
            .into_iter()
            .filter(|tag| !tag.trim().is_empty())
            .collect()
    }
}

fn required_non_empty(
    value: Option<String>,
    field: &'static str,
) -> Result<String, TemplateCatalogError> {
    let value = value.ok_or(TemplateCatalogError::MissingField { field })?;
    let normalized = value.trim();

    if normalized.is_empty() {
        return Err(TemplateCatalogError::EmptyField { field });
    }

    Ok(normalized.to_owned())
}

fn normalize_optional(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let normalized = value.trim();
        if normalized.is_empty() {
            None
        } else {
            Some(normalized.to_owned())
        }
    })
}

fn parse_language(value: String) -> Result<Language, TemplateCatalogError> {
    match value.as_str() {
        "neutral" => Ok(Language::Neutral),
        "dotnet" | "csharp" | "c#" => Ok(Language::Dotnet),
        "go" => Ok(Language::Go),
        "rust" => Ok(Language::Rust),
        _ => Err(TemplateCatalogError::UnsupportedLanguage { value }),
    }
}

#[cfg(test)]
#[path = "template_catalog_parser.tests.rs"]
mod tests;
