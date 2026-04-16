use std::collections::HashMap;
use std::path::{Path, PathBuf};

use mustache;
use n_framework_nfw_core_application::features::workspace_management::models::new_command_resolution::NewCommandResolution;

pub fn render_path(
    relative_path: &Path,
    resolution: &NewCommandResolution,
) -> Result<PathBuf, String> {
    let mut rendered_path = PathBuf::new();

    for component in relative_path.components() {
        let rendered_component =
            render_text(component.as_os_str().to_string_lossy().as_ref(), resolution)?;
        rendered_path.push(rendered_component);
    }

    Ok(rendered_path)
}

pub fn render_bytes(bytes: &[u8], resolution: &NewCommandResolution) -> Result<Vec<u8>, String> {
    match String::from_utf8(bytes.to_vec()) {
        Ok(renderable_text) => {
            let rendered = render_text(&renderable_text, resolution)?;
            Ok(rendered.into_bytes())
        }
        Err(_) => Ok(bytes.to_vec()),
    }
}

fn render_text(text: &str, resolution: &NewCommandResolution) -> Result<String, String> {
    let project_guid = stable_project_guid(&resolution.workspace_name, &resolution.template_id);

    // Convert to mustache data format - include both formats for backward compatibility
    let mut data = HashMap::new();

    // Mustache format keys: {{TokenName}}
    data.insert(
        "WorkspaceName".to_owned(),
        resolution.workspace_name.clone(),
    );
    data.insert("ServiceName".to_owned(), resolution.workspace_name.clone());
    data.insert("Namespace".to_owned(), resolution.namespace_base.clone());
    data.insert("ProjectGuid".to_owned(), project_guid.clone());

    // Underscore format keys: __TokenName__ (stripped of underscores for mustache lookup)
    // Note: mustache will match {{TokenName}} in template, so we just need the key to be TokenName
    // Both formats resolve to the same key since mustache strips {{ }} from the template side
    // and we use the same clean key names above

    // Compile and render the template
    let template =
        mustache::compile_str(text).map_err(|e| format!("failed to compile template: {}", e))?;

    template
        .render_to_string(&data)
        .map_err(|e| format!("failed to render template: {}", e))
}

pub fn stable_project_guid(workspace_name: &str, template_id: &str) -> String {
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
