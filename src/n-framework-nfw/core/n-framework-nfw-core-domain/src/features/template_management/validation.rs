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
/// use n_framework_nfw_core_domain::features::template_management::validation::is_kebab_case;
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
#[path = "validation.tests.rs"]
mod tests;
