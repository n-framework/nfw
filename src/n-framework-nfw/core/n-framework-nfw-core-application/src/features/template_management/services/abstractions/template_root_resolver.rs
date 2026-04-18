use std::path::{Path, PathBuf};
use serde_yaml::Value as YamlValue;

pub trait TemplateRootResolver: Send + Sync {
    fn resolve(&self, nfw_yaml: &YamlValue, template_id: &str, workspace_root: &Path) -> Result<PathBuf, String>;
}
