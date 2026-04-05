# Feature Specification: `nfw check` Architecture Validation

## User Scenarios & Testing

### User Story 1 - Detect Architecture Violations in a Workspace (Priority: P1)

As a tech lead, I want to run `nfw check` and detect forbidden references, namespaces, and packages so that boundary violations are caught before merge or release.

**Why this priority**: Architecture enforcement is the core purpose of this feature. Without reliable violation detection, teams cannot trust the framework's boundary guarantees.

**Independent Test**: Run `nfw check` against a workspace fixture containing known violations and verify that the command exits non-zero and reports each violation with actionable remediation guidance.

**Acceptance Scenarios**:

1. **Given** a workspace containing a project reference that violates layer rules, **When** I run `nfw check`, **Then** the command reports the violating source project, target reference, and a concrete fix suggestion.
2. **Given** a workspace containing forbidden namespace usage, **When** I run `nfw check`, **Then** the command reports the violating file and forbidden namespace with remediation guidance.
3. **Given** a workspace containing forbidden package usage, **When** I run `nfw check`, **Then** the command reports the violating project and forbidden package with remediation guidance.

---

### User Story 2 - Support CI Gatekeeping with Deterministic Exit Outcomes (Priority: P1)

As a CI engineer, I want `nfw check` to run non-interactively with deterministic exit status so that pipelines can fail fast on architecture regressions.

**Why this priority**: CI enforcement prevents violations from reaching main branches. Deterministic pass/fail outcomes are required for automation reliability.

**Independent Test**: Run `nfw check` in a non-interactive test job for both valid and invalid fixtures and verify expected exit behavior in both cases.

**Acceptance Scenarios**:

1. **Given** a valid workspace fixture with no violations, **When** `nfw check` runs in non-interactive mode, **Then** it exits with status `0`.
2. **Given** an invalid workspace fixture with one or more violations, **When** `nfw check` runs in non-interactive mode, **Then** it exits with a non-zero status.
3. **Given** a workspace with multiple violations, **When** `nfw check` runs, **Then** all detected violations are reported in one command run before final exit.

---

### User Story 3 - Understand and Fix Violations Quickly (Priority: P2)

As a developer, I want clear, actionable output from `nfw check` so that I can correct architecture violations without guesswork.

**Why this priority**: Detection alone is insufficient if the output is vague. Developers must know what failed and what to change.

**Independent Test**: Trigger each violation class in fixtures and verify output includes location, violation rule, and a practical fix hint.

**Acceptance Scenarios**:

1. **Given** a violation is detected, **When** output is produced, **Then** each finding includes the violation type and affected artifact location.
2. **Given** a violation is detected, **When** output is produced, **Then** each finding includes a remediation hint aligned with documented layer rules.
3. **Given** no violations are detected, **When** output is produced, **Then** the command indicates successful validation clearly.

## Edge Cases

- **Workspace not found**: If command runs outside an NFramework workspace, validation must fail with guidance on running inside a workspace root.
- **Missing or unreadable project metadata**: If a project or source artifact cannot be read, output must identify the problematic path, continue validating remaining readable artifacts, and treat unreadable artifacts as validation failures.
- **Multiple violation classes in one project**: If one project violates references, namespaces, and packages, output must include each violation instance.
- **Duplicate findings from repeated scans**: Validation output must avoid reporting the same violation more than once for the same source location and rule.
- **Empty workspace**: If no projects are found, command must return a deterministic result and explain that nothing was validated.
- **Interrupted execution**: If execution is interrupted, command must terminate with a non-success outcome suitable for automation.

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST provide an `nfw check` command for workspace architecture validation.
- **FR-002**: `nfw check` MUST scan workspace projects for forbidden project references as defined by NFramework layer rules.
- **FR-003**: `nfw check` MUST scan workspace source content for forbidden namespace usage as defined by NFramework layer rules.
- **FR-004**: `nfw check` MUST scan workspace project dependency declarations for forbidden package usage as defined by NFramework layer rules.
- **FR-005**: When at least one violation is detected, `nfw check` MUST exit with a non-zero status.
- **FR-006**: When no violations are detected, `nfw check` MUST exit with status `0`.
- **FR-007**: `nfw check` MUST run without interactive prompts.
- **FR-008**: For each violation, output MUST identify the violating project or file and the forbidden reference, namespace, or package.
- **FR-009**: For each violation, output MUST include an actionable remediation hint.
- **FR-010**: The command MUST support reporting multiple violations in a single run.
- **FR-011**: The project MUST include architecture validation test fixtures that prove at least one valid workspace case and at least one invalid workspace case.
- **FR-012**: Fixture-backed tests MUST verify detection behavior for each violation class: forbidden project reference, forbidden namespace usage, and forbidden package usage.
- **FR-013**: Fixture-backed tests MUST verify exit status semantics for both valid and invalid workspace cases.
- **FR-014**: Validation rules and command behavior MUST align with documented NFramework architecture boundaries.
- **FR-015**: If project or source artifacts are unreadable, `nfw check` MUST report each unreadable path as a validation error, continue scanning remaining readable artifacts, and exit with a non-zero status.
- **FR-016**: Forbidden package validation MUST evaluate only direct package references declared by each project and MUST NOT fail solely due to transitive dependencies.
- **FR-017**: `nfw check` MUST execute `make lint` at workspace root as part of validation and MUST report lint failures as validation findings with actionable remediation.
- **FR-018**: `nfw check` MUST execute `make test` inside each declared service path (`services.*.path` in `nfw.yaml`) and MUST report failing service test runs as validation findings with actionable remediation.

### Key Entities

- **Architecture Rule Set**: Declared boundary constraints that define allowed and forbidden project references, namespaces, and packages by layer.
- **Validation Finding**: A single detected violation including rule type, violating location, violated rule, and remediation hint.
- **Validation Summary**: Aggregated command outcome containing total findings, grouped finding counts by type, and final exit outcome.
- **Architecture Fixture Workspace**: Representative sample workspace used to prove valid and invalid architecture behavior.

## Success Criteria

### Measurable Outcomes

- **SC-001**: In fixture-backed acceptance testing, `nfw check` detects 100% of seeded forbidden reference, namespace, and package violations.
- **SC-002**: In fixture-backed acceptance testing, valid architecture fixtures produce zero findings and exit status `0` in 100% of runs.
- **SC-003**: In fixture-backed acceptance testing, invalid architecture fixtures produce non-zero exit status in 100% of runs.
- **SC-004**: For 100% of reported findings in test validation, output includes both exact violation location and a remediation hint.
- **SC-005**: In CI-oriented non-interactive test runs, command behavior remains deterministic across repeated executions of identical fixtures.
- **SC-006**: In fixture-backed acceptance testing, service test failures are detected in 100% of seeded failing service cases and reported as actionable findings.

## Assumptions

- Architecture boundary rules established by existing workspace and service specs are the source of truth for allowed and forbidden dependencies.
- Workspace artifacts include enough project and source metadata to evaluate project references, namespaces, and package usage.
- Initial validation scope targets architecture checks explicitly called out in PRD and roadmap deliverables.

## Dependencies

- `docs/PRD.md` US-004 and FR-35 for architecture validation scope and outcomes.
- `docs/ROADMAP.md` Phase 2 deliverable requiring `nfw check` validation of forbidden references, namespaces, and packages.
- `docs/SPECIFICATION_GUIDELINES.md` for module spec structure and quality expectations.
- `specs/001-phase1-foundations-core-contracts/tasks.md` F4-T001 as the downstream spec instruction source.
- `src/nfw/specs/002-workspace-structure-new-command/spec.md` and `src/nfw/specs/003-add-service-dotnet-template-based/spec.md` for workspace and layer convention context.

## Clarifications

- Q: Should `nfw check` stop at first violation? → A: No. It should report all detected violations in one run, then exit non-zero.
- Q: Must `nfw check` support CI use without prompts? → A: Yes. It must be fully non-interactive.
- Q: Are architecture fixtures required as part of this feature scope? → A: Yes. Fixtures proving valid and invalid detection are mandatory.

### Session 2026-04-05

- Q: How should unreadable project or source artifacts affect validation outcome? → A: Report unreadable artifacts as validation errors, continue scanning, and exit non-zero.
- Q: Should forbidden package validation include transitive dependencies? → A: No. Validate direct package references only.

## Non-Goals

- Defining new layer boundaries beyond existing architecture rules.
- Implementing service scaffolding, command/query generation, or CRUD generation logic.
- Introducing runtime policy enforcement outside explicit `nfw check` command execution.
