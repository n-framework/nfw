use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RawTemplateMetadata {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
    pub language: Option<String>,
    pub tags: Option<Vec<String>>,
    pub author: Option<String>,
    pub min_cli_version: Option<String>,
    pub source_url: Option<String>,
    pub generators: Option<std::collections::HashMap<String, String>>,
}
