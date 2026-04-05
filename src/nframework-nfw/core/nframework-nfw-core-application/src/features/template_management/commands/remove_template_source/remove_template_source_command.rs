/// Command to remove a template source.
#[derive(Debug, Clone)]
pub struct RemoveTemplateSourceCommand {
    pub name: String,
}

impl RemoveTemplateSourceCommand {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// Result of successfully executing a RemoveTemplateSourceCommand.
#[derive(Debug, Clone)]
pub struct RemoveTemplateSourceCommandResult {
    pub source_name: String,
}
