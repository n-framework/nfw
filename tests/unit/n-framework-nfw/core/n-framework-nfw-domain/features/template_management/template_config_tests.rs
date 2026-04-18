use n_framework_nfw_core_domain::features::template_management::errors::TemplateConfigError;
use n_framework_nfw_core_domain::features::template_management::template_config::{
    TemplateConfig, TemplateStep,
};

#[test]
fn template_config_validate_succeeds_on_empty_steps() {
    let config = TemplateConfig::new(None, vec![], vec![]);
    // Empty steps are allowed for legacy template support
    assert!(config.is_ok());
}

#[test]
fn template_config_validate_fails_on_invalid_step() {
    let config = TemplateConfig::new(
        None,
        vec![TemplateStep::Render {
            source: "".to_string(), // Invalid: empty source
            destination: "dest".to_string(),
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
        vec![TemplateStep::Render {
            source: "src".to_string(),
            destination: "dest".to_string(),
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
    let config = TemplateConfig::new(Some("official/dotnet-service".to_string()), vec![], vec![]);
    assert!(config.is_ok());
}
