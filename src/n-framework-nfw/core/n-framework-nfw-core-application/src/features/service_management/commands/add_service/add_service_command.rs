use crate::features::service_management::models::add_service_command_request::AddServiceCommandRequest;

#[derive(Debug, Clone)]
pub struct AddServiceCommand {
    pub service_name: Option<String>,
    pub template_id: Option<String>,
    pub no_input: bool,
    pub is_interactive_terminal: bool,
}

impl AddServiceCommand {
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

    pub fn to_request(&self) -> AddServiceCommandRequest {
        AddServiceCommandRequest::new(
            self.service_name.clone(),
            self.template_id.clone(),
            self.no_input,
            self.is_interactive_terminal,
        )
    }
}

#[derive(Debug, Clone)]
pub struct AddServiceCommandResult {
    pub service_name: String,
    pub output_path: std::path::PathBuf,
    pub template_id: String,
    pub template_version: String,
}
