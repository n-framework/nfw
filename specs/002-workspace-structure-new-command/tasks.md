# Tasks: Workspace Structure and `nfw new` Command

**Input**: Design documents from `/src/nfw/specs/002-workspace-structure-new-command/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Include unit/integration tests for deterministic routing, validation, and workspace generation acceptance criteria.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`)
- All task descriptions include exact file paths

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare workspace-management feature skeleton across clean-architecture crates.

- [X] T001 Create workspace management feature module skeleton in `src/nframework-nfw/core/nframework-nfw-domain/src/features/workspace_management/` and export it from `src/nframework-nfw/core/nframework-nfw-domain/src/features/mod.rs`
- [X] T002 Create workspace management application module skeleton in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/` and export it from `src/nframework-nfw/core/nframework-nfw-application/src/features/mod.rs`
- [X] T003 [P] Create filesystem workspace management module skeleton in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/workspace_management/` and export it from `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/mod.rs`
- [X] T004 [P] Create CLI workspace command module skeleton in `src/nframework-nfw/presentation/nframework-nfw-cli/src/commands/workspace/` and export it from `src/nframework-nfw/presentation/nframework-nfw-cli/src/commands/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build core abstractions and shared models that all user stories depend on.

**⚠️ CRITICAL**: No user story work begins before this phase completes.

- [X] T005 Define `WorkspaceBlueprint` domain model in `src/nframework-nfw/core/nframework-nfw-domain/src/features/workspace_management/workspace_blueprint.rs`
- [X] T006 [P] Define `NamespaceConvention` domain model in `src/nframework-nfw/core/nframework-nfw-domain/src/features/workspace_management/namespace_convention.rs`
- [X] T007 [P] Define `NewCommandRequest` and `NewCommandResolution` models in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/models/new_command_request.rs` and `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/models/new_command_resolution.rs`
- [X] T008 Define workspace generation abstraction interfaces in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/services/abstraction/workspace_writer.rs`, `prompt_service.rs`, and `workspace_name_validator.rs`
- [X] T009 [P] Define foundational workspace errors and stable exit mapping in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/models/errors/workspace_new_error.rs` and update `src/nframework-nfw/core/nframework-nfw-application/src/features/cli/exit_codes.rs`
- [X] T010 Wire workspace services into CLI service collection in `src/nframework-nfw/presentation/nframework-nfw-cli/src/startup/cli_service_collection_factory.rs`

**Checkpoint**: Foundation ready; user stories can proceed.

---

## Phase 3: User Story 1 - Create a New Workspace Baseline (Priority: P1) 🎯 MVP

**Goal**: Generate a deterministic workspace with layered root, template-driven artifact generation, namespace conventions, and YAML baseline configs.

**Independent Test**: `nfw new BillingPlatform --template official/blank-workspace --no-input` creates required structure (`src/`, `tests/`, `docs/`), template-defined artifacts, and YAML baseline configuration.

### Tests for User Story 1

- [X] T011 [P] [US1] Add domain model tests for blueprint and namespace rules in `tests/unit/nframework-nfw/core/nframework-nfw-domain/features/workspace_management/workspace_blueprint_tests.rs`
- [X] T012 [P] [US1] Add integration test for generated workspace structure and YAML config outputs in `tests/integration/nframework-nfw/features/workspace_new/workspace_layout_test.rs`

### Implementation for User Story 1

- [X] T013 [P] [US1] Implement workspace blueprint construction service in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/services/workspace_blueprint_builder.rs`
- [X] T014 [P] [US1] Implement namespace resolution service in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/services/namespace_resolver.rs`
- [X] T015 [US1] Implement filesystem workspace writer for layered root + template-content rendering in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/workspace_management/services/file_system_workspace_writer.rs`
- [X] T016 [US1] Implement YAML baseline configuration via template-defined `nfw.yaml` content rendering in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/workspace_management/services/file_system_workspace_writer.rs`
- [X] T017 [US1] Implement workspace initialization orchestrator in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/services/workspace_initialization_service.rs`
- [X] T018 [US1] Create `new` command application entrypoint in `src/nframework-nfw/presentation/nframework-nfw-cli/src/commands/workspace/new_workspace.rs`

**Checkpoint**: US1 is independently functional and testable.

---

## Phase 4: User Story 2 - Select Templates in Interactive and Non-Interactive Modes (Priority: P1)

**Goal**: Support prompt-based missing input collection in interactive terminals and strict `--no-input` behavior in automation.

**Independent Test**: Interactive mode prompts for missing required fields; `--no-input` never prompts and fails fast on missing required values.

### Tests for User Story 2

- [X] T019 [P] [US2] Add unit tests for input resolution/prompt gating in `tests/unit/nframework-nfw/core/nframework-nfw-application/features/workspace_management/services/input_resolution_service_tests.rs`
- [X] T020 [P] [US2] Add integration tests for `--no-input` and missing required values in `tests/integration/nframework-nfw/features/workspace_new/no_input_validation_test.rs`

### Implementation for User Story 2

- [X] T021 [P] [US2] Implement interactive prompt abstraction + runtime adapter in `src/nframework-nfw/presentation/nframework-nfw-cli/src/runtime/interactive_prompt_service.rs`
- [X] T022 [US2] Implement input resolution service (interactive vs non-interactive) in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/services/input_resolution_service.rs`
- [X] T023 [US2] Integrate template selection and validation with template-management services in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/services/template_selection_for_new_service.rs`
- [X] T024 [US2] Update `new_workspace` command flow to enforce `--no-input` no-prompt path in `src/nframework-nfw/presentation/nframework-nfw-cli/src/commands/workspace/new_workspace.rs`

**Checkpoint**: US2 works independently and preserves automation safety.

---

## Phase 5: User Story 3 - Route CLI Commands Predictably (Priority: P2)

**Goal**: Add deterministic parsing/routing for `nfw new` with actionable error handling for invalid command shapes.

**Independent Test**: Valid `nfw new` commands route to workspace generation; invalid flags/combinations return stable non-zero exits and actionable guidance.

### Tests for User Story 3

- [X] T025 [P] [US3] Add runtime routing tests for `nfw new` command path in `tests/unit/nframework-nfw/presentation/nframework-nfw-cli/runtime/new_command_routing_tests.rs`
- [X] T026 [P] [US3] Add integration tests for invalid flags, unknown options, and unknown subcommands in `tests/integration/nframework-nfw/features/workspace_new/cli_routing_errors_test.rs`

### Implementation for User Story 3

- [X] T027 [US3] Extend CLI command spec with `new`, `--template`, and `--no-input` in `src/nframework-nfw/presentation/nframework-nfw-cli/src/runtime/nfw_cli_runtime.rs`
- [X] T028 [US3] Register `new` route handler in `src/nframework-nfw/presentation/nframework-nfw-cli/src/runtime/nfw_cli_runtime.rs` and map to `src/nframework-nfw/presentation/nframework-nfw-cli/src/commands/workspace/new_workspace.rs`
- [X] T029 [US3] Implement deterministic option-combination validation and actionable errors in `src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/services/new_command_validator.rs`
- [X] T030 [US3] Update CLI startup wiring for new command dependencies in `src/nframework-nfw/presentation/nframework-nfw-cli/src/startup/cli_service_collection_factory.rs` and `src/nframework-nfw/presentation/nframework-nfw-cli/src/startup/cli_bootstrapper.rs`

**Checkpoint**: US3 delivers predictable CLI routing and error behavior.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final consistency checks, docs alignment, and acceptance validation.

- [X] T031 [P] Update workspace generation documentation in `src/nfw/docs/` with `nfw new` usage, flags, and failure behaviors
- [X] T032 Align feature spec quickstart and generated command help text in `src/nfw/specs/002-workspace-structure-new-command/quickstart.md` and `src/nframework-nfw/presentation/nframework-nfw-cli/src/runtime/nfw_cli_runtime.rs`
- [X] T033 [P] Add regression integration test for reproducible output from identical inputs in `tests/integration/nframework-nfw/features/workspace_new/reproducible_generation_test.rs`
- [X] T034 Run and document acceptance verification commands in `src/nfw/specs/002-workspace-structure-new-command/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies.
- **Phase 2 (Foundational)**: Depends on Phase 1; blocks all user stories.
- **Phase 3 (US1)**: Depends on Phase 2.
- **Phase 4 (US2)**: Depends on Phase 2 and integrates with US1 command flow.
- **Phase 5 (US3)**: Depends on Phase 2; should land after core `new` command scaffolding (T018).
- **Phase 6 (Polish)**: Depends on completion of selected user stories.

### User Story Dependencies

- **US1 (P1)**: First MVP slice; no dependency on other stories after foundation.
- **US2 (P1)**: Depends on US1 command baseline to apply interactive/non-interactive behavior.
- **US3 (P2)**: Can start after foundation but should merge after `new` command baseline exists to avoid routing conflicts.

### Within Each User Story

- Tests first (or early), then models/services, then CLI/runtime wiring.
- Validation logic before final integration.
- Story checkpoint must pass independently.

## Parallel Opportunities

- Setup: T003 and T004 can run in parallel after T001/T002 scaffolding decisions.
- Foundational: T006, T007, T009 can run in parallel.
- US1: T011, T012, T013, T014 can run in parallel; T015/T016 after model services are stable.
- US2: T019, T020, T021 can run in parallel.
- US3: T025 and T026 can run in parallel; T027/T028 can proceed together before T030 integration.
- Polish: T031 and T033 can run in parallel.

---

## Parallel Example: User Story 1

```bash
# Parallelizable US1 tasks
Task: "T011 [US1] Add domain model tests in tests/unit/nframework-nfw/core/nframework-nfw-domain/features/workspace_management/workspace_blueprint_tests.rs"
Task: "T013 [US1] Implement workspace blueprint builder in src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/services/workspace_blueprint_builder.rs"
Task: "T014 [US1] Implement namespace resolver in src/nframework-nfw/core/nframework-nfw-application/src/features/workspace_management/services/namespace_resolver.rs"
```

## Parallel Example: User Story 2

```bash
# Parallelizable US2 tasks
Task: "T019 [US2] Add input resolution tests in tests/unit/nframework-nfw/core/nframework-nfw-application/features/workspace_management/services/input_resolution_service_tests.rs"
Task: "T021 [US2] Implement interactive prompt runtime adapter in src/nframework-nfw/presentation/nframework-nfw-cli/src/runtime/interactive_prompt_service.rs"
```

## Parallel Example: User Story 3

```bash
# Parallelizable US3 tasks
Task: "T025 [US3] Add routing tests in tests/unit/nframework-nfw/presentation/nframework-nfw-cli/runtime/new_command_routing_tests.rs"
Task: "T027 [US3] Extend CLI command spec in src/nframework-nfw/presentation/nframework-nfw-cli/src/runtime/nfw_cli_runtime.rs"
```

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Phase 1 (Setup)
2. Complete Phase 2 (Foundational)
3. Complete Phase 3 (US1)
4. Validate US1 independently via quickstart and integration checks

### Incremental Delivery

1. Deliver US1 (workspace baseline generation)
2. Deliver US2 (interactive and `--no-input` behavior)
3. Deliver US3 (deterministic routing and error contract)
4. Complete Polish for reproducibility and docs consistency

### Team Parallel Strategy

1. One engineer handles foundational domain/application models.
2. One engineer handles CLI runtime + command routing.
3. One engineer handles filesystem writer + integration tests.
4. Merge at story checkpoints only.

---

## Notes

- All tasks follow strict checklist format with IDs, labels, and file paths.
- `[P]` marks tasks that can run independently on disjoint files.
- Story phases preserve independent testability and incremental delivery.
- Suggested MVP scope: **Phase 1 + Phase 2 + Phase 3 (US1)**.
