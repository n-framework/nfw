# Tasks: Template-Based `nfw add service`

**Input**: Design documents from `/src/nfw/specs/003-add-service-dotnet-template-based/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Include unit/integration tests for template selection behavior, generation output, workspace context/cleanup behavior, and provenance persistence.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US4`)
- All task descriptions include exact file paths

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare service-generation feature skeleton across clean-architecture crates.

- [X] T001 Create service management domain module skeleton in `src/nframework-nfw/core/nframework-nfw-domain/src/features/service_management/` and export it from `src/nframework-nfw/core/nframework-nfw-domain/src/features/mod.rs`
- [X] T002 Create service management application module skeleton in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/` and export it from `src/nframework-nfw/core/nframework-nfw-application/src/features/mod.rs`
- [X] T003 [P] Create filesystem service management module skeleton in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/service_management/` and export it from `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/mod.rs`
- [X] T004 [P] Create CLI add-service command module skeleton in `src/nframework-nfw/presentation/nframework-nfw-cli/src/commands/service/` and export it from `src/nframework-nfw/presentation/nframework-nfw-cli/src/commands/mod.rs`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build shared models, abstractions, and contracts required by all stories.

**⚠️ CRITICAL**: No user story work begins before this phase completes.

- [X] T005 Define `AddServiceCommandRequest` and validation model in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/models/add_service_command_request.rs`
- [X] T006 [P] Define `ServiceTemplateResolution` model in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/models/service_template_resolution.rs`
- [X] T007 [P] Define `ServiceGenerationPlan` model in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/models/service_generation_plan.rs`
- [X] T008 [P] Remove language-specific layer-dependency enforcement from `nfw add service` scope and keep CLI language-agnostic
- [X] T009 Define service-generation abstraction interfaces in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/abstractions/service_template_selector.rs`, `service_template_renderer.rs`, and `service_provenance_store.rs`
- [X] T010 Define service-generation error model and exit-code mapping in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/models/errors/add_service_error.rs` and update `src/nframework-nfw/core/nframework-nfw-application/src/features/cli/exit_codes.rs`
- [X] T011 Wire service-management services into CLI dependency registration in `src/nframework-nfw/presentation/nframework-nfw-cli/src/startup/cli_service_collection_factory.rs`

**Checkpoint**: Foundation ready; user stories can proceed.

---

## Phase 3: User Story 1 - Generate a Service from Template (Priority: P1) 🎯 MVP

**Goal**: Generate a service from selected template into `src/<ServiceName>/` with deterministic rendering and first-build success.

**Independent Test**: `nfw add service Orders --template official/dotnet-service --no-input` creates service under `src/Orders/` with expected layer projects and buildable output.

### Tests for User Story 1

- [X] T012 [P] [US1] Add request validation tests for service name and non-interactive template requirement in `tests/unit/nframework-nfw/core/nframework-nfw-application/features/service_management/add_service_request_validation_tests.rs`
- [X] T013 [P] [US1] Add integration test for generated service output path and structure in `tests/integration/nframework-nfw/features/service_add/service_generation_layout_test.rs`
- [X] T040 [P] [US1] Add integration test for workspace-context failure outside `nfw.yaml` workspace in `tests/integration/nframework-nfw/features/service_add/service_workspace_context_validation_test.rs`
- [X] T041 [P] [US1] Add integration smoke test for first-build success of generated service in `tests/integration/nframework-nfw/features/service_add/service_first_build_smoke_test.rs`
- [X] T042 [P] [US1] Add integration tests for rollback cleanup on render failure and SIGINT interruption in `tests/integration/nframework-nfw/features/service_add/service_generation_cleanup_test.rs`

### Implementation for User Story 1

- [X] T014 [P] [US1] Implement add-service request validator in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/add_service_request_validator.rs`
- [X] T015 [P] [US1] Implement generation-plan builder for `src/<ServiceName>/` output in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_generation_plan_builder.rs`
- [X] T016 [US1] Implement filesystem template rendering and write operations in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/service_management/services/file_system_service_template_renderer.rs`
- [X] T017 [US1] Implement add-service command handler in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/commands/add_service/add_service_command_handler.rs`
- [X] T018 [US1] Implement CLI command handler for `nfw add service` in `src/nframework-nfw/presentation/nframework-nfw-cli/src/commands/service/add_service.rs`
- [X] T019 [US1] Register `add service` command routing and options in `src/nframework-nfw/presentation/nframework-nfw-cli/src/runtime/nfw_cli_runtime.rs`
- [X] T043 [US1] Implement explicit workspace-context guard (`nfw.yaml` discovery) in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/add_service_workspace_context_guard.rs`
- [X] T044 [US1] Implement partial-output cleanup coordinator for failure/SIGINT paths in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/service_management/services/service_generation_cleanup.rs` and integrate it in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/service_management/services/file_system_service_template_renderer.rs`

**Checkpoint**: US1 is independently functional and testable.

---

## Phase 4: User Story 4 - Reuse Existing Template System (Priority: P1)

**Goal**: Resolve service templates through existing catalog/cache/versioning and enforce `type=service` eligibility.

**Independent Test**: Template is selected via existing template-management stack; invalid template or wrong type fails before rendering.

### Tests for User Story 4

- [X] T020 [P] [US4] Add unit tests for template eligibility and version resolution behavior in `tests/unit/nframework-nfw/core/nframework-nfw-application/features/service_management/service_template_selection_tests.rs`
- [X] T021 [P] [US4] Add integration test for invalid template ID and wrong template type failures in `tests/integration/nframework-nfw/features/service_add/service_template_validation_test.rs`

### Implementation for User Story 4

- [X] T022 [P] [US4] Implement service-template selection adapter using existing template-management services in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_template_selection_service.rs`
- [X] T023 [US4] Implement interactive template prompt adapter for add-service flow in `src/nframework-nfw/presentation/nframework-nfw-cli/src/runtime/interactive_service_template_prompt.rs`
- [X] T024 [US4] Integrate interactive/non-interactive template selection policy in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/add_service_input_resolution_service.rs`
- [X] T025 [US4] Enforce template metadata `type=service` check before render in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_template_selection_service.rs`

**Checkpoint**: US4 reuses template infrastructure with strict eligibility and deterministic selection.

---

## Phase 7: Polish & Cross-Cutting Concerns

**Purpose**: Persist provenance, align docs/contracts, and run end-to-end acceptance validation.

- [X] T035 [P] Implement template provenance persistence in workspace YAML writer in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-yaml/src/features/workspace_management/services/workspace_metadata_writer.rs`
- [X] T036 Implement provenance write flow for add-service in `src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_template_provenance_service.rs`
- [X] T037 [P] Add integration test for provenance persistence in `nfw.yaml` under service entry in `tests/integration/nframework-nfw/features/service_add/service_template_provenance_persistence_test.rs`
- [X] T038 Align command docs and quickstart examples for template-first add-service flow in `src/nfw/docs/` and `src/nfw/specs/003-add-service-dotnet-template-based/quickstart.md`
- [X] T039 Run and record acceptance verification command set from repository root in `src/nfw/specs/003-add-service-dotnet-template-based/quickstart.md`

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
Task: "T012 [US1] Add request validation tests in tests/unit/nframework-nfw/core/nframework-nfw-application/features/service_management/add_service_request_validation_tests.rs"
Task: "T014 [US1] Implement request validator in src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/add_service_request_validator.rs"
Task: "T015 [US1] Implement generation-plan builder in src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_generation_plan_builder.rs"
```

## Parallel Example: User Story 4

```bash
# Parallelizable US4 tasks
Task: "T020 [US4] Add template selection tests in tests/unit/nframework-nfw/core/nframework-nfw-application/features/service_management/service_template_selection_tests.rs"
Task: "T022 [US4] Implement service-template selection adapter in src/nframework-nfw/core/nframework-nfw-application/src/features/service_management/services/service_template_selection_service.rs"
```

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Phase 1 (Setup)
2. Complete Phase 2 (Foundational)
3. Complete Phase 3 (US1)
4. Validate US1 independently via quickstart and integration checks

### Incremental Delivery

1. Deliver US1 (template-based service generation)
2. Deliver US4 (template-system reuse and selection policy)
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
