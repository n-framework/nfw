use std::fs;
use std::path::Path;

use n_framework_nfw_cli::commands::generate::{GenerateCliCommand, GenerateRequest};
use n_framework_nfw_core_application::features::template_management::models::template_error::TemplateError;
use n_framework_nfw_core_application::features::template_management::services::template_engine::TemplateEngine;
use n_framework_nfw_core_domain::features::template_management::template_config::TemplateConfig;
use n_framework_nfw_core_domain::features::template_management::template_parameters::TemplateParameters;
use tempfile::TempDir;

#[derive(Clone)]
struct MockTemplateEngine {
    result: Result<(), TemplateError>,
}

impl MockTemplateEngine {
    fn success() -> Self {
        Self { result: Ok(()) }
    }

    fn failure(error: TemplateError) -> Self {
        Self { result: Err(error) }
    }
}

impl TemplateEngine for MockTemplateEngine {
    fn execute(
        &self,
        _config: &TemplateConfig,
        _template_root: &Path,
        _output_root: &Path,
        _parameters: &TemplateParameters,
    ) -> Result<(), TemplateError> {
        self.result.clone()
    }
}

fn create_sandbox() -> TempDir {
    tempfile::tempdir().unwrap()
}

#[test]
fn execute_fails_on_invalid_name() {
    let engine = MockTemplateEngine::success();
    let command = GenerateCliCommand::new(engine);

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "Invalid Name",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid name"));
}

#[test]
fn execute_fails_on_invalid_feature() {
    let engine = MockTemplateEngine::success();
    let command = GenerateCliCommand::new(engine);

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: Some("Invalid Feature!"),
        params: None,
    });

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid feature"));
}

#[test]
fn execute_fails_on_missing_nfw_yaml() {
    let sandbox = create_sandbox();
    let engine = MockTemplateEngine::success();
    let command = GenerateCliCommand::new(engine).with_base_directory(sandbox.path().to_path_buf());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("could not find nfw.yaml")
    );
}

#[test]
fn execute_fails_on_malformed_nfw_yaml() {
    let sandbox = create_sandbox();
    fs::write(sandbox.path().join("nfw.yaml"), "name: { invalid yaml").unwrap();

    let engine = MockTemplateEngine::success();
    let command = GenerateCliCommand::new(engine).with_base_directory(sandbox.path().to_path_buf());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("invalid nfw.yaml"));
}

#[test]
fn execute_fails_on_malformed_params() {
    let sandbox = create_sandbox();

    // Setup dummy nfw.yaml
    fs::write(
        sandbox.path().join("nfw.yaml"),
        "name: test\nnamespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    )
    .unwrap();

    // Setup dummy template
    let template_root = sandbox.path().join("templates").join("mock-cmd");
    fs::create_dir_all(&template_root).unwrap();
    fs::write(
        template_root.join("template.yaml"),
        "id: mock-cmd\nsteps:\n  - action: render\n    source: s\n    destination: d\n",
    )
    .unwrap();

    let engine = MockTemplateEngine::success();
    let command = GenerateCliCommand::new(engine).with_base_directory(sandbox.path().to_path_buf());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: Some("InvalidParamFormat"),
    });

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("invalid parameter format"),
        "Expected 'invalid parameter format' but got: {}",
        err
    );
}

#[test]
fn execute_fails_on_empty_name() {
    let engine = MockTemplateEngine::success();
    let command = GenerateCliCommand::new(engine);

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("name cannot be empty")
    );
}

#[test]
fn execute_fails_on_missing_namespace_in_nfw_yaml() {
    let sandbox = create_sandbox();
    // nfw.yaml has no 'namespace' key
    fs::write(
        sandbox.path().join("nfw.yaml"),
        "name: test\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    )
    .unwrap();

    let template_root = sandbox.path().join("templates").join("mock-cmd");
    fs::create_dir_all(&template_root).unwrap();
    fs::write(
        template_root.join("template.yaml"),
        "id: mock-cmd\nsteps:\n  - action: render\n    source: s\n    destination: d\n",
    )
    .unwrap();

    let engine = MockTemplateEngine::success();
    let command = GenerateCliCommand::new(engine).with_base_directory(sandbox.path().to_path_buf());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("missing 'namespace'"),
        "Unexpected error: {}",
        err
    );
}

#[test]
fn execute_fails_on_param_without_value() {
    let sandbox = create_sandbox();
    fs::write(
        sandbox.path().join("nfw.yaml"),
        "name: test\nnamespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    )
    .unwrap();

    let template_root = sandbox.path().join("templates").join("mock-cmd");
    fs::create_dir_all(&template_root).unwrap();
    fs::write(
        template_root.join("template.yaml"),
        "id: mock-cmd\nsteps:\n  - action: render\n    source: s\n    destination: d\n",
    )
    .unwrap();

    let engine = MockTemplateEngine::success();
    let command = GenerateCliCommand::new(engine).with_base_directory(sandbox.path().to_path_buf());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: Some("KeyOnly"),
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("invalid parameter format"),
        "Unexpected error: {}",
        err
    );
}

#[test]
fn execute_fails_on_engine_error() {
    let sandbox = create_sandbox();
    // Setup dummy nfw.yaml
    fs::write(
        sandbox.path().join("nfw.yaml"),
        "name: test\nnamespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    )
    .unwrap();

    // Setup dummy template
    let template_root = sandbox.path().join("templates").join("mock-cmd");
    fs::create_dir_all(&template_root).unwrap();
    fs::write(
        template_root.join("template.yaml"),
        "id: mock-cmd\nsteps:\n  - action: render\n    source: s\n    destination: d\n",
    )
    .unwrap();

    let engine = MockTemplateEngine::failure(TemplateError::TemplateRenderError {
        message: "simulated engine failure".to_string(),
        step_index: Some(0),
        template_id: Some("mock-cmd".to_string()),
        file_path: None,
        source: None,
    });
    let command = GenerateCliCommand::new(engine).with_base_directory(sandbox.path().to_path_buf());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("simulated engine failure"),
        "Expected engine failure message but got: {}",
        err
    );
}

#[test]
fn execute_fails_on_io_error() {
    let sandbox = create_sandbox();
    fs::write(
        sandbox.path().join("nfw.yaml"),
        "name: test\nnamespace: App\ntemplate_sources:\n  local: templates\ntemplates:\n  command: mock-cmd\n",
    )
    .unwrap();

    let template_root = sandbox.path().join("templates").join("mock-cmd");
    fs::create_dir_all(&template_root).unwrap();
    fs::write(
        template_root.join("template.yaml"),
        "id: mock-cmd\nsteps:\n  - action: render\n    source: s\n    destination: d\n",
    )
    .unwrap();

    let engine = MockTemplateEngine::failure(TemplateError::io(
        "simulated io failure",
        sandbox.path().join("some/path"),
    ));
    let command = GenerateCliCommand::new(engine).with_base_directory(sandbox.path().to_path_buf());

    let result = command.execute(GenerateRequest {
        generator_type: "command",
        name: "ValidName",
        feature: None,
        params: None,
    });

    assert!(result.is_err());
    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("simulated io failure"),
        "Expected io failure message but got: {}",
        err
    );
}
