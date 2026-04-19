#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceBlueprint {
    pub workspace_name: String,
    pub root_directories: Vec<String>,
}

impl WorkspaceBlueprint {
    pub fn new(workspace_name: impl Into<String>) -> Self {
        let workspace_name = workspace_name.into();

        Self {
            workspace_name,
            root_directories: vec!["src".to_owned(), "tests".to_owned(), "docs".to_owned()],
        }
    }
}

#[cfg(test)]
#[path = "workspace_blueprint.tests.rs"]
mod tests;
