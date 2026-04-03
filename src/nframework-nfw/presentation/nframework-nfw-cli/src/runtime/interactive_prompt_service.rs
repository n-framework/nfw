use std::io::{self, IsTerminal, Write};

use nframework_nfw_application::features::workspace_management::services::abstraction::prompt_service::PromptService;

#[derive(Debug, Default, Clone, Copy)]
pub struct InteractivePromptService;

impl InteractivePromptService {
    pub fn new() -> Self {
        Self
    }
}

impl PromptService for InteractivePromptService {
    fn is_interactive(&self) -> bool {
        io::stdin().is_terminal() && io::stdout().is_terminal()
    }

    fn prompt(&self, message: &str) -> Result<String, String> {
        print!("{message}: ");
        io::stdout()
            .flush()
            .map_err(|error| format!("failed to flush prompt: {error}"))?;

        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .map_err(|error| format!("failed to read prompt input: {error}"))?;

        Ok(value.trim().to_owned())
    }
}
