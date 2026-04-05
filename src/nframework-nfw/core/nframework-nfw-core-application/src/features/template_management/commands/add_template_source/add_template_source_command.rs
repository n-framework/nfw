/// Command to add a template source.
#[derive(Debug, Clone)]
pub struct AddTemplateSourceCommand {
    pub name: String,
    pub url: String,
}

impl AddTemplateSourceCommand {
    pub fn new(name: String, url: String) -> Self {
        Self { name, url }
    }
}

/// Result of successfully executing an AddTemplateSourceCommand.
#[derive(Debug, Clone)]
pub struct AddTemplateSourceCommandResult {
    pub source_name: String,
}
