pub trait Validator {
    fn is_kebab_case(&self, value: &str) -> bool;
    fn is_git_url(&self, value: &str) -> bool;
}
