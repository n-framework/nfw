use super::*;

#[test]
fn given_new_file_created_when_get_created_files_then_returns_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let tracker = FileTracker::new(temp_dir.path()).unwrap();

    assert!(tracker.get_created_files().is_empty());

    let new_file = temp_dir.path().join("new.txt");
    fs::write(&new_file, "content").unwrap();

    let created = tracker.get_created_files();
    assert_eq!(created.len(), 1);
    assert_eq!(created[0], new_file);
}

#[test]
fn given_new_file_created_when_cleanup_then_removes_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let tracker = FileTracker::new(temp_dir.path()).unwrap();

    let new_file = temp_dir.path().join("new.txt");
    fs::write(&new_file, "content").unwrap();

    tracker.cleanup_created_files().unwrap();
    assert!(!new_file.exists());
}

#[test]
fn given_yaml_backup_created_when_restore_then_returns_original_content() {
    let temp_dir = tempfile::tempdir().unwrap();
    let yaml_path = temp_dir.path().join("nfw.yaml");

    fs::write(&yaml_path, "original content").unwrap();

    let backup = YamlBackup::create(&yaml_path).unwrap();

    fs::write(&yaml_path, "modified content").unwrap();

    backup.restore().unwrap();

    let content = fs::read_to_string(&yaml_path).unwrap();
    assert_eq!(content, "original content");
}
