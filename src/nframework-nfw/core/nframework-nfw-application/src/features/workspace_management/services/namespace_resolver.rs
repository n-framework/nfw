use nframework_nfw_domain::features::workspace_management::namespace_convention::NamespaceConvention;

#[derive(Debug, Default, Clone, Copy)]
pub struct NamespaceResolver;

impl NamespaceResolver {
    pub fn new() -> Self {
        Self
    }

    pub fn resolve_workspace_base_namespace(&self, workspace_name: &str) -> String {
        NamespaceConvention::from_workspace_name(workspace_name).workspace_base_namespace
    }
}
