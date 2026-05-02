use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneralType {
    String,
    Integer,
    Decimal,
    Boolean,
    DateTime,
    Uuid,
    Bytes,
}

impl GeneralType {
    pub fn from_cli_type(cli_type: &str) -> Option<Self> {
        match cli_type {
            "string" => Some(Self::String),
            "int" | "long" => Some(Self::Integer),
            "decimal" | "double" | "float" => Some(Self::Decimal),
            "bool" => Some(Self::Boolean),
            "DateTime" | "DateTimeOffset" => Some(Self::DateTime),
            "Guid" => Some(Self::Uuid),
            "byte[]" => Some(Self::Bytes),
            _ => None,
        }
    }

    pub fn supported_cli_types() -> &'static [&'static str] {
        &[
            "string",
            "int",
            "long",
            "decimal",
            "double",
            "float",
            "bool",
            "DateTime",
            "DateTimeOffset",
            "Guid",
            "byte[]",
        ]
    }
}

impl fmt::Display for GeneralType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String => write!(f, "string"),
            Self::Integer => write!(f, "integer"),
            Self::Decimal => write!(f, "decimal"),
            Self::Boolean => write!(f, "boolean"),
            Self::DateTime => write!(f, "datetime"),
            Self::Uuid => write!(f, "uuid"),
            Self::Bytes => write!(f, "bytes"),
        }
    }
}

impl FromStr for GeneralType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "string" => Ok(Self::String),
            "integer" => Ok(Self::Integer),
            "decimal" => Ok(Self::Decimal),
            "boolean" => Ok(Self::Boolean),
            "datetime" => Ok(Self::DateTime),
            "uuid" => Ok(Self::Uuid),
            "bytes" => Ok(Self::Bytes),
            _ => Err(format!("unknown general type: {s}")),
        }
    }
}

#[cfg(test)]
#[path = "general_type.tests.rs"]
mod tests;
