use serde::{Deserialize, Serialize};

use crate::features::cli::configuration::models::source_config::SourceConfig;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SourcesFile {
    #[serde(default)]
    pub sources: Vec<SourceConfig>,
}
