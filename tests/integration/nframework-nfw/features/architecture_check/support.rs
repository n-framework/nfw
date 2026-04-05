use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct ProjectConfig<'a> {
    pub workspace_root: &'a Path,
    pub relative_project_dir: &'a str,
    pub project_name: &'a str,
    pub project_references: &'a [&'a str],
    pub package_references: &'a [&'a str],
    pub source_files: &'a [(&'a str, &'a str)],
}

pub fn create_workspace(test_name: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be monotonic")
        .as_nanos();

    let root = std::env::temp_dir().join(format!("nfw-architecture-check-{test_name}-{unique}"));
    fs::create_dir_all(&root).expect("workspace directory should be created");
    fs::write(root.join("nfw.yaml"), "workspace:\n  name: TestWorkspace\n")
        .expect("workspace metadata should be written");
    fs::write(
        root.join("Makefile"),
        ".PHONY: lint test\nlint:\n\t@echo \"lint ok\"\n\ntest:\n\t@echo \"test ok\"\n",
    )
    .expect("workspace Makefile should be written");
    root
}

pub fn add_project(config: ProjectConfig) -> PathBuf {
    let project_dir = config.workspace_root.join(config.relative_project_dir);
    fs::create_dir_all(&project_dir).expect("project directory should be created");

    let project_file = project_dir.join(format!("{}.csproj", config.project_name));
    let project_references_xml = config
        .project_references
        .iter()
        .map(|value| format!(r#"<ProjectReference Include="{value}" />"#))
        .collect::<Vec<_>>()
        .join("\n    ");
    let package_references_xml = config
        .package_references
        .iter()
        .map(|value| format!(r#"<PackageReference Include="{value}" Version="1.0.0" />"#))
        .collect::<Vec<_>>()
        .join("\n    ");

    let csproj = format!(
        "<Project Sdk=\"Microsoft.NET.Sdk\">
	  <ItemGroup>
	    {project_references_xml}
	    {package_references_xml}
	  </ItemGroup>
	</Project>
"
    );
    fs::write(&project_file, csproj).expect("project file should be written");

    for (relative_source_path, content) in config.source_files {
        let source_path = project_dir.join(relative_source_path);
        if let Some(parent) = source_path.parent() {
            fs::create_dir_all(parent).expect("source directory should be created");
        }
        fs::write(source_path, content).expect("source file should be written");
    }

    project_file
}

pub fn run_nfw_check(workspace_root: &Path) -> Output {
    let sandbox_home = workspace_root.join(".home");
    let sandbox_config = sandbox_home.join(".config");
    fs::create_dir_all(&sandbox_config).expect("sandbox config directory should be created");

    Command::new(env!("CARGO_BIN_EXE_nframework-nfw-cli"))
        .arg("check")
        .current_dir(workspace_root)
        .env("HOME", &sandbox_home)
        .env("XDG_CONFIG_HOME", &sandbox_config)
        .output()
        .expect("nfw check should execute")
}

pub fn cleanup_workspace(workspace_root: &Path) {
    let _ = fs::remove_dir_all(workspace_root);
}
