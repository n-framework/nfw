pub trait WorkspaceNameValidator {
    fn is_valid_workspace_name(&self, value: &str) -> bool;
}
