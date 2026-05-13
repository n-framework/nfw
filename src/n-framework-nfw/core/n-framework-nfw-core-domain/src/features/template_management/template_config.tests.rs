use super::*;

#[test]
fn template_config_validate_fails_on_empty_steps() {
    let config = TemplateConfig::new(None, vec![], vec![]);
    assert!(config.is_err());
    assert!(matches!(
        config.unwrap_err(),
        TemplateConfigError::InvalidStep { .. }
    ));
}

#[test]
fn template_config_validate_fails_on_invalid_step() {
    let config = TemplateConfig::new(
        None,
        vec![TemplateStepConfig {
            condition: None,
            action: TemplateStepAction::Render {
                source: "".to_string(), // Invalid: empty source
                destination: "dest".to_string(),
            },
        }],
        vec![],
    );
    assert!(config.is_err());
    assert!(matches!(
        config.unwrap_err(),
        TemplateConfigError::InvalidStep { .. }
    ));
}

#[test]
fn template_config_validate_succeeds_with_valid_steps() {
    let config = TemplateConfig::new(
        None,
        vec![TemplateStepConfig {
            condition: None,
            action: TemplateStepAction::Render {
                source: "src".to_string(),
                destination: "dest".to_string(),
            },
        }],
        vec![],
    );
    assert!(config.is_ok());
}

#[test]
fn template_config_validate_fails_on_invalid_id() {
    let config = TemplateConfig::new(Some("invalid id with spaces".to_string()), vec![], vec![]);
    assert!(config.is_err());
    assert!(matches!(
        config.unwrap_err(),
        TemplateConfigError::InvalidFormat { .. }
    ));
}

#[test]
fn template_config_validate_succeeds_with_namespaced_id() {
    let config = TemplateConfig::new(
        Some("official/dotnet-service".to_string()),
        vec![TemplateStepConfig {
            condition: None,
            action: TemplateStepAction::RunCommand {
                command: "echo test".to_string(),
                working_directory: None,
            },
        }],
        vec![],
    );
    assert!(config.is_ok());
}

#[test]
fn template_config_validate_fails_on_empty_run_command() {
    let config = TemplateConfig::new(
        None,
        vec![TemplateStepConfig {
            condition: None,
            action: TemplateStepAction::RunCommand {
                command: "  ".to_string(),
                working_directory: None,
            },
        }],
        vec![],
    );
    assert!(config.is_err());
    assert!(matches!(
        config.unwrap_err(),
        TemplateConfigError::InvalidStep { .. }
    ));
}

#[test]
fn template_config_validate_succeeds_with_valid_run_command() {
    let config = TemplateConfig::new(
        None,
        vec![TemplateStepConfig {
            condition: None,
            action: TemplateStepAction::RunCommand {
                command: "dotnet add package Mediator".to_string(),
                working_directory: Some("src/core".to_string()),
            },
        }],
        vec![],
    );
    assert!(config.is_ok());
}

#[test]
fn template_config_parses_generators() {
    let json = r#"{
"id": "t1",
"steps": [
  {
    "action": "run_command",
    "command": "echo"
  }
],
"inputs": [],
"generators": {
  "persistence": "sub/persistence",
  "mediator": "sub/mediator"
}
}"#;
    let config: TemplateConfig = serde_json::from_str(json).expect("should parse generators");
    let generators = config.generators().expect("generators should be set");
    assert_eq!(generators.get("persistence").unwrap(), "sub/persistence");
    assert_eq!(generators.get("mediator").unwrap(), "sub/mediator");
}

#[test]
fn template_config_parses_step_conditions() {
    let json = r#"{
"id": "t1",
"steps": [
  {
    "action": "render",
    "source": "src",
    "destination": "dest",
    "if": "{{ not no_api }}"
  }
],
"inputs": []
}"#;
    let config: TemplateConfig = serde_json::from_str(json).expect("should parse condition");
    let steps = config.steps();
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].condition.as_deref(), Some("{{ not no_api }}"));
}
