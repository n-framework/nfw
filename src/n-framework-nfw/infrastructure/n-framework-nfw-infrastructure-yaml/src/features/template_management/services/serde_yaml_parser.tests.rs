use super::*;
use n_framework_nfw_core_domain::features::template_management::template_config::{
    InjectionTarget, TemplateConfig, TemplateStep,
};

#[test]
fn test_parse_mediator_template() {
    let yaml = r#"
id: dotnet-service/mediator
name: Mediator Module
description: Adds NFramework Mediator support via source-generated DI registration
version: 1.0.0
language: csharp
steps:
  - action: run_command
    command: 'dotnet add package Mediator.Abstractions --version 3.0.*'
    working_directory: 'src/core/{{ Service }}.Core.Application'
  - action: inject
    source: 'content/mediator_registration.cs.tera'
    destination: 'src/core/{{ Service }}.Core.Application/ApplicationServiceRegistration.Nfw.g.cs'
    injection_target:
      type: region
      value: module-registrations
"#;
    let parser = SerdeYamlParser::new();
    let result: Result<TemplateConfig, String> = parser.parse(yaml);
    assert!(
        result.is_ok(),
        "Failed to parse template: {:?}",
        result.err()
    );
    let config = result.unwrap();
    assert_eq!(config.steps().len(), 2);

    match &config.steps()[1] {
        TemplateStep::Inject {
            injection_target, ..
        } => {
            assert!(matches!(injection_target, InjectionTarget::Region(_)));
            if let InjectionTarget::Region(val) = injection_target {
                assert_eq!(val, "module-registrations");
            }
        }
        _ => panic!("Expected Inject step at index 1"),
    }
}
