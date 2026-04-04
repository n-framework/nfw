use std::path::Path;

pub trait GeneratedProjectDependencyInspector {
    fn inspect_dependencies(&self, service_root: &Path) -> Result<Vec<(String, String)>, String>;
}
