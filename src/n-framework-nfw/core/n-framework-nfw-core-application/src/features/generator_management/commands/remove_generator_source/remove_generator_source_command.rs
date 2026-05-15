/// Command to remove a generator source.
#[derive(Debug, Clone)]
pub struct RemoveGeneratorSourceCommand {
    pub name: String,
}

impl RemoveGeneratorSourceCommand {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

/// Result of successfully executing a RemoveGeneratorSourceCommand.
#[derive(Debug, Clone)]
pub struct RemoveGeneratorSourceCommandResult {
    pub source_name: String,
}
