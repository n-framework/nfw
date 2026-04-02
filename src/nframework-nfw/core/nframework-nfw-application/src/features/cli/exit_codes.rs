#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitCodes {
    Success = 0,
    ValidationError = 2,
    NotFound = 3,
    Conflict = 4,
    ExternalDependencyFailure = 10,
    InternalError = 1,
}
