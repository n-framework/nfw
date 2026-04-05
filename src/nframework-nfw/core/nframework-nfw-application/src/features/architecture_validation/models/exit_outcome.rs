#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitOutcome {
    Success,
    ViolationFound,
    ExecutionInterrupted,
}
