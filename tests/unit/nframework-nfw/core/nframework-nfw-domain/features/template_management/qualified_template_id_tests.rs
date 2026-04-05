use nframework_nfw_core_domain::features::template_management::qualified_template_id::QualifiedTemplateId;

#[test]
fn creates_qualified_identifier() {
    let identifier = QualifiedTemplateId::new("official".to_owned(), "web-api".to_owned());

    assert!(identifier.is_qualified());
    assert_eq!(identifier.source, Some("official".to_owned()));
    assert_eq!(identifier.template, "web-api");
}

#[test]
fn creates_unqualified_identifier() {
    let identifier = QualifiedTemplateId::unqualified("web-api".to_owned());

    assert!(!identifier.is_qualified());
    assert_eq!(identifier.source, None);
    assert_eq!(identifier.template, "web-api");
}

#[test]
fn parses_identifier_text() {
    let qualified =
        QualifiedTemplateId::parse("official/web-api").expect("qualified identifier should parse");
    assert!(qualified.is_qualified());

    let unqualified =
        QualifiedTemplateId::parse("web-api").expect("unqualified identifier should parse");
    assert!(!unqualified.is_qualified());

    assert!(QualifiedTemplateId::parse("official/").is_none());
    assert!(QualifiedTemplateId::parse("").is_none());
}
