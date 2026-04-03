/// Command to refresh template catalogs from sources.
#[derive(Debug, Clone, Default)]
pub struct RefreshTemplatesCommand;

/// Result of successfully executing a RefreshTemplatesCommand.
#[derive(Debug, Clone)]
pub struct RefreshTemplatesCommandResult {
    pub source_count: usize,
    pub template_count: usize,
    pub warnings: Vec<String>,
}
