use std::str::FromStr;

use nframework_nfw_domain::features::template_management::language::Language;
use nframework_nfw_domain::features::template_management::template_metadata::TemplateMetadata;
use nframework_nfw_domain::features::versioning::version::Version;

use crate::features::template_management::models::errors::TemplateCatalogError;
use crate::features::template_management::models::raw_template_metadata::RawTemplateMetadata;
use crate::features::template_management::services::abstraction::validator::Validator;
use crate::features::template_management::services::abstraction::yaml_parser::YamlParser;
use crate::features::versioning::abstraction::version_comparator::VersionComparator;

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

        let id = required_non_empty(raw.id, "id")?;
        if !self.validator.is_kebab_case(&id) {
            return Err(TemplateCatalogError::InvalidField {
                field: "id",
                reason: "must use kebab-case (example: web-api)".to_owned(),
            });
        }

        let name = required_non_empty(raw.name, "name")?;
        let description = required_non_empty(raw.description, "description")?;

        let version = required_non_empty(raw.version, "version")?;
        self.version_comparator.parse(&version).map_err(|error| {
            TemplateCatalogError::InvalidField {
                field: "version",
                reason: error,
            }
        })?;
        let parsed_version =
            Version::from_str(&version).map_err(|error| TemplateCatalogError::InvalidField {
                field: "version",
                reason: error.to_string(),
            })?;

        let language = parse_language(required_non_empty(raw.language, "language")?)?;

        let min_cli_version = match raw.min_cli_version {
            Some(value) => {
                let value = required_non_empty(Some(value), "min_cli_version")?;
                self.version_comparator.parse(&value).map_err(|error| {
                    TemplateCatalogError::InvalidField {
                        field: "min_cli_version",
                        reason: error,
                    }
                })?;
                Some(Version::from_str(&value).map_err(|error| {
                    TemplateCatalogError::InvalidField {
                        field: "min_cli_version",
                        reason: error.to_string(),
                    }
                })?)
            }
            None => None,
        };

        let source_url = normalize_optional(raw.source_url);
        if let Some(url) = source_url.as_deref()
            && !self.validator.is_git_url(url)
        {
            return Err(TemplateCatalogError::InvalidField {
                field: "source_url",
                reason: "must be a valid git URL".to_owned(),
            });
        }

        let tags = raw
            .tags
            .unwrap_or_default()
            .into_iter()
            .filter(|tag| !tag.trim().is_empty())
            .collect::<Vec<_>>();

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
        "dotnet" => Ok(Language::Dotnet),
        "go" => Ok(Language::Go),
        "rust" => Ok(Language::Rust),
        _ => Err(TemplateCatalogError::UnsupportedLanguage { value }),
    }
}
