use n_framework_nfw_core_domain::features::entity_generation::errors::entity_generation_error::EntityGenerationError;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::general_type::GeneralType;
use n_framework_nfw_core_domain::features::entity_generation::value_objects::property_definition::PropertyDefinition;

use std::collections::HashSet;

/// Parses CLI property definitions in `Name:Type` or `Name:Type?` format into `PropertyDefinition` values.
#[derive(Debug, Clone)]
pub struct PropertySyntaxParser;

impl PropertySyntaxParser {
    pub fn parse(input: &str) -> Result<Vec<PropertyDefinition>, EntityGenerationError> {
        let trimmed = input.trim();
        if trimmed.is_empty() {
            return Err(EntityGenerationError::EmptyProperties);
        }

        let mut definitions = Vec::new();
        let mut seen_names = HashSet::new();

        for part in trimmed.split(',') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            let definition = Self::parse_single(part)?;

            let lower_name = definition.name().to_lowercase();
            if !seen_names.insert(lower_name) {
                return Err(EntityGenerationError::DuplicatePropertyName {
                    name: definition.name().to_owned(),
                });
            }

            definitions.push(definition);
        }

        if definitions.is_empty() {
            return Err(EntityGenerationError::EmptyProperties);
        }

        Ok(definitions)
    }

    fn parse_single(input: &str) -> Result<PropertyDefinition, EntityGenerationError> {
        let Some((name, type_part)) = input.split_once(':') else {
            return Err(EntityGenerationError::InvalidPropertySyntax {
                input: input.to_owned(),
            });
        };

        let name = name.trim();
        let type_part = type_part.trim();

        if name.is_empty() || type_part.is_empty() {
            return Err(EntityGenerationError::InvalidPropertySyntax {
                input: input.to_owned(),
            });
        }

        let (cli_type, nullable) = if let Some(stripped) = type_part.strip_suffix('?') {
            (stripped, true)
        } else {
            (type_part, false)
        };

        let general_type = GeneralType::from_cli_type(cli_type).ok_or_else(|| {
            EntityGenerationError::UnsupportedPropertyType {
                cli_type: cli_type.to_owned(),
                supported: GeneralType::supported_cli_types().join(", "),
            }
        })?;

        PropertyDefinition::try_new(name.to_owned(), general_type, nullable)
    }
}

#[cfg(test)]
#[path = "property_syntax_parser.tests.rs"]
mod tests;
