# Data Model: `nfw check` Architecture Validation

## Entity: Architecture Rule Set

- Purpose: Canonical policy definition of allowed and forbidden dependencies across architecture layers.
- Fields:
  - `rule_id` (string, unique): Stable identifier for each rule.
  - `rule_type` (enum): `project_reference | namespace_usage | package_usage`.
  - `source_layer` (string): Layer where rule applies.
  - `target_pattern` (string): Forbidden target reference/namespace/package pattern.
  - `message_template` (string): Base remediation guidance.
  - `severity` (enum): `error` for initial scope.
- Validation rules:
  - `rule_id` must be unique.
  - `rule_type` must be one of the supported violation classes.
  - `target_pattern` must be non-empty.

## Entity: Validation Finding

- Purpose: One concrete violation (or unreadable-artifact/lint/test error) produced by a check run.
- Fields:
  - `finding_id` (string, unique within run): Deterministic key per location+rule.
  - `finding_type` (enum): `project_reference | namespace_usage | package_usage | unreadable_artifact | lint_issue | test_issue`.
  - `location` (string): Project path or source file path.
  - `offending_value` (string): Forbidden reference/namespace/package, or unreadable path context.
  - `violated_rule_id` (string, optional for unreadable): Rule identifier if applicable.
  - `remediation_hint` (string): Actionable fix text.
- Validation rules:
  - `location` and `remediation_hint` are required.
  - Duplicate `finding_id` values are not allowed in one run.

## Entity: Validation Summary

- Purpose: Aggregate result for command output and exit behavior.
- Fields:
  - `total_findings` (integer >= 0)
  - `project_reference_count` (integer >= 0)
  - `namespace_usage_count` (integer >= 0)
  - `package_usage_count` (integer >= 0)
  - `unreadable_artifact_count` (integer >= 0)
  - `lint_issue_count` (integer >= 0)
  - `test_issue_count` (integer >= 0)
  - `exit_outcome` (enum): `success | violation_found | execution_interrupted`.
- State transitions:
  - `success` when all counts are zero.
  - `violation_found` when any findings exist.
  - `execution_interrupted` when run is interrupted.

## Entity: Architecture Fixture Workspace

- Purpose: Deterministic workspace input used by tests to prove valid/invalid detection behavior.
- Fields:
  - `fixture_id` (string, unique)
  - `fixture_kind` (enum): `valid | invalid_project_reference | invalid_namespace_usage | invalid_package_usage | invalid_unreadable_artifact`.
  - `expected_exit_non_zero` (boolean)
  - `expected_finding_types` (set of enums)
- Validation rules:
  - `valid` fixtures require empty `expected_finding_types`.
  - Invalid fixtures require at least one expected finding type.

## Relationships

- Architecture Rule Set (1..*) -> (0..*) Validation Finding via `violated_rule_id`.
- Architecture Fixture Workspace (1) -> (0..*) Validation Finding as expected outcomes.
- Validation Summary aggregates counts from Validation Finding.
