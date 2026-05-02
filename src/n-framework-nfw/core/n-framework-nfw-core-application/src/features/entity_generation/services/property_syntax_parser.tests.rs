use super::*;

#[test]
fn parses_single_property() {
    let result = PropertySyntaxParser::parse("Name:string").unwrap();
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].name(), "Name");
    assert_eq!(result[0].cli_type(), "string");
    assert!(!result[0].nullable());
}

#[test]
fn parses_nullable_property() {
    let result = PropertySyntaxParser::parse("Email:string?").unwrap();
    assert_eq!(result.len(), 1);
    assert!(result[0].nullable());
}

#[test]
fn parses_multiple_properties() {
    let result = PropertySyntaxParser::parse("Name:string,Price:decimal,Active:bool").unwrap();
    assert_eq!(result.len(), 3);
    assert_eq!(result[0].name(), "Name");
    assert_eq!(result[1].name(), "Price");
    assert_eq!(result[2].name(), "Active");
}

#[test]
fn rejects_empty_input() {
    let result = PropertySyntaxParser::parse("");
    assert!(matches!(
        result,
        Err(EntityGenerationError::EmptyProperties)
    ));
}

#[test]
fn rejects_whitespace_only() {
    let result = PropertySyntaxParser::parse("   ");
    assert!(matches!(
        result,
        Err(EntityGenerationError::EmptyProperties)
    ));
}

#[test]
fn rejects_missing_colon() {
    let result = PropertySyntaxParser::parse("NameString");
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidPropertySyntax { .. })
    ));
}

#[test]
fn rejects_missing_type() {
    let result = PropertySyntaxParser::parse("Name:");
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidPropertySyntax { .. })
    ));
}

#[test]
fn rejects_missing_name() {
    let result = PropertySyntaxParser::parse(":string");
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidPropertySyntax { .. })
    ));
}

#[test]
fn rejects_unsupported_type() {
    let result = PropertySyntaxParser::parse("Items:List<int>");
    assert!(matches!(
        result,
        Err(EntityGenerationError::UnsupportedPropertyType { .. })
    ));
}

#[test]
fn rejects_duplicate_property_names() {
    let result = PropertySyntaxParser::parse("Name:string,Name:int");
    assert!(matches!(
        result,
        Err(EntityGenerationError::DuplicatePropertyName { .. })
    ));
}

#[test]
fn rejects_case_insensitive_duplicates() {
    let result = PropertySyntaxParser::parse("ProductPrice:string,Productprice:int");
    assert!(matches!(
        result,
        Err(EntityGenerationError::DuplicatePropertyName { .. })
    ));
}

#[test]
fn trims_whitespace_around_properties() {
    let result = PropertySyntaxParser::parse(" Name : string , Price : decimal ").unwrap();
    assert_eq!(result.len(), 2);
    assert_eq!(result[0].name(), "Name");
    assert_eq!(result[1].name(), "Price");
}

#[test]
fn rejects_non_pascal_case_property_name() {
    let result = PropertySyntaxParser::parse("name:string");
    assert!(matches!(
        result,
        Err(EntityGenerationError::InvalidEntityName { .. })
    ));
}
