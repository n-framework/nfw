# Tasks: Generator-Based `nfw add service`

**Input**: Design documents from `/src/nfw/specs/003-add-service-dotnet-generator-based/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Include unit/integration tests for generator selection behavior, generation output, workspace context/cleanup behavior, and provenance persistence.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US4`)
- All task descriptions include exact file paths

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare service-generation feature skeleton across clean-architecture crates.

- [x] T001 Create service management domain module skeleton in `src/nframework-nfw/core/nframework-nfw-domain/src/features/service_management/` and export it from `src/nframework-nfw/core/nframework-nfw-domain/src/features/mod.rs`
- [x] T002 Create service management application module skeleton in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/` and export it from `src/nframework-nfw/core/nframework-nfw-application/src/features/mod.rs`
- [x] T003 [P] Create filesystem service management module skeleton in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/service_management/` and export it from `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/mod.rs`
- [x] T004 [P] Create CLI add-service command module skeleton in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/service/` and export it from `src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build shared models, abstractions, and contracts required by all stories.

**⚠️ CRITICAL**: No user story work begins before this phase completes.

- [x] T005 Define `AddServiceCommandRequest` and validation model in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/models/add_service_command_request.rs`
- [x] T006 [P] Define `ServiceGeneratorResolution` model in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/models/service_generator_resolution.rs`
- [x] T007 [P] Define `ServiceGenerationPlan` model in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/models/service_generation_plan.rs`
- [x] T008 [P] Remove language-specific layer-dependency enforcement from `nfw add service` scope and keep CLI language-agnostic
- [x] T009 Define service-generation abstraction interfaces in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/abstractions/service_generator_selector.rs`, `service_generator_renderer.rs`, and `service_provenance_store.rs`
- [x] T010 Define service-generation error model and exit-code mapping in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/models/errors/add_service_error.rs` and update `src/nframework-nfw/core/nframework-nfw-application/src/features/cli/exit_codes.rs`
- [x] T011 Wire service-management services into CLI dependency registration in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/startup/cli_service_collection_factory.rs`

**Checkpoint**: Foundation ready; user stories can proceed.

---

## Phase 3: User Story 1 - Generate a Service from Generator (Priority: P1) 🎯 MVP

**Goal**: Generate a service from selected generator into `src/<ServiceName>/` with deterministic rendering and first-build success.

**Independent Test**: `nfw add service Orders --generator official/dotnet-service --no-input` creates service under `src/Orders/` with expected layer projects and buildable output.

### Tests for User Story 1

- [x] T012 [P] [US1] Add request validation tests for service name and non-interactive generator requirement in `tests/unit/nframework-nfw/core/nframework-nfw-application/features/service_management/add_service_request_validation.tests.rs`
- [x] T013 [P] [US1] Add integration test for generated service output path and structure in `tests/integration/nframework-nfw/features/service_add/service_generation_layout_test.rs`
- [x] T040 [P] [US1] Add integration test for workspace-context failure outside `nfw.yaml` workspace in `tests/integration/nframework-nfw/features/service_add/service_workspace_context_validation_test.rs`
- [x] T041 [P] [US1] Add integration smoke test for first-build success of generated service in `tests/integration/nframework-nfw/features/service_add/service_first_build_smoke_test.rs`
- [x] T042 [P] [US1] Add integration tests for rollback cleanup on render failure and SIGINT interruption in `tests/integration/nframework-nfw/features/service_add/service_generation_cleanup_test.rs`

### Implementation for User Story 1

- [x] T014 [P] [US1] Implement add-service request validator in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/add_service_request_validator.rs`
- [x] T015 [P] [US1] Implement generation-plan builder for `src/<ServiceName>/` output in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_generation_plan_builder.rs`
- [x] T016 [US1] Implement filesystem generator rendering and write operations in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/service_management/services/file_system_service_generator_renderer.rs`
- [x] T017 [US1] Implement add-service command handler in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/commands/add_service/add_service_command_handler.rs`
- [x] T018 [US1] Implement CLI command handler for `nfw add service` in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/service/add_service.rs`
- [x] T019 [US1] Register `add service` command routing and options in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/runtime/nfw_cli_runtime.rs`
- [x] T043 [US1] Implement explicit workspace-context guard (`nfw.yaml` discovery) in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/add_service_workspace_context_guard.rs`
- [x] T044 [US1] Implement partial-output cleanup coordinator for failure/SIGINT paths in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/service_management/services/service_generation_cleanup.rs` and integrate it in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/service_management/services/file_system_service_generator_renderer.rs`

**Checkpoint**: US1 is independently functional and testable.

---

## Phase 4: User Story 4 - Reuse Existing Generator System (Priority: P1)

**Goal**: Resolve service generators through existing catalog/cache/versioning and enforce `type=service` eligibility.

**Independent Test**: Generator is selected via existing generator-management stack; invalid generator or wrong type fails before rendering.

### Tests for User Story 4

- [x] T020 [P] [US4] Add unit tests for generator eligibility and version resolution behavior in `tests/unit/nframework-nfw/core/nframework-nfw-application/features/service_management/service_generator_selection.tests.rs`
- [x] T021 [P] [US4] Add integration test for invalid generator ID and wrong generator type failures in `tests/integration/nframework-nfw/features/service_add/service_generator_validation_test.rs`

### Implementation for User Story 4

- [x] T022 [P] [US4] Implement service-generator selection adapter using existing generator-management services in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_generator_selection_service.rs`
- [x] T023 [US4] Implement interactive generator prompt adapter for add-service flow in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/runtime/interactive_service_generator_prompt.rs`
- [x] T024 [US4] Integrate interactive/non-interactive generator selection policy in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/add_service_input_resolution_service.rs`
- [x] T025 [US4] Enforce generator metadata `type=service` check before render in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_generator_selection_service.rs`

**Checkpoint**: US4 reuses generator infrastructure with strict eligibility and deterministic selection.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Persist provenance, align docs/contracts, and run end-to-end acceptance validation.

- [x] T035 [P] Implement generator provenance persistence in workspace YAML writer in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-yaml/src/features/workspace_management/services/workspace_metadata_writer.rs`
- [x] T036 Implement provenance write flow for add-service in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_generator_provenance_service.rs`
- [x] T037 [P] Add integration test for provenance persistence in `nfw.yaml` under service entry in `tests/integration/nframework-nfw/features/service_add/service_generator_provenance_persistence_test.rs`
- [x] T038 Align command docs and quickstart examples for generator-first add-service flow in `src/nfw/docs/` and `src/nfw/specs/003-add-service-dotnet-generator-based/quickstart.md`
- [x] T039 Run and record acceptance verification command set from repository root in `src/nfw/specs/003-add-service-dotnet-generator-based/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies.
- **Phase 2 (Foundational)**: Depends on Phase 1; blocks all user stories.
- **Phase 3 (US1)**: Depends on Phase 2.
- **Phase 4 (US4)**: Depends on Phase 2 and integrates with US1 command flow.
- **Phase 7 (Polish)**: Depends on completion of selected user stories.

### User Story Dependencies

- **US1 (P1)**: MVP slice after foundation.
- **US4 (P1)**: Depends on US1 command baseline for integration points.

### Within Each User Story

- Tests first (or early), then core services, then CLI/runtime wiring and command-handler integration.
- Story checkpoint must pass independently.

## Parallel Opportunities

- Setup: T003 and T004 can run in parallel.
- Foundational: T006, T007, T008 can run in parallel after T005.
- US1: T012, T013, T014, T015, T040, T041, T042 can run in parallel before command-handler integration.
- US4: T020, T021, T022 can run in parallel before T024/T025 integration.
- Polish: T035 and T037 can run in parallel.

---

## Parallel Example: User Story 1

```bash
# Parallelizable US1 tasks
Task: "T012 [US1] Add request validation tests in tests/unit/nframework-nfw/core/nframework-nfw-application/features/service_management/add_service_request_validation.tests.rs"
Task: "T014 [US1] Implement request validator in src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/add_service_request_validator.rs"
Task: "T015 [US1] Implement generation-plan builder in src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_generation_plan_builder.rs"
```

## Parallel Example: User Story 4

```bash
# Parallelizable US4 tasks
Task: "T020 [US4] Add generator selection tests in tests/unit/nframework-nfw/core/nframework-nfw-application/features/service_management/service_generator_selection.tests.rs"
Task: "T022 [US4] Implement service-generator selection adapter in src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_generator_selection_service.rs"
```

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Phase 1 (Setup)
2. Complete Phase 2 (Foundational)
3. Complete Phase 3 (US1)
4. Validate US1 independently via quickstart and integration checks

### Incremental Delivery

1. Deliver US1 (generator-based service generation)
2. Deliver US4 (generator-system reuse and selection policy)
3. Complete Polish (provenance persistence, docs, acceptance validation)

### Team Parallel Strategy

1. One engineer handles CLI/runtime command surface.
2. One engineer handles application command handlers and validation services.
3. One engineer handles filesystem/yaml infrastructure and integration tests.
4. Merge at user-story checkpoints.

---

## Notes

- All tasks follow strict checklist format with IDs, labels, and file paths.
- `[P]` marks tasks that can run independently on disjoint files.
- Suggested MVP scope: **Phase 1 + Phase 2 + Phase 3 (US1)**.
