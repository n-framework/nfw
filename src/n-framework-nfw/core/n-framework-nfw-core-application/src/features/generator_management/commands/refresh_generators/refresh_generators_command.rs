/// Command to refresh generator catalogs from sources.
#[derive(Debug, Clone, Default)]
pub struct RefreshGeneratorsCommand;

/// Result of successfully executing a RefreshGeneratorsCommand.
#[derive(Debug, Clone)]
pub struct RefreshGeneratorsCommandResult {
    pub source_count: usize,
    pub generator_count: usize,
    pub warnings: Vec<String>,
}
