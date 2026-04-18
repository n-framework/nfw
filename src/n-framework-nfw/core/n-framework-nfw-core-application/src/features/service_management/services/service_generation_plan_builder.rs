use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
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

        let workspace_namespace = read_workspace_namespace(workspace_root).map_err(|e| {
            AddServiceError::InvalidWorkspaceContext(format!(
                "failed to read workspace namespace from nfw.yaml: {e}"
            ))
        })?;
        let workspace_name = read_workspace_name(workspace_root).map_err(|e| {
            AddServiceError::InvalidWorkspaceContext(format!(
                "failed to read workspace name from nfw.yaml: {e}"
            ))
        })?;
        let namespace = format!("{workspace_namespace}.{service_name}");
        let qualified_template_id = template_resolution.qualified_template_id();

        let mut parameters = TemplateParameters::new()
            .with_name(service_name)
            .with_namespace(&namespace);

        // Standard parameters
        parameters
            .insert("WorkspaceName", &workspace_name)
            .expect("valid key");
        parameters
            .insert("ServiceName", service_name)
            .expect("valid key");
        parameters
            .insert(
                "ProjectGuid",
                stable_project_guid(service_name, &qualified_template_id),
            )
            .expect("valid key");

        Ok(ServiceGenerationPlan {
            service_name: service_name.to_owned(),
            output_root,
            template_cache_path: template_resolution.template_cache_path.clone(),
            template_id: qualified_template_id,
            template_version: template_resolution.resolved_version.clone(),
            namespace,
            placeholder_values: parameters,
        })
    }
}

fn read_workspace_namespace(workspace_root: &Path) -> Result<String, String> {
    let yaml = load_workspace_yaml(workspace_root)?;
    yaml.get("workspace")
        .and_then(|w| w.get("namespace"))
        .and_then(|n| n.as_str())
        .map(ToOwned::to_owned)
        .ok_or_else(|| "missing 'workspace.namespace' in nfw.yaml".to_string())
}

fn read_workspace_name(workspace_root: &Path) -> Result<String, String> {
    let yaml = load_workspace_yaml(workspace_root)?;
    yaml.get("workspace")
        .and_then(|w| w.get("name"))
        .and_then(|n| n.as_str())
        .map(ToOwned::to_owned)
        .ok_or_else(|| "missing 'workspace.name' in nfw.yaml".to_string())
}

fn load_workspace_yaml(workspace_root: &Path) -> Result<Value, String> {
    let path = workspace_root.join("nfw.yaml");
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("failed to read {}: {}", path.display(), e))?;
    serde_yaml::from_str::<Value>(&content).map_err(|e| format!("failed to parse nfw.yaml: {}", e))
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
