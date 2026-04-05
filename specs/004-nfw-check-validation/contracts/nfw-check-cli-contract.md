# Contract: `nfw check` CLI

## Command Shape

- Command: `nfw check`
- Mode: Non-interactive only.
- Input scope: Current workspace rooted at NFramework workspace marker.

## Exit Contract

- `0`: No findings detected.
- Non-zero: One or more findings detected or required artifacts unreadable.
- `130` (Unix-like): Interrupted by SIGINT.

## Output Contract

### Success Output

- Prints a clear success validation message.
- Indicates that no forbidden references, namespaces, direct package violations, lint failures, or service test failures were found.

### Failure Output

For each finding, output MUST include:

- Finding type (`project_reference`, `namespace_usage`, `package_usage`, `unreadable_artifact`, `lint_issue`, or `test_issue`)
- Location (project/file path)
- Offending value (forbidden target or unreadable artifact context)
- Actionable remediation hint

### Aggregated Reporting

- Multiple findings are reported in one run.
- Duplicate findings for same location+rule are not repeated.

## Validation Scope Contract

- Project reference checks: evaluate workspace project dependency graph against forbidden rules.
- Namespace checks: evaluate source content for forbidden namespace usage.
- Package checks: evaluate direct package references declared by each project only.
- Unreadable artifacts: emit validation errors and continue scanning remaining readable artifacts.
- Lint checks: execute `make lint` in each service path declared at `services.*.path` in `nfw.yaml` and emit `lint_issue` finding when lint fails.
- Service test checks: execute `make test` in each service path declared at `services.*.path` in `nfw.yaml` and emit `test_issue` finding when tests fail.
