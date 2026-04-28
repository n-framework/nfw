#[path = "../service_add/support.rs"]
mod support;

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Mutex;

use n_framework_core_cli_abstractions::Command;
use n_framework_nfw_cli::commands::r#gen::command::GenMediatorCommandCliCommand;
use n_framework_nfw_cli::startup::cli_service_collection_factory::CliServiceCollectionFactory;

// Tests in this file must run sequentially because they temporarily mutate
// `std::env::current_dir()`, which is shared global process state.
static DIR_LOCK: Mutex<()> = Mutex::new(());

struct TestCommand {
    opts: HashMap<String, String>,
}

impl Command for TestCommand {
    fn name(&self) -> &str {
        "command"
    }
    fn args(&self) -> &[String] {
        &[]
    }
    fn option(&self, name: &str) -> Option<&str> {
        self.opts.get(name).map(|s| s.as_str())
    }
}

fn make_opts(name: &str, param: Option<&str>, param_json: Option<&str>) -> HashMap<String, String> {
    let mut opts = HashMap::new();
    opts.insert("name".to_string(), name.to_string());
    opts.insert("no-input".to_string(), "true".to_string());
    if let Some(p) = param {
        opts.insert("param".to_string(), p.to_string());
    }
    if let Some(j) = param_json {
        opts.insert("param-json".to_string(), j.to_string());
    }
    opts
}

fn setup_template(sandbox: &Path, template_yaml: &str, tera_source: &str) {
    fs::write(
        sandbox.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\nservices:\n  TestService:\n    path: src/TestService\n    template:\n      id: mock-cmd-template\ntemplate_sources:\n  local: \"templates\"\n",
    ).expect("failed to write nfw.yaml");

    let tpl_dir = sandbox
        .join("templates")
        .join("mock-cmd-template")
        .join("command");
    fs::create_dir_all(&tpl_dir).expect("failed to create template dir");
    fs::write(tpl_dir.join("template.yaml"), template_yaml).expect("failed to write template.yaml");
    fs::write(tpl_dir.join("cmd.rs.tera"), tera_source).expect("failed to write tera source");
}

fn run(sandbox: &Path, opts: HashMap<String, String>) -> Result<(), String> {
    let _guard = DIR_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let services = CliServiceCollectionFactory::create();
    let original_dir = std::env::current_dir().expect("should have current dir");
    std::env::set_current_dir(sandbox).expect("should set current dir");
    let result = GenMediatorCommandCliCommand::handle(&TestCommand { opts }, &services);
    std::env::set_current_dir(&original_dir).expect("should restore current dir");
    result
}

// ---------------------------------------------------------------------------

#[test]
fn artifact_smoke_test_creates_files_from_template() {
    let sandbox = support::create_sandbox_directory("generate-smoke");

    setup_template(
        &sandbox,
        "id: mock-cmd-template\nsteps:\n  - action: render\n    source: cmd.rs.tera\n    destination: src/Commands/{{Name}}Command.rs\n",
        "// Generated {{Name}} command in namespace {{Namespace}}\n// Param: {{MyParam}}\npub struct {{Name}}Command;\n",
    );

    let result = run(
        &sandbox,
        make_opts("CreateUser", Some("MyParam=Value123"), None),
    );
    assert!(result.is_ok(), "Generation failed: {:?}", result.err());

    let generated_file = sandbox.join("src/TestService/src/Commands/CreateUserCommand.rs");
    assert!(
        generated_file.exists(),
        "Generated file not found at {:?}",
        generated_file
    );

    let content = fs::read_to_string(&generated_file).unwrap();
    assert!(content.contains("Generated CreateUser command"));
    assert!(content.contains("namespace TestApp"));
    assert!(content.contains("Param: Value123"));

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn artifact_handles_password_input_via_param_json() {
    let sandbox = support::create_sandbox_directory("gen-password");

    setup_template(
        &sandbox,
        "id: mock-cmd-template\ninputs:\n  - id: db-password\n    type: password\n    prompt: Database password\nsteps:\n  - action: render\n    source: cmd.rs.tera\n    destination: out.rs\n",
        "// secret set\n",
    );

    let result = run(
        &sandbox,
        make_opts("TestCmd", None, Some(r#"{"db-password":"s3cr3t"}"#)),
    );
    assert!(result.is_ok(), "Password input failed: {:?}", result.err());

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn artifact_handles_confirm_input_via_param_json() {
    let sandbox = support::create_sandbox_directory("gen-confirm");

    setup_template(
        &sandbox,
        "id: mock-cmd-template\ninputs:\n  - id: enable_logging\n    type: confirm\n    prompt: Enable logging?\n    default: false\nsteps:\n  - action: render\n    source: cmd.rs.tera\n    destination: out.rs\n",
        "// logging: {{ enable_logging }}\n",
    );

    let result = run(
        &sandbox,
        make_opts("TestCmd", None, Some(r#"{"enable_logging":true}"#)),
    );
    assert!(result.is_ok(), "Confirm input failed: {:?}", result.err());

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn artifact_handles_select_input_via_param_json() {
    let sandbox = support::create_sandbox_directory("gen-select");

    setup_template(
        &sandbox,
        "id: mock-cmd-template\ninputs:\n  - id: db_driver\n    type: select\n    prompt: Choose database driver\n    options:\n      - postgres\n      - mysql\n      - sqlite\nsteps:\n  - action: render\n    source: cmd.rs.tera\n    destination: out.rs\n",
        "// driver: {{ db_driver }}\n",
    );

    let result = run(
        &sandbox,
        make_opts("TestCmd", None, Some(r#"{"db_driver":"postgres"}"#)),
    );
    assert!(result.is_ok(), "Select input failed: {:?}", result.err());

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn artifact_handles_multiselect_input_via_param_json() {
    let sandbox = support::create_sandbox_directory("gen-multiselect");

    setup_template(
        &sandbox,
        "id: mock-cmd-template\ninputs:\n  - id: features\n    type: multiselect\n    prompt: Choose features\n    options:\n      - auth\n      - logging\n      - metrics\nsteps:\n  - action: render\n    source: cmd.rs.tera\n    destination: out.rs\n",
        "// features: {% for f in features %}{{ f }} {% endfor %}\n",
    );

    let result = run(
        &sandbox,
        make_opts("TestCmd", None, Some(r#"{"features":["auth","metrics"]}"#)),
    );
    assert!(
        result.is_ok(),
        "Multiselect input failed: {:?}",
        result.err()
    );

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn artifact_handles_object_input_via_param_json() {
    let sandbox = support::create_sandbox_directory("gen-object");

    setup_template(
        &sandbox,
        "id: mock-cmd-template\ninputs:\n  - id: author\n    type: object\n    prompt: Author info\n    properties:\n      - id: name\n        type: text\n        prompt: Author name\n      - id: email\n        type: text\n        prompt: Author email\nsteps:\n  - action: render\n    source: cmd.rs.tera\n    destination: out.rs\n",
        "// author: {{ author.name }} <{{ author.email }}>\n",
    );

    let result = run(
        &sandbox,
        make_opts(
            "TestCmd",
            None,
            Some(r#"{"author":{"name":"Alice","email":"alice@example.com"}}"#),
        ),
    );
    assert!(result.is_ok(), "Object input failed: {:?}", result.err());

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn artifact_handles_list_input_via_param_json() {
    let sandbox = support::create_sandbox_directory("gen-list");

    setup_template(
        &sandbox,
        "id: mock-cmd-template\ninputs:\n  - id: tags\n    type: list\n    prompt: Tags\n    items:\n      id: tag\n      type: text\n      prompt: Tag value\nsteps:\n  - action: render\n    source: cmd.rs.tera\n    destination: out.rs\n",
        "// tags: {% for t in tags %}{{ t }} {% endfor %}\n",
    );

    let result = run(
        &sandbox,
        make_opts("TestCmd", None, Some(r#"{"tags":["v1","latest"]}"#)),
    );
    assert!(result.is_ok(), "List input failed: {:?}", result.err());

    support::cleanup_sandbox_directory(&sandbox);
}

#[test]
fn artifact_fails_on_missing_required_param_in_no_input_mode() {
    let sandbox = support::create_sandbox_directory("gen-missing-param");

    setup_template(
        &sandbox,
        "id: mock-cmd-template\ninputs:\n  - id: required-input\n    type: text\n    prompt: Required value\nsteps:\n  - action: render\n    source: cmd.rs.tera\n    destination: out.rs\n",
        "// value: {{ required-input }}\n",
    );

    // Intentionally omit 'required-input' to trigger no-input failure
    let result = run(&sandbox, make_opts("TestCmd", None, None));
    assert!(
        result.is_err(),
        "Expected failure on missing required param"
    );

    support::cleanup_sandbox_directory(&sandbox);
}
