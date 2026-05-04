---
description: "Task list for Generate Repository Command feature"
---

# Tasks: Generate Repository Command

**Input**: Design documents from `src/nfw/specs/011-gen-repository-command/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Tests**: Integration tests included (required in spec FR-007).

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2, US3)
- Include exact file paths in descriptions

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Project initialization and basic structure

- [x] T001 Create feature branch and verify .specify/feature.json points to correct directory
- [x] T002 [P] Verify dependencies: clap, serde, serde_yaml in src/nfw/Cargo.toml
- [x] T003 [P] Verify core-persistence-dotnet submodule is accessible at src/core-persistence-dotnet/ (prerequisite for template rendering)

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core infrastructure that MUST be complete before ANY user story can be implemented

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T004 Create repository template directory at src/nfw-templates/src/dotnet-service/repository/
- [x] T005 Create repository template configuration file at src/nfw-templates/src/dotnet-service/repository/template.yaml with steps (render interface, render implementation, inject DI)
- [x] T006 [P] Create interface template file at src/nfw-templates/src/dotnet-service/repository/content/interface/IEntityRepository.cs.tera
- [x] T007 [P] Create implementation template file at src/nfw-templates/src/dotnet-service/repository/content/implementation/EntityRepository.cs.tera
- [x] T008 [P] Create DI registration template file at src/nfw-templates/src/dotnet-service/repository/content/di-registration/registration.tera
- [x] T009 Add `repository: ./repository/` generator entry to src/nfw-templates/src/dotnet-service/template.yaml under generators section
- [x] T010 Create repository command structure in src/nfw/src/commands/gen/repository.rs with CLI argument parsing (entity name, --feature flag)

**Checkpoint**: Foundation ready - user story implementation can now begin

---

## Phase 3: User Story 1 - Generate repository for default feature (Priority: P1) 🎯 MVP

**Goal**: Generate repository files (interface, implementation, DI registration) for existing entity in default feature folder

**Independent Test**: Run `nfw gen repository User` with valid entity → verify files generated, CLI exits with code 0

### Tests for User Story 1 ⚠️

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T011 [P] [US1] Integration test: successful repository generation in src/nfw/tests/integration/gen_repository_tests.rs (entity exists, persistence configured)
- [x] T012 [P] [US1] Integration test: entity not found error in src/nfw/tests/integration/gen_repository_tests.rs
- [x] T013 [P] [US1] Integration test: persistence not configured error in src/nfw/tests/integration/gen_repository_tests.rs
- [x] T014 [P] [US1] Validation test: 100% generated files match template spec (SC-003) in src/nfw/tests/integration/gen_repository_tests.rs
- [x] T015 [P] [US1] Validation test: 100% files in correct locations per template (SC-004) in src/nfw/tests/integration/gen_repository_tests.rs
- [x] T016 [P] [US1] Implement stderr output for all error messages in src/nfw/src/commands/gen/repository.rs (FR-010, Constitution II)

### Implementation for User Story 1

- [x] T017 [US1] Implement entity existence validation in src/nfw/src/commands/gen/repository.rs (check Domain/Entities/ folder)
- [x] T018 [US1] Implement persistence configuration validation in src/nfw/src/commands/gen/repository.rs (read nfw.yaml, check persistence section)
- [x] T019 [US1] Implement template configuration reading in src/nfw/src/commands/gen/repository.rs (parse repository/template.yaml)
- [x] T020 [US1] Implement template application logic in src/nfw/src/commands/gen/repository.rs (render files, inject DI)
- [x] T021 [US1] Add default feature auto-detection logic in src/nfw/src/commands/gen/repository.rs (find feature containing entity)
- [x] T022 [US1] Implement error handling for edge cases in src/nfw/src/commands/gen/repository.rs (invalid entity name, missing permissions, etc.)

**Checkpoint**: At this point, User Story 1 should be fully functional and testable independently

---

## Phase 4: User Story 2 - Generate repository for specific feature (Priority: P2)

**Goal**: Generate repository for specific feature using --feature flag

**Independent Test**: Run `nfw gen repository Order --feature payments` → verify files generated in payments feature's layers

### Tests for User Story 2 ⚠️

- [x] T023 [P] [US2] Integration test: successful repository generation with --feature flag in src/nfw/tests/integration/gen_repository_tests.rs
- [x] T024 [P] [US2] Integration test: invalid feature folder error in src/nfw/tests/integration/gen_repository_tests.rs

### Implementation for User Story 2

- [x] T025 [US2] Implement --feature parameter handling in src/nfw/src/commands/gen/repository.rs
- [x] T026 [US2] Implement feature folder validation in src/nfw/src/commands/gen/repository.rs (check feature exists)
- [x] T027 [US2] Update template application to use specified feature path in src/nfw/src/commands/gen/repository.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work independently

---

## Phase 5: User Story 3 - Command performance and validation (Priority: P3)

**Goal**: Command completes in <2 seconds, validates preconditions quickly

**Independent Test**: Time `nfw gen repository User` execution → verify <2 seconds; verify error cases return in <1 second

### Tests for User Story 3 ⚠️

- [x] T028 [P] [US3] Performance test: command completes in <2 seconds in src/nfw/tests/integration/gen_repository_tests.rs
- [x] T029 [P] [US3] Performance test: error cases return in <1 second in src/nfw/tests/integration/gen_repository_tests.rs
- [x] T030 [P] [US3] Performance benchmark: 100% of valid commands complete in <2s, 100% errors in <1s (validate SC-001, SC-002) in src/nfw/tests/integration/gen_repository_tests.rs

### Implementation for User Story 3

- [x] T031 [US3] Add execution time measurement and validation in src/nfw/src/commands/gen/repository.rs
- [x] T032 [US3] Optimize validation steps for <2 second completion in src/nfw/src/commands/gen/repository.rs
- [x] T033 [US3] Ensure clean shutdown on Ctrl+C interrupt in src/nfw/src/commands/gen/repository.rs
- [x] T034 [US3] Implement atomic file operations for concurrency safety in src/nfw/src/commands/gen/repository.rs

**Checkpoint**: All user stories should now be independently functional

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Improvements that affect multiple user stories

- [x] T035 [P] Update quickstart.md with verified examples in src/nfw/specs/011-gen-repository-command/quickstart.md
- [x] T036 [P] Code cleanup and formatting in src/nfw/src/commands/gen/repository.rs
- [x] T037 [P] Add unit tests for helper functions in src/nfw/tests/unit/repository_command_tests.rs:
  - Test argument parsing (valid/invalid entity names)
  - Test feature auto-detection logic
  - Test template path substitution ({{ Service }}, {{ Feature }}, {{ Entity }})
  - Test error message formatting (stderr output)
- [x] T038 Run cargo clippy and fix all warnings in src/nfw/
- [x] T039 Verify constitution compliance: single-step build (`cd src/nfw && make build`), CLI I/O, no suppression, deterministic tests
- [x] T040 [P] Aggregate validation: Run full integration test suite and verify SC-005 (100% tests pass, all user scenarios covered) in src/nfw/tests/integration/gen_repository_tests.rs

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 3+)**: All depend on Foundational (Phase 2) completion
  - User stories can then proceed in parallel (if staffed)
  - Or sequentially in priority order (P1 → P2 → P3)
- **Polish (Phase 6)**: Depends on all desired user stories being complete

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Can start after Foundational (Phase 2) - May integrate with US1 but should be independently testable
- **User Story 3 (P3)**: Can start after Foundational (Phase 2) - May integrate with US1/US2 but should be independently testable

### Within Each User Story

- Tests (if included) MUST be written and FAIL before implementation
- Validation logic before file generation
- Core implementation before integration
- Story complete before moving to next priority

### Parallel Opportunities

- All Setup tasks marked [P] can run in parallel
- All Foundational tasks marked [P] can run in parallel (within Phase 2)
- Once Foundational phase completes, all user stories can start in parallel (if team capacity allows)
- All tests for a user story marked [P] can run in parallel
- Template file creation tasks (T006, T007, T008) can run in parallel

---

## Parallel Example: User Story 1

```bash
# Launch all tests for User Story 1 together (if tests requested):
Task: "T011 Integration test: successful repository generation"
Task: "T012 Integration test: entity not found error"
Task: "T013 Integration test: persistence not configured error"
Task: "T014 Validation: files match template spec"
Task: "T015 Validation: files in correct locations"

# Launch all template tasks (already done in Foundational, but shown for reference):
Task: "T006 Create interface template file"
Task: "T007 Create implementation template file"
Task: "T008 Create DI registration template file"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks all stories)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test User Story 1 independently
5. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP!)
3. Add User Story 2 → Test independently → Deploy/Demo
4. Add User Story 3 → Test independently → Deploy/Demo
5. Each story adds value without breaking previous stories

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: User Story 1
   - Developer B: User Story 2
   - Developer C: User Story 3
3. Stories complete and integrate independently

---

## Notes

- [P] tasks = different files, no dependencies
- [Story] label maps task to specific user story for traceability
- Each user story should be independently completable and testable
- Verify tests fail before implementing
- Commit after each task or logical group
- Stop at any checkpoint to validate story independently
- Avoid: vague tasks, same file conflicts, cross-story dependencies that break independence
