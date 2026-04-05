use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde_yaml::Value;

use crate::features::service_management::models::errors::add_service_error::AddServiceError;
use crate::features::service_management::models::service_generation_plan::ServiceGenerationPlan;
use crate::features::service_management::models::service_template_resolution::ServiceTemplateResolution;

#[derive(Debug, Default, Clone, Copy)]
pub struct ServiceGenerationPlanBuilder;

impl ServiceGenerationPlanBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(
        &self,
        service_name: &str,
        workspace_root: &Path,
        template_resolution: &ServiceTemplateResolution,
    ) -> Result<ServiceGenerationPlan, AddServiceError> {
        let output_root = workspace_root.join("src").join(service_name);
        if output_root.exists() {
            return Err(AddServiceError::TargetDirectoryAlreadyExists(
                output_root.display().to_string(),
            ));
        }

        let workspace_namespace =
            read_workspace_namespace(workspace_root).unwrap_or_else(|| "NFramework".to_owned());
        let workspace_name =
            read_workspace_name(workspace_root).unwrap_or_else(|| "workspace".to_owned());
        let namespace = format!("{workspace_namespace}.{service_name}");
        let qualified_template_id = template_resolution.qualified_template_id();

        let mut placeholder_values = BTreeMap::<String, String>::new();
        placeholder_values.insert("__WorkspaceName__".to_owned(), workspace_name.clone());
        placeholder_values.insert("__ServiceName__".to_owned(), service_name.to_owned());
        placeholder_values.insert("__Namespace__".to_owned(), namespace.clone());
        placeholder_values.insert(
            "__ProjectGuid__".to_owned(),
            stable_project_guid(service_name, &qualified_template_id),
        );

        Ok(ServiceGenerationPlan {
            service_name: service_name.to_owned(),
            output_root,
            template_cache_path: template_resolution.template_cache_path.clone(),
            template_id: qualified_template_id,
            template_version: template_resolution.resolved_version.clone(),
            namespace,
            placeholder_values,
        })
    }
}

fn read_workspace_namespace(workspace_root: &Path) -> Option<String> {
    let yaml = load_workspace_yaml(workspace_root)?;
    yaml.get("workspace")?
        .get("namespace")?
        .as_str()
        .map(ToOwned::to_owned)
}

fn read_workspace_name(workspace_root: &Path) -> Option<String> {
    let yaml = load_workspace_yaml(workspace_root)?;
    yaml.get("workspace")?
        .get("name")?
        .as_str()
        .map(ToOwned::to_owned)
}

fn load_workspace_yaml(workspace_root: &Path) -> Option<Value> {
    let path = workspace_root.join("nfw.yaml");
    let content = fs::read_to_string(path).ok()?;
    serde_yaml::from_str::<Value>(&content).ok()
}

fn stable_project_guid(service_name: &str, template_id: &str) -> String {
    let mut state_a: u64 = 0xcbf29ce484222325;
    let mut state_b: u64 = 0x8422_2325_cbf2_9ce4;
    for byte in service_name.bytes().chain(template_id.bytes()) {
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
