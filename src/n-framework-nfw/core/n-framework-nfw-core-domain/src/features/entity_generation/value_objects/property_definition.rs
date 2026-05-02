use super::general_type::GeneralType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyDefinition {
    name: String,
    cli_type: String,
    general_type: GeneralType,
    nullable: bool,
}

impl PropertyDefinition {
    pub fn new(name: String, cli_type: String, general_type: GeneralType, nullable: bool) -> Self {
        Self {
            name,
            cli_type,
            general_type,
            nullable,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cli_type(&self) -> &str {
        &self.cli_type
    }

    pub fn general_type(&self) -> &GeneralType {
        &self.general_type
    }

    pub fn nullable(&self) -> bool {
        self.nullable
    }
}
