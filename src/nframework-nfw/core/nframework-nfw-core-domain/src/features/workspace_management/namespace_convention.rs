#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceConvention {
    pub workspace_base_namespace: String,
    pub service_suffix: String,
    pub layer_suffix: String,
}

impl NamespaceConvention {
    pub fn from_workspace_name(workspace_name: &str) -> Self {
        let workspace_base_namespace = workspace_name
            .split(['-', '_'])
            .filter(|segment| !segment.trim().is_empty())
            .map(to_pascal_case)
            .collect::<String>();

        Self {
            workspace_base_namespace,
            service_suffix: "Service".to_owned(),
            layer_suffix: "Application".to_owned(),
        }
    }

    pub fn service_namespace(&self, service_name: &str) -> String {
        format!(
            "{}.{}.{}",
            self.workspace_base_namespace,
            to_pascal_case(service_name),
            self.service_suffix
        )
    }
}

fn to_pascal_case(value: &str) -> String {
    value
        .split(['-', '_', ' '])
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => format!("{}{}", first.to_ascii_uppercase(), chars.as_str()),
                None => String::new(),
            }
        })
        .collect::<String>()
}
