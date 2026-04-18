#[path = "../service_add/support.rs"]
mod support;

use std::fs;

#[test]
fn generate_command_creates_files_from_template() {
    let workspace_root = support::create_sandbox_directory("generate-smoke");
    
    // Create nfw.yaml with template mapping
    fs::write(
        workspace_root.join("nfw.yaml"),
        "workspace:\n  name: Test\n  namespace: TestApp\ntemplate_sources:\n  local: \"templates\"\ntemplates:\n  command: mock-cmd-template\n",
    )
    .expect("failed to write nfw.yaml");

    // Create templates directory
    let templates_path = workspace_root.join("templates");
    let cmd_template_path = templates_path.join("mock-cmd-template");
    fs::create_dir_all(&cmd_template_path).expect("failed to create template dir");

    // Create template.yaml
    fs::write(
        cmd_template_path.join("template.yaml"),
        "id: mock-cmd-template\nsteps:\n  - action: render\n    source: cmd.rs.tera\n    destination: src/Commands/{{Name}}Command.rs\n",
    ).expect("failed to write template.yaml");

    // Create template source file
    fs::write(
        cmd_template_path.join("cmd.rs.tera"),
        "// Generated {{Name}} command in namespace {{Namespace}}\n// Param: {{MyParam}}\npub struct {{Name}}Command;\n",
    ).expect("failed to write template source");

    // We need to run nfw generate. 
    // Since we are in integration tests, we can use the nfw binary if it was built.
    // Or we can use nfw_cli_runtime directly if we mock everything.
    
    // Actually, let's use the command line if available.
    // But it's easier to use the presentation layer directly if we can.
    
    use n_framework_nfw_cli::runtime::nfw_cli_runtime::handle_gen_command;
    use n_framework_nfw_cli::startup::cli_service_collection_factory::CliServiceCollectionFactory;
    use n_framework_core_cli_abstractions::Command;
    use std::collections::HashMap;

    struct TestCommand {
        options: HashMap<String, String>,
    }

    impl Command for TestCommand {
        fn name(&self) -> &str { "command" }
        fn args(&self) -> &[String] { &[] }
        fn option(&self, name: &str) -> Option<&str> { self.options.get(name).map(|s| s.as_str()) }
    }
    
    // Manually setup service collection (simulating startup)
    let services = CliServiceCollectionFactory::create();
    
    // Override the working directory provider to our sandbox
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(&workspace_root).unwrap();

    let mut options = HashMap::new();
    options.insert("name".to_string(), "CreateUser".to_string());
    options.insert("param".to_string(), "MyParam=Value123".to_string());
    
    let command = TestCommand { options };

    let result = handle_gen_command(&command, &services);
    
    // Revert dir
    std::env::set_current_dir(original_dir).unwrap();

    assert!(result.is_ok(), "Generation failed: {:?}", result.err());

    // Verify file exists
    let generated_file = workspace_root.join("src/Commands/CreateUserCommand.rs");
    assert!(generated_file.exists(), "Generated file not found at {:?}", generated_file);

    let content = fs::read_to_string(generated_file).unwrap();
    assert!(content.contains("Generated CreateUser command"));
    assert!(content.contains("namespace TestApp"));
    assert!(content.contains("Param: Value123"));

    support::cleanup_sandbox_directory(&workspace_root);
}
