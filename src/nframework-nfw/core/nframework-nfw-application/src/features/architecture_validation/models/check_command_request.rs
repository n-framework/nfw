use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckCommandRequest {
    pub start_directory: PathBuf,
}

impl CheckCommandRequest {
    pub fn new(start_directory: PathBuf) -> Self {
        Self { start_directory }
    }
}
