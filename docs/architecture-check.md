# Architecture Check (`nfw check`)

`nfw check` validates workspace architecture boundaries and fails with a non-zero exit code when violations are found.

## What It Validates

- Forbidden project references between architecture layers
- Forbidden namespace usage in source files
- Forbidden direct package references in project files
- Unreadable artifacts as validation errors
- Service lint health by executing `make lint` in each declared service path
- Service test health by executing `make test` in each declared service path

## Polyglot Scope

Current validation scans multiple project ecosystems:

- C#: `.csproj` + `.cs`
- Rust: `Cargo.toml` + `.rs`
- Node.js/TypeScript: `package.json` + `.js/.jsx/.ts/.tsx`

## Exit Behavior

- `0`: No findings
- non-zero: One or more findings or unreadable artifacts

## Output Shape

For each finding:

- `type`: `project_reference`, `namespace_usage`, `package_usage`, `unreadable_artifact`, `lint_issue`, or `test_issue`
- `location`: file path of the violating artifact
- `offending`: forbidden reference/namespace/package or unreadable artifact reason
- `hint`: actionable remediation guidance

## Usage

Run from a workspace root (or a child directory under it):

```bash
nfw check
```
