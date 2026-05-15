/// Command to add a generator source.
#[derive(Debug, Clone)]
pub struct AddGeneratorSourceCommand {
    pub name: String,
    pub url: String,
}

impl AddGeneratorSourceCommand {
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}

/// Result of successfully executing an AddGeneratorSourceCommand.
#[derive(Debug, Clone)]
pub struct AddGeneratorSourceCommandResult {
    pub source_name: String,
}
