use super::*;

#[test]
fn builder_validates_mandatory_fields() {
    let builder = EntityGenerationParameters::builder();
    let result = builder.build();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "entity_name cannot be empty");

    let builder = EntityGenerationParameters::builder().entity_name("Product".to_string());
    let result = builder.build();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "namespace cannot be empty");

    let builder = EntityGenerationParameters::builder()
        .entity_name("Product".to_string())
        .namespace("MyNamespace".to_string())
        .service_name("MyService".to_string())
        .service_path(PathBuf::from("/path/to/service"));
    let result = builder.build();
    assert!(result.is_ok());
}
