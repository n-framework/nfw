#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewCommandRequest {
    pub workspace_name: Option<String>,
    pub template_id: Option<String>,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

impl NewCommandRequest {
    pub fn new(
        workspace_name: Option<String>,
        template_id: Option<String>,
        no_input: bool,
        is_interactive_terminal: bool,
    ) -> Self {
        Self {
            workspace_name,
            template_id,
            no_input,
            is_interactive_terminal,
        }
    }
}
