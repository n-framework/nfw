use std::path::Path;

pub trait GeneratedApiContractInspector {
    fn assert_health_endpoints(&self, service_root: &Path) -> Result<(), String>;
}
