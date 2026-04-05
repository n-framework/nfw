/// Shared validation utilities for template management.
///
/// This module provides reusable validation functions that are used
/// across multiple layers of the application.
/// Checks if a string follows kebab-case naming convention.
///
/// Kebab-case requires:
/// - Cannot start or end with a hyphen
/// - Cannot contain consecutive hyphens
/// - Can only contain lowercase letters, digits, and hyphens
///
/// # Arguments
/// * `value` - The string to validate
///
/// # Returns
/// `true` if the string is valid kebab-case, `false` otherwise
///
/// # Examples
/// ```
/// use nframework_nfw_core_domain::features::template_management::validation::is_kebab_case;
///
/// assert!(is_kebab_case("valid-name"));
/// assert!(is_kebab_case("my-template-123"));
/// assert!(!is_kebab_case("-invalid"));
/// assert!(!is_kebab_case("invalid-"));
/// assert!(!is_kebab_case("in--valid"));
/// assert!(!is_kebab_case("Invalid"));
/// assert!(!is_kebab_case("invalid_name"));
/// ```
pub fn is_kebab_case(value: &str) -> bool {
    has_valid_kebab_placement(value) && has_only_valid_kebab_characters(value)
}

/// Checks if a string has hyphens in valid positions.
///
/// Returns `false` if the string starts or ends with a hyphen,
/// or contains consecutive hyphens.
fn has_valid_kebab_placement(value: &str) -> bool {
    !value.starts_with('-') && !value.ends_with('-') && !value.contains("--")
}

/// Checks if a string contains only valid kebab-case characters.
///
/// Valid characters are lowercase letters, digits, and hyphens.
fn has_only_valid_kebab_characters(value: &str) -> bool {
    value
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
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
}
