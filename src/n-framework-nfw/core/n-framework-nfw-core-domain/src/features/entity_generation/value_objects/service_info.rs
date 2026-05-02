use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ServiceInfo {
    name: String,
    path: PathBuf,
    modules: Vec<String>,
    entity_specs_path: Option<PathBuf>,
}

impl ServiceInfo {
    pub fn new(
        name: String,
        path: PathBuf,
        modules: Vec<String>,
        entity_specs_path: Option<PathBuf>,
    ) -> Self {
        Self {
            name,
            path,
            modules,
            entity_specs_path,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn modules(&self) -> &[String] {
        &self.modules
    }

    pub fn has_module(&self, module: &str) -> bool {
        self.modules.iter().any(|m| m == module)
    }

    pub fn entity_specs_path(&self) -> Option<&PathBuf> {
        self.entity_specs_path.as_ref()
    }

    pub fn resolved_entity_specs_path(&self) -> PathBuf {
        self.entity_specs_path
            .clone()
            .unwrap_or_else(|| self.path.join("specs").join("entities"))
    }
}
