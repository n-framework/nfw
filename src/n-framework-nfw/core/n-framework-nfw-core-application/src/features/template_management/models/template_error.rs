#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    #[error("template configuration error: {0}")]
    ConfigError(String),
    #[error("template rendering error: {0}")]
    RenderError(String),
    #[error("template injection error: {0}")]
    InjectionError(String),
    #[error("filesystem error: {0}")]
    FileSystemError(String),
}
