use super::*;

#[test]
fn test_valid_kebab_case() {
    assert!(is_kebab_case("valid"));
    assert!(is_kebab_case("my-template"));
    assert!(is_kebab_case("my-template-123"));
    assert!(is_kebab_case("a"));
    assert!(is_kebab_case("abc-123-def"));
}

#[test]
fn test_invalid_starting_hyphen() {
    assert!(!is_kebab_case("-invalid"));
    assert!(!is_kebab_case("--invalid"));
}

#[test]
fn test_invalid_ending_hyphen() {
    assert!(!is_kebab_case("invalid-"));
    assert!(!is_kebab_case("invalid--"));
}

#[test]
fn test_invalid_consecutive_hyphens() {
    assert!(!is_kebab_case("in--valid"));
    assert!(!is_kebab_case("my---template"));
}

#[test]
fn test_invalid_uppercase() {
    assert!(!is_kebab_case("Invalid"));
    assert!(!is_kebab_case("my-Template"));
    assert!(!is_kebab_case("INVALID"));
}

#[test]
fn test_invalid_characters() {
    assert!(!is_kebab_case("invalid_name"));
    assert!(!is_kebab_case("invalid.name"));
    assert!(!is_kebab_case("invalid space"));
    assert!(!is_kebab_case("invalid@template"));
}

#[test]
fn test_empty_string() {
    // Empty string has valid placement (no hyphens to check)
    // but has no characters to validate
    assert!(has_valid_kebab_placement(""));
    assert!(has_only_valid_kebab_characters(""));
    // Overall, empty string is technically valid kebab-case
    // but should be validated separately for non-emptiness
    assert!(is_kebab_case(""));
}

#[test]
fn test_single_hyphen() {
    assert!(!is_kebab_case("-"));
}

#[test]
fn test_numbers_valid() {
    assert!(is_kebab_case("123"));
    assert!(is_kebab_case("template-123"));
    assert!(is_kebab_case("123-template"));
}
