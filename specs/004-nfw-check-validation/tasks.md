# Tasks: `nfw check` Architecture Validation

**Input**: Design documents from `/home/ac/Code/n-framework/src/nfw/specs/004-nfw-check-validation/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/nfw-check-cli-contract.md, quickstart.md

**Tests**: Test tasks are included because the specification explicitly requires architecture validation fixtures and detection proof for valid/invalid cases.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare feature folders, fixtures, and baseline wiring points.

- [X] T001 Create architecture validation feature module folders in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/
- [X] T002 [P] Create integration fixture folder for check command in src/nfw/tests/integration/nframework-nfw/features/architecture_check/
- [X] T003 [P] Create unit test folder for architecture validation services in src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-application/features/architecture_validation/
- [X] T004 [P] Create CLI runtime test folder for check command routing in src/nfw/tests/unit/nframework-nfw/presentation/n-framework-nfw-cli/runtime/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build core rule/finding abstractions and app-level command contract used by all stories.

**⚠️ CRITICAL**: No user story work starts until this phase is complete.

- [X] T005 Define architecture validation domain models in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/models/mod.rs
- [X] T006 [P] Define validation error types in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/models/errors/mod.rs
- [X] T007 Define validation service abstractions in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/services/abstractions/mod.rs
- [X] T008 [P] Add check command request/result models in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/models/check_command_request.rs
- [X] T009 Implement check command handler skeleton in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/commands/check/check_command_handler.rs
- [X] T010 Register architecture validation feature exports in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/mod.rs
- [X] T011 Wire concrete check command handler dependency in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/startup/cli_service_types.rs

**Checkpoint**: Foundational contracts and feature wiring are ready.

---

## Phase 3: User Story 1 - Detect Architecture Violations in a Workspace (Priority: P1) 🎯 MVP

**Goal**: Detect forbidden project references, forbidden namespace usage, and forbidden direct package usage, including valid/invalid fixture coverage.

**Independent Test**: Run architecture check against valid and invalid fixtures and verify each violation class is detected and reported.

### Tests for User Story 1

- [X] T012 [P] [US1] Add valid fixture integration test in src/nfw/tests/integration/nframework-nfw/features/architecture_check/valid_workspace_check_test.rs
- [X] T013 [P] [US1] Add forbidden project reference fixture test in src/nfw/tests/integration/nframework-nfw/features/architecture_check/forbidden_project_reference_test.rs
- [X] T014 [P] [US1] Add forbidden namespace fixture test in src/nfw/tests/integration/nframework-nfw/features/architecture_check/forbidden_namespace_usage_test.rs
- [X] T015 [P] [US1] Add forbidden direct package fixture test in src/nfw/tests/integration/nframework-nfw/features/architecture_check/forbidden_package_usage_test.rs
- [X] T016 [P] [US1] Add shared fixture support utilities in src/nfw/tests/integration/nframework-nfw/features/architecture_check/support.rs

### Implementation for User Story 1

- [X] T017 [P] [US1] Implement workspace rule-set loader service in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/services/rule_set_loader.rs
- [X] T018 [P] [US1] Implement project reference validator in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/services/project_reference_validator.rs
- [X] T019 [P] [US1] Implement namespace usage validator in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/services/namespace_usage_validator.rs
- [X] T020 [P] [US1] Implement direct package usage validator in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/services/package_usage_validator.rs
- [X] T021 [US1] Implement finding deduplication and summary aggregation in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/services/finding_aggregation_service.rs
- [X] T022 [US1] Complete check command handler orchestration across validators in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/commands/check/check_command_handler.rs
- [X] T023 [US1] Add architecture validation service unit tests in src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-application/features/architecture_validation/validation_services.tests.rs

**Checkpoint**: User Story 1 delivers full detection behavior with valid/invalid fixtures.

---

## Phase 4: User Story 2 - Support CI Gatekeeping with Deterministic Exit Outcomes (Priority: P1)

**Goal**: Provide stable non-interactive execution and deterministic exit codes for success/failure/interruption.

**Independent Test**: Execute `nfw check` via CLI runtime tests for pass, violation, unreadable artifact, and interruption outcomes.

### Tests for User Story 2

- [X] T024 [P] [US2] Add runtime routing and execution test for check command in src/nfw/tests/unit/nframework-nfw/presentation/n-framework-nfw-cli/runtime/check_command_routing.tests.rs
- [X] T025 [P] [US2] Add exit code mapping tests for check command results in src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-application/features/cli/exit_codes.tests.rs
- [X] T026 [P] [US2] Add unreadable artifact failure integration test in src/nfw/tests/integration/nframework-nfw/features/architecture_check/unreadable_artifact_test.rs
- [X] T027 [P] [US2] Add deterministic repeat-run integration test in src/nfw/tests/integration/nframework-nfw/features/architecture_check/deterministic_repeatability_test.rs
- [X] T028 [P] [US2] Add stdout vs stderr contract test for check command in src/nfw/tests/unit/nframework-nfw/presentation/n-framework-nfw-cli/runtime/check_command_stdio.tests.rs

### Implementation for User Story 2

- [X] T029 [US2] Add `check` command spec and options in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/runtime/nfw_cli_runtime.rs
- [X] T030 [US2] Implement check CLI command executor in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/check/run_check.rs
- [X] T031 [US2] Register check command module export in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/mod.rs
- [X] T032 [US2] Wire check command handler into service collection in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/startup/cli_service_collection_factory.rs
- [X] T033 [US2] Add check command handler type to service collection in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/startup/cli_service_types.rs
- [X] T034 [US2] Extend exit-code mapping for check command failures and interruptions in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/cli/exit_codes.rs

**Checkpoint**: User Story 2 provides CI-safe deterministic command outcomes.

---

## Phase 5: User Story 3 - Understand and Fix Violations Quickly (Priority: P2)

**Goal**: Emit actionable, location-specific, non-duplicated findings with remediation hints.

**Independent Test**: Trigger each finding type and verify output includes type, location, offending value, and remediation hint.

### Tests for User Story 3

- [X] T035 [P] [US3] Add integration test for actionable finding output format in src/nfw/tests/integration/nframework-nfw/features/architecture_check/actionable_output_test.rs
- [X] T036 [P] [US3] Add integration test for multi-finding aggregate output in src/nfw/tests/integration/nframework-nfw/features/architecture_check/multi_violation_reporting_test.rs
- [X] T037 [P] [US3] Add unit tests for remediation hint resolution in src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-application/features/architecture_validation/remediation_hint.tests.rs

### Implementation for User Story 3

- [X] T038 [US3] Implement finding-to-message formatter with location and remediation fields in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/check/check_output_formatter.rs
- [X] T039 [US3] Integrate formatted output into check CLI command execution in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/check/run_check.rs
- [X] T040 [US3] Implement remediation hint policy mapping by rule type in src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/architecture_validation/services/remediation_hint_service.rs
- [X] T041 [US3] Add success summary output path for zero-finding runs in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/check/check_output_formatter.rs

**Checkpoint**: User Story 3 provides actionable and human-friendly diagnostics.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final consistency, docs, and verification across all stories.

- [X] T042 [P] Add architecture check feature module documentation in src/nfw/docs/architecture-check.md
- [X] T043 Update quickstart commands and expected output examples in src/nfw/specs/004-nfw-check-validation/quickstart.md
- [X] T044 [P] Run architecture_check feature test subset in src/nfw/tests/integration/nframework-nfw/features/architecture_check/
- [X] T045 Run full module validation (`make -C src/nfw build && make -C src/nfw test && make -C src/nfw lint`) using src/nfw/Makefile

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies.
- **Phase 2 (Foundational)**: Depends on Phase 1 and blocks all user stories.
- **Phase 3 (US1)**: Depends on Phase 2.
- **Phase 4 (US2)**: Depends on Phase 2 and US1 command-handler availability.
- **Phase 5 (US3)**: Depends on US1 findings model and US2 command execution path.
- **Phase 6 (Polish)**: Depends on completion of selected user stories.

### User Story Dependencies

- **US1 (P1)**: Core MVP, no user-story dependency.
- **US2 (P1)**: Depends on US1 core validation behavior.
- **US3 (P2)**: Depends on US1/US2 output pipeline.

### Within Each User Story

- Write tests first and confirm they fail before implementation.
- Implement validators/services before command integration.
- Complete command integration before output polishing.

### Parallel Opportunities

- Setup folder creation tasks marked `[P]` run concurrently.
- US1 validator implementations (T018-T020) run in parallel.
- US1 fixture tests (T012-T016) run in parallel.
- US2 tests (T024-T028) run in parallel.
- US3 tests (T035-T037) run in parallel.

---

## Parallel Example: User Story 1

```bash
Task: "T013 [US1] forbidden project reference fixture test"
Task: "T014 [US1] forbidden namespace fixture test"
Task: "T015 [US1] forbidden package fixture test"

Task: "T018 [US1] implement project reference validator"
Task: "T019 [US1] implement namespace usage validator"
Task: "T020 [US1] implement direct package validator"
```

## Parallel Example: User Story 2

```bash
Task: "T024 [US2] runtime routing test"
Task: "T025 [US2] exit-code mapping tests"
Task: "T026 [US2] unreadable artifact integration test"
Task: "T027 [US2] deterministic repeatability integration test"
Task: "T028 [US2] stdout/stderr contract test"
```

## Parallel Example: User Story 3

```bash
Task: "T035 [US3] actionable output integration test"
Task: "T036 [US3] multi-violation reporting integration test"
Task: "T037 [US3] remediation hint unit tests"
```

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Phase 1 and Phase 2.
2. Deliver Phase 3 (US1) with all fixture-based detection tests passing.
3. Validate `nfw check` correctly detects valid/invalid architecture violations.

### Incremental Delivery

1. Add US1 for detection correctness.
2. Add US2 for deterministic CI exit behavior.
3. Add US3 for actionable output quality.
4. Finish polish and full module validation.

### Parallel Team Strategy

1. One engineer handles app-layer validators (US1).
2. One engineer handles CLI wiring/exit semantics (US2).
3. One engineer handles output formatting/remediation messaging (US3).
