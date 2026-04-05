use nframework_nfw_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;
use std::path::{Path, PathBuf};

pub fn render_path(relative_path: &Path, resolution: &NewCommandResolution) -> PathBuf {
    let mut rendered_path = PathBuf::new();

    for component in relative_path.components() {
        let rendered_component =
            render_text(component.as_os_str().to_string_lossy().as_ref(), resolution);
        rendered_path.push(rendered_component);
    }

    rendered_path
}

pub fn render_bytes(bytes: &[u8], resolution: &NewCommandResolution) -> Vec<u8> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(renderable_text) => render_text(&renderable_text, resolution).into_bytes(),
        Err(_) => bytes.to_vec(),
    }
}

fn render_text(text: &str, resolution: &NewCommandResolution) -> String {
    let project_guid = stable_project_guid(&resolution.workspace_name, &resolution.template_id);
    text.replace("__WorkspaceName__", &resolution.workspace_name)
        .replace("__ServiceName__", &resolution.workspace_name)
        .replace("__Namespace__", &resolution.namespace_base)
        .replace("__ProjectGuid__", &project_guid)
}

fn stable_project_guid(workspace_name: &str, template_id: &str) -> String {
    let mut state_a: u64 = 0xcbf29ce484222325;
    let mut state_b: u64 = 0x8422_2325_cbf2_9ce4;
    for byte in workspace_name.bytes().chain(template_id.bytes()) {
        state_a ^= byte as u64;
        state_a = state_a.wrapping_mul(0x100000001b3);

        state_b ^= (byte as u64) << 1;
        state_b = state_b.wrapping_mul(0x100000001b3);
    }

    let part1 = (state_a >> 32) as u32;
    let part2 = ((state_a >> 16) & 0xffff) as u16;
    let part3 = (state_a & 0xffff) as u16;
    let part4 = ((state_b >> 48) & 0xffff) as u16;
    let part5 = state_b & 0xffff_ffff_ffff;

    format!("{part1:08x}-{part2:04x}-{part3:04x}-{part4:04x}-{part5:012x}")
}
