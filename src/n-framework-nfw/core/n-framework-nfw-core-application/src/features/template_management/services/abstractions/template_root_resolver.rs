use serde_yaml::Value as YamlValue;
use std::path::{Path, PathBuf};

pub trait TemplateRootResolver: Send + Sync {
    fn resolve(
        &self,
        nfw_yaml: &YamlValue,
        template_id: &str,
        workspace_root: &Path,
    ) -> Result<PathBuf, String>;
}
