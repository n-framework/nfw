mod support;

#[cfg(unix)]
mod unix_only {
    use std::fs;
    use std::os::unix::fs::PermissionsExt;

    use super::support::{
        ProjectConfig, add_project, cleanup_workspace, create_workspace, run_nfw_check,
    };

    #[test]
    fn check_reports_unreadable_artifact_and_fails() {
        let workspace_root = create_workspace("unreadable");
        add_project(ProjectConfig {
            workspace_root: &workspace_root,
            relative_project_dir: "src/NFramework.Domain",
            project_name: "NFramework.Domain",
            project_references: &[],
            package_references: &[],
            source_files: &[(
                "DomainModel.cs",
                "namespace NFramework.Domain;\npublic class DomainModel {}\n",
            )],
        });

        let unreadable_file = workspace_root.join("src/NFramework.Domain/DomainModel.cs");
        let mut permissions = fs::metadata(&unreadable_file)
            .expect("source file metadata should exist")
            .permissions();
        permissions.set_mode(0o000);
        fs::set_permissions(&unreadable_file, permissions).expect("file should become unreadable");

        let output = run_nfw_check(&workspace_root);
        let stderr = String::from_utf8_lossy(&output.stderr);

        assert!(!output.status.success());
        assert!(stderr.contains("type=unreadable_artifact"));

        let mut restore_permissions = fs::metadata(&unreadable_file)
            .expect("source file metadata should still exist")
            .permissions();
        restore_permissions.set_mode(0o644);
        let _ = fs::set_permissions(&unreadable_file, restore_permissions);

        cleanup_workspace(&workspace_root);
    }
}
