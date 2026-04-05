use nframework_nfw_core_domain::features::workspace_management::workspace_blueprint::WorkspaceBlueprint;

use crate::features::workspace_management::models::new_command_resolution::NewCommandResolution;

pub trait WorkspaceWriter {
    fn write_workspace(
        &self,
        blueprint: &WorkspaceBlueprint,
        resolution: &NewCommandResolution,
    ) -> Result<(), String>;
}
