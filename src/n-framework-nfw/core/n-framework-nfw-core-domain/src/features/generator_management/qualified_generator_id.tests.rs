use super::*;

#[test]
fn creates_qualified_identifier() {
    let identifier = QualifiedGeneratorId::new("official".to_owned(), "web-api".to_owned());

    assert!(identifier.is_qualified());
    assert_eq!(identifier.source, Some("official".to_owned()));
    assert_eq!(identifier.generator, "web-api");
}

#[test]
fn creates_unqualified_identifier() {
    let identifier = QualifiedGeneratorId::unqualified("web-api".to_owned());

    assert!(!identifier.is_qualified());
    assert_eq!(identifier.source, None);
    assert_eq!(identifier.generator, "web-api");
}

#[test]
fn parses_identifier_text() {
    let qualified =
        QualifiedGeneratorId::parse("official/web-api").expect("qualified identifier should parse");
    assert!(qualified.is_qualified());

    let unqualified =
        QualifiedGeneratorId::parse("web-api").expect("unqualified identifier should parse");
    assert!(!unqualified.is_qualified());

    assert!(QualifiedGeneratorId::parse("official/").is_none());
    assert!(QualifiedGeneratorId::parse("").is_none());
}
