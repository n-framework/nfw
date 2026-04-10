#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddServiceCommandRequest {
    pub service_name: Option<String>,
    pub template_id: Option<String>,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

impl AddServiceCommandRequest {
    pub fn new(
        service_name: Option<String>,
        template_id: Option<String>,
        no_input: bool,
        is_interactive_terminal: bool,
    ) -> Self {
        Self {
            service_name,
            template_id,
            no_input,
            is_interactive_terminal,
        }
    }

    pub fn is_non_interactive(&self) -> bool {
        self.no_input || !self.is_interactive_terminal
    }
}
