use nframework_nfw_core_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;

#[derive(Debug, Default, Clone, Copy)]
pub struct WorkspaceBlueprintBuilder;

impl WorkspaceBlueprintBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(&self, workspace_name: &str) -> WorkspaceBlueprint {
        WorkspaceBlueprint::new(workspace_name)
    }
}
