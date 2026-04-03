pub trait PromptService {
    fn is_interactive(&self) -> bool;
    fn prompt(&self, message: &str) -> Result<String, String>;
}
