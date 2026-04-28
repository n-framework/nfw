use super::*;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_exit_code_standard() {
        let error_str = "[exit:1] standard error message";
        let cli_error = error_str.parse_exit_code();
        assert_eq!(cli_error.exit_code, 1);
        assert_eq!(cli_error.message, "standard error message");
        assert!(!cli_error.is_silent);
    }

    #[test]
    fn test_parse_exit_code_silent() {
        let error_str = "[exit:1:silent] silent error message";
        let cli_error = error_str.parse_exit_code();
        assert_eq!(cli_error.exit_code, 1);
        assert_eq!(cli_error.message, "silent error message");
        assert!(cli_error.is_silent);
    }

    #[test]
    fn test_parse_exit_code_internal_fallback() {
        let error_str = "plain error message without protocol";
        let cli_error = error_str.parse_exit_code();
        assert_eq!(cli_error.exit_code, 1);
        assert_eq!(cli_error.message, "plain error message without protocol");
        assert!(!cli_error.is_silent);
    }

    #[test]
    fn test_parse_exit_code_invalid_format_fallback() {
        let error_str = "[exit:abc] invalid code";
        let cli_error = error_str.parse_exit_code();
        assert_eq!(cli_error.exit_code, 1);
        assert_eq!(cli_error.message, "[exit:abc] invalid code");
        assert!(!cli_error.is_silent);
    }
}
