# Research: `nfw check` Architecture Validation

## Decision 1: Validate three violation classes in one command
- Decision: `nfw check` validates forbidden project references, forbidden namespace usage, and forbidden package usage in one run.
- Rationale: The feature scope explicitly requires all three classes and a single CI-friendly audit command.
- Alternatives considered:
  - Split into separate commands per rule type: rejected due to fragmented CI workflow and weaker developer feedback loop.
  - Validate only project references first: rejected because spec requires namespace and package checks as well.

## Decision 2: Report all findings, then fail once
- Decision: The command reports all detected violations in one execution and exits non-zero when any finding exists.
- Rationale: This reduces rework by letting teams fix all issues in one iteration and matches clarified behavior.
- Alternatives considered:
  - Fail fast on first violation: rejected because it slows remediation and obscures full impact.

## Decision 3: Unreadable artifacts are validation failures, not warnings
- Decision: Unreadable project/source artifacts are emitted as validation errors; scan continues for readable artifacts; final exit is non-zero.
- Rationale: Prevents false green checks in CI while still maximizing actionable output from the run.
- Alternatives considered:
  - Ignore unreadable artifacts: rejected due to risk of hidden violations.
  - Abort immediately on unreadable artifact: rejected due to reduced feedback quality.

## Decision 4: Package validation scope is direct dependencies only
- Decision: Forbidden package checks target direct project-declared package references only.
- Rationale: Direct dependencies are under team control and produce deterministic, actionable results with fewer false positives.
- Alternatives considered:
  - Include all transitive dependencies: rejected due to noisy findings and remediation outside immediate project control.
  - Hybrid/transitive attribution model: deferred as potential enhancement after baseline delivery.

## Decision 5: Fixture-first validation strategy
- Decision: Architecture validation is proven with fixture workspaces containing valid and invalid cases for each violation class.
- Rationale: Fixtures provide deterministic, repeatable acceptance coverage aligned with spec success criteria.
- Alternatives considered:
  - Only unit tests with mocks: rejected because fixture-based end-to-end behavior is explicitly required.
