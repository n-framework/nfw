use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use n_framework_core_template_abstractions::{FileGenerator, TemplateContext};
use n_framework_core_template_mustache::{MustacheFileGenerator, MustacheTemplateRenderer};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct DirectoryEntrySnapshot {
    relative_path: PathBuf,
    content: Option<String>,
}

#[test]
fn generating_twice_from_same_template_produces_identical_structure() {
    let sandbox = create_sandbox_directory();
    let template_root = sandbox.join("template");
    let output_a = sandbox.join("output-a");
    let output_b = sandbox.join("output-b");

    create_template(&template_root);

    let mut values: BTreeMap<String, Value> = BTreeMap::new();
    values.insert(
        "ServiceName".to_owned(),
        Value::String("BillingService".to_owned()),
    );
    values.insert(
        "Namespace".to_owned(),
        Value::String("Acme.Billing".to_owned()),
    );
    let context = TemplateContext::new(values);

    let generator = MustacheFileGenerator::new(MustacheTemplateRenderer::new());
    generator
        .generate(&template_root, &output_a, &context)
        .expect("first generation should succeed");
    generator
        .generate(&template_root, &output_b, &context)
        .expect("second generation should succeed");

    let snapshot_a = snapshot_directory(&output_a, &output_a);
    let snapshot_b = snapshot_directory(&output_b, &output_b);

    assert_eq!(snapshot_a, snapshot_b);
}

fn create_template(template_root: &Path) {
    let source_directory = template_root.join("{{ServiceName}}");
    fs::create_dir_all(&source_directory).expect("template directory should be created");

    fs::write(
        source_directory.join("README.md"),
        "# {{ServiceName}}\nnamespace: {{Namespace}}\n",
    )
    .expect("template readme should be written");
    fs::write(
        source_directory.join("{{ServiceName}}.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"></Project>\n",
    )
    .expect("template project file should be written");
}

fn snapshot_directory(root: &Path, current: &Path) -> Vec<DirectoryEntrySnapshot> {
    let mut snapshots = Vec::new();
    let entries = fs::read_dir(current).expect("directory should be readable");

    for entry in entries {
        let path = entry.expect("directory entry should be valid").path();
        let relative_path = path
            .strip_prefix(root)
            .expect("relative path should resolve")
            .to_path_buf();

        if path.is_dir() {
            snapshots.push(DirectoryEntrySnapshot {
                relative_path: relative_path.clone(),
                content: None,
            });
            snapshots.extend(snapshot_directory(root, &path));
            continue;
        }

        let content = fs::read_to_string(&path).expect("file content should be readable");
        snapshots.push(DirectoryEntrySnapshot {
            relative_path,
            content: Some(content),
        });
    }

    snapshots.sort();
    snapshots
}

fn create_sandbox_directory() -> PathBuf {
    let unix_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be valid")
        .as_nanos();
    let path =
        std::env::temp_dir().join(format!("nfw-phase8-reproducibility-test-{unix_timestamp}"));
    fs::create_dir_all(&path).expect("sandbox directory should be created");
    path
}
