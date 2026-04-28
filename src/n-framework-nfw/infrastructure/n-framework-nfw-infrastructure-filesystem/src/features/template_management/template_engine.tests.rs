use super::*;
use std::fs;
use tempfile::TempDir;

fn create_sandbox() -> TempDir {
    tempfile::tempdir().unwrap()
}

#[test]
fn execute_render_step_creates_file() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let source_file = template_root.join("hello.txt.tera");
    fs::write(&source_file, "Hello, {{ Name }}!").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Render {
            source: "hello.txt.tera".to_string(),
            destination: "hello.txt".to_string(),
        }],
        vec![],
    )
    .unwrap();

    let mut parameters = TemplateParameters::new();
    let _ = parameters.insert("Name", "World");

    let engine = FileSystemTemplateEngine::new();
    engine
        .execute(&config, &template_root, &output_root, &parameters)
        .unwrap();

    let output_file = output_root.join("hello.txt");
    assert!(output_file.exists());
    assert_eq!(fs::read_to_string(output_file).unwrap(), "Hello, World!");
}

#[test]
fn prevents_path_traversal_in_render_destination() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let source_file = template_root.join("leak.txt");
    fs::write(&source_file, "secret").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Render {
            source: "leak.txt".to_string(),
            destination: "../outside.txt".to_string(),
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    let result = engine.execute(
        &config,
        &template_root,
        &output_root,
        &TemplateParameters::new(),
    );

    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("parent directory traversal"));
}

#[test]
fn prevents_absolute_path_traversal() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let source_file = template_root.join("leak.txt");
    fs::write(&source_file, "secret").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Render {
            source: "leak.txt".to_string(),
            destination: "/tmp/evil.txt".to_string(),
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    let result = engine.execute(
        &config,
        &template_root,
        &output_root,
        &TemplateParameters::new(),
    );

    assert!(result.is_err());
    assert!(format!("{}", result.unwrap_err()).contains("escaping output root"));
}

#[test]
fn inject_at_end_works() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let target_file = output_root.join("app.txt");
    fs::write(&target_file, "start").unwrap();

    let source_file = template_root.join("inject.txt");
    fs::write(&source_file, "content").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Inject {
            source: "inject.txt".to_string(),
            destination: "app.txt".to_string(),
            injection_target: InjectionTarget::AtEnd,
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    engine
        .execute(
            &config,
            &template_root,
            &output_root,
            &TemplateParameters::new(),
        )
        .unwrap();

    assert_eq!(fs::read_to_string(target_file).unwrap(), "start\ncontent");
}

#[test]
fn inject_region_works() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let target_file = output_root.join("app.txt");
    fs::write(
        &target_file,
        "head\n// <nfw:deps:start>\n// <nfw:deps:end>\nfoot",
    )
    .unwrap();

    let source_file = template_root.join("inject.txt");
    fs::write(&source_file, "new_dep\n").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Inject {
            source: "inject.txt".to_string(),
            destination: "app.txt".to_string(),
            injection_target: InjectionTarget::Region("deps".to_string()),
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    engine
        .execute(
            &config,
            &template_root,
            &output_root,
            &TemplateParameters::new(),
        )
        .unwrap();

    assert_eq!(
        fs::read_to_string(target_file).unwrap(),
        "head\n// <nfw:deps:start>\nnew_dep\n// <nfw:deps:end>\nfoot"
    );
}

#[test]
fn inject_region_fails_when_start_marker_is_missing() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let target_file = output_root.join("app.txt");
    fs::write(&target_file, "head\n// <nfw:deps:end>\nfoot").unwrap();

    let source_file = template_root.join("inject.txt");
    fs::write(&source_file, "new_dep\n").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Inject {
            source: "inject.txt".to_string(),
            destination: "app.txt".to_string(),
            injection_target: InjectionTarget::Region("deps".to_string()),
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    let result = engine.execute(
        &config,
        &template_root,
        &output_root,
        &TemplateParameters::new(),
    );

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("region start marker"));
    assert!(err.contains("not found"));
}

#[test]
fn inject_region_fails_when_end_marker_is_missing() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let target_file = output_root.join("app.txt");
    fs::write(&target_file, "head\n// <nfw:deps:start>\nfoot").unwrap();

    let source_file = template_root.join("inject.txt");
    fs::write(&source_file, "new_dep\n").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Inject {
            source: "inject.txt".to_string(),
            destination: "app.txt".to_string(),
            injection_target: InjectionTarget::Region("deps".to_string()),
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    let result = engine.execute(
        &config,
        &template_root,
        &output_root,
        &TemplateParameters::new(),
    );

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(err.contains("region end marker"));
    assert!(err.contains("not found"));
}

#[test]
fn inject_region_auto_indents_content() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let target_file = output_root.join("app.cs");
    fs::write(
        &target_file,
        "class Foo\n{\n    void Bar()\n    {\n        // <nfw:deps:start>\n        // <nfw:deps:end>\n    }\n}\n",
    )
    .unwrap();

    let source_file = template_root.join("inject.txt");
    fs::write(&source_file, "services.AddMediator();\n").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Inject {
            source: "inject.txt".to_string(),
            destination: "app.cs".to_string(),
            injection_target: InjectionTarget::Region("deps".to_string()),
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    engine
        .execute(
            &config,
            &template_root,
            &output_root,
            &TemplateParameters::new(),
        )
        .unwrap();

    assert_eq!(
        fs::read_to_string(target_file).unwrap(),
        "class Foo\n{\n    void Bar()\n    {\n        // <nfw:deps:start>\n        services.AddMediator();\n        // <nfw:deps:end>\n    }\n}\n"
    );
}

#[test]
fn render_folder_step_copies_and_renders_directory_tree() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    let source_dir = template_root.join("components");
    let nested_dir = source_dir.join("nested");
    fs::create_dir_all(&nested_dir).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    fs::write(source_dir.join("root.txt.tera"), "Root {{ Name }}").unwrap();
    fs::write(nested_dir.join("child.txt.tera"), "Child {{ Name }}").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::RenderFolder {
            source: "components".to_string(),
            destination: "out_components".to_string(),
        }],
        vec![],
    )
    .unwrap();

    let mut parameters = TemplateParameters::new();
    let _ = parameters.insert("Name", "World");

    let engine = FileSystemTemplateEngine::new();
    engine
        .execute(&config, &template_root, &output_root, &parameters)
        .unwrap();

    let out_dir = output_root.join("out_components");
    assert!(out_dir.join("root.txt").exists());
    assert!(out_dir.join("nested").join("child.txt").exists());
    assert_eq!(
        fs::read_to_string(out_dir.join("root.txt")).unwrap(),
        "Root World"
    );
}

#[test]
fn render_step_fails_when_source_file_missing() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Render {
            source: "does_not_exist.txt".to_string(),
            destination: "out.txt".to_string(),
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    let result = engine.execute(
        &config,
        &template_root,
        &output_root,
        &TemplateParameters::new(),
    );

    assert!(result.is_err());
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("failed to read template source"),
        "Unexpected error: {}",
        err
    );
}

#[test]
fn inject_fails_when_destination_file_missing() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let source_file = template_root.join("inject.txt");
    fs::write(&source_file, "content").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Inject {
            source: "inject.txt".to_string(),
            destination: "missing_target.txt".to_string(),
            injection_target: InjectionTarget::AtEnd,
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    let result = engine.execute(
        &config,
        &template_root,
        &output_root,
        &TemplateParameters::new(),
    );

    assert!(result.is_err());
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("failed to read target file for injection"),
        "Unexpected error: {}",
        err
    );
}

#[test]
fn execute_fails_on_parent_directory_creation_failure() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    // Create a file where a directory should be
    let obstacle = output_root.join("file_blocking_dir");
    fs::write(&obstacle, "I am a file, not a directory").unwrap();

    let source_file = template_root.join("hello.txt");
    fs::write(&source_file, "Hello").unwrap();

    // Destination requires creating a directory where 'obstacle' already is
    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Render {
            source: "hello.txt".to_string(),
            destination: "file_blocking_dir/nested/hello.txt".to_string(),
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    let result = engine.execute(
        &config,
        &template_root,
        &output_root,
        &TemplateParameters::new(),
    );

    assert!(result.is_err());
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("failed to create parent directory"),
        "Unexpected error: {}",
        err
    );
}

#[test]
fn execute_fails_on_template_syntax_error() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let source_file = template_root.join("bad.txt.tera");
    fs::write(&source_file, "Hello, {{ Unclosed bracket").unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Render {
            source: "bad.txt.tera".to_string(),
            destination: "out.txt".to_string(),
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    let result = engine.execute(
        &config,
        &template_root,
        &output_root,
        &TemplateParameters::new(),
    );

    assert!(result.is_err());
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("template rendering error") || err.contains("syntax error"),
        "Unexpected error: {}",
        err
    );
}

#[test]
fn run_command_step_executes_successfully() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::RunCommand {
            command: "echo hello > result.txt".to_string(),
            working_directory: None,
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    engine
        .execute(
            &config,
            &template_root,
            &output_root,
            &TemplateParameters::new(),
        )
        .unwrap();

    let result_file = output_root.join("result.txt");
    assert!(result_file.exists());
    assert_eq!(fs::read_to_string(result_file).unwrap().trim(), "hello");
}

#[test]
fn run_command_step_fails_on_nonzero_exit() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::RunCommand {
            command: "exit 1".to_string(),
            working_directory: None,
        }],
        vec![],
    )
    .unwrap();

    let engine = FileSystemTemplateEngine::new();
    let result = engine.execute(
        &config,
        &template_root,
        &output_root,
        &TemplateParameters::new(),
    );

    assert!(result.is_err());
    let err = format!("{}", result.unwrap_err());
    assert!(
        err.contains("command execution error"),
        "Unexpected error: {}",
        err
    );
}

#[test]
fn run_command_step_renders_tera_placeholders() {
    let sandbox = create_sandbox();
    let template_root = sandbox.path().join("templates");
    let output_root = sandbox.path().join("output");
    fs::create_dir_all(&template_root).unwrap();
    fs::create_dir_all(&output_root).unwrap();

    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::RunCommand {
            command: "echo {{ Name }} > greeting.txt".to_string(),
            working_directory: None,
        }],
        vec![],
    )
    .unwrap();

    let mut parameters = TemplateParameters::new();
    let _ = parameters.insert("Name", "NFramework");

    let engine = FileSystemTemplateEngine::new();
    engine
        .execute(&config, &template_root, &output_root, &parameters)
        .unwrap();

    let result_file = output_root.join("greeting.txt");
    assert!(result_file.exists());
    assert_eq!(
        fs::read_to_string(result_file).unwrap().trim(),
        "NFramework"
    );
}
