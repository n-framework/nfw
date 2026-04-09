# Tasks: Build & Test Workflows

**Input**: Design documents from `src/nfw/specs/005-build-test-workflows/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/

**Tests**: Include smoke tests (shell scripts) and integration tests (Rust) for CLI workflows, build/test validation, and benchmark harness.

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`US1`, `US2`, `US3`)
- All task descriptions include exact file paths

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Prepare test infrastructure directories and shared test utilities.

- [x] T001 Create smoke test directory `tests/smoke/` and verify it is excluded from Rust workspace compilation
- [x] T002 [P] Create benchmark script directory `scripts/benchmark/`
- [x] T003 [P] Create test fixture directory structure `tests/fixtures/valid-workspace/` and `tests/fixtures/invalid-workspace/`
- [x] T004 Create shared smoke test utility functions (temp directory creation, cleanup traps, assertion helpers) in `tests/smoke/common.sh`

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build shared test infrastructure that all user stories depend on.

**⚠️ CRITICAL**: No user story work begins before this phase completes.

- [x] T005 Define smoke test runner script that discovers and executes all `*_test.sh` files under `tests/smoke/` in `tests/smoke/run_smoke_tests.sh`
- [x] T006 [P] Define benchmark harness entry point with argument parsing (--iterations, --test, --help) in `scripts/benchmark/run_benchmark.sh`
- [x] T007 [P] Define benchmark result JSON schema file at `scripts/benchmark/benchmark_result_schema.json` matching `contracts/benchmark-result-schema.yaml`
- [x] T008 [P] Define environment metadata collection function (CPU cores, RAM, OS, disk type, CLI version) in `scripts/benchmark/env_metadata.sh` — reused by benchmark harness (T024) and CI workflows (T029)
- [x] T009 Create valid workspace fixture with minimal `nfw.yaml`, `src/`, `tests/`, `docs/` structure in `tests/fixtures/valid-workspace/`
- [x] T010 Create invalid workspace fixture with known architecture violations in `tests/fixtures/invalid-workspace/`

**Checkpoint**: Foundation ready; user stories can proceed.

---

## Phase 3: User Story 1 - Smoke Test End-to-End Workflows (Priority: P1) 🎯 MVP

**Goal**: Implement CLI smoke tests validating template selection, workspace generation, and service scaffolding in both interactive and non-interactive modes.

**Independent Test**: Run `tests/smoke/run_smoke_tests.sh` and verify all scenarios pass with deterministic exit status 0.

### Tests for User Story 1

- [x] T011 [P] [US1] Implement template selection smoke test (non-interactive mode) in `tests/smoke/template_selection_test.sh`
  - Validates `nfw new TestWorkspace --template official/blank-workspace --no-input` exits 0
  - Validates workspace directory created with `nfw.yaml` at root
  - Validates template identifier recorded in configuration
  - Uses isolated temp directory with cleanup trap
  - Validates temp directory is fully cleaned up after test completion (FR-006)

- [x] T012 [P] [US1] Implement workspace generation smoke test in `tests/smoke/workspace_generation_test.sh`
  - Validates `nfw new` creates `src/`, `tests/`, `docs/` directories
  - Validates baseline configuration files present
  - Validates no manual edits required for build
  - Uses isolated temp directory with cleanup trap
  - Validates temp directory is fully cleaned up after test completion (FR-006)
  - Validates no cross-test contamination from concurrent runs (FR-005)

- [x] T013 [P] [US1] Implement service scaffolding smoke test in `tests/smoke/service_scaffolding_test.sh`
  - Validates `nfw add service TestService --lang dotnet` exits 0
  - Validates four-layer structure (Domain, Application, Infrastructure, Api)
  - Validates service compiles successfully
  - Validates project references follow layer dependency rules
  - Uses isolated temp directory with cleanup trap
  - Validates temp directory is fully cleaned up after test completion (FR-006)

### Implementation for User Story 1

- [x] T014 [US1] Wire smoke test runner into `Makefile` as `make smoke-tests` target in `src/nfw/Makefile`
- [x] T015 [US1] Add smoke test prerequisites check (CLI installed, template cache populated, .NET SDK available) in `tests/smoke/common.sh`
- [x] T016 [US1] Add smoke test documentation to `src/nfw/docs/SMOKE_TESTS.md` with usage examples and expected output

**Checkpoint**: US1 is independently functional. Running `make smoke-tests` validates all core CLI workflows.

---

## Phase 4: User Story 2 - Single-Command Build and Test for Generated Workspaces (Priority: P1)

**Goal**: Implement integration tests validating that generated workspaces build and test with single commands from workspace root.

**Independent Test**: Generate a workspace, run `make build` then `make test`, and verify both succeed without manual file edits.

### Tests for User Story 2

- [x] T017 [P] [US2] Implement build workflow integration test in `tests/integration/nframework-nfw/features/workspace_new/build_test_workflow_test.rs`
  - Generates workspace via `nfw new` with test template
  - Runs `make build` from workspace root
  - Validates exit code 0 and successful compilation
  - Validates no warnings suppressed
  - Cleans up generated workspace
  - Note: Shares file with T018/T019 as separate `#[test]` functions (idiomatic Rust)

- [x] T018 [US2] Implement test workflow integration test in `tests/integration/nframework-nfw/features/workspace_new/build_test_workflow_test.rs`
  - Uses workspace generated by T017's test fixture setup
  - Runs `make test` from workspace root
  - Validates exit code 0 and all tests pass
  - Validates test results reported with counts
  - Cleans up generated workspace
  - Note: Adds to same file as T017 as separate `#[test]` function

- [x] T019 [US2] Implement build/test failure reporting integration test in `tests/integration/nframework-nfw/features/workspace_new/build_test_workflow_test.rs`
  - Creates workspace with intentionally broken project
  - Runs `make build` and validates non-zero exit
  - Validates error output identifies failing project
  - Validates actionable error guidance present

### Implementation for User Story 2

- [x] T020 [US2] Verify generated workspace `Makefile` includes `build` and `test` targets — this is a validation task confirming spec `002-workspace-structure-new-command` template content satisfies FR-007/FR-008; if missing, file cross-repo issue against that spec's template repository
  - **Result**: Blank workspace template does NOT include a Makefile. Only `src/`, `nfw.yaml`, and `README.md` are generated. Cross-repo issue needed.
- [x] T021 [US2] Verify generated workspace documentation indicates build and test commands — this is a validation task confirming spec `002-workspace-structure-new-command` template README content satisfies FR-011; if missing, file cross-repo issue against that spec's template repository
  - **Result**: README does NOT document `make build` or `make test` commands. Only documents `nfw add service`. Cross-repo issue needed.

**Checkpoint**: US2 works independently. Generated workspaces build and test on first run.

---

## Phase 5: User Story 3 - Performance Benchmark for Workspace and Service Creation (Priority: P2)

**Goal**: Implement benchmark harness measuring workspace and service creation performance against the <1 second SC-001 target.

**Independent Test**: Run `scripts/benchmark/run_benchmark.sh` on baseline hardware and verify workspace + service creation completes in under 1 second.

### Tests for User Story 3

- [x] T022 [P] [US3] Implement benchmark harness timing logic in `scripts/benchmark/run_benchmark.sh`
  - Captures wall-clock time using `date +%s%N` (nanoseconds)
  - Runs workspace creation benchmark (`nfw new`)
  - Runs service creation benchmark (`nfw add service`)
  - Runs combined benchmark (workspace + service) validating SC-001 target: total < 1 second on baseline hardware (2 CPU cores, 4GB RAM)
  - Supports --iterations flag for multiple runs

- [x] T023 [P] [US3] Implement benchmark statistics computation (median, p95, min, max, mean) in `scripts/benchmark/run_benchmark.sh`
  - Computes statistics across iterations
  - Compares p95 against target_ms (1000ms)
  - Sets passed/fail status based on comparison

- [x] T024 [US3] Implement benchmark JSON result output in `scripts/benchmark/run_benchmark.sh`
  - Outputs results to stdout in JSON format
  - Saves results to `benchmark-results.json`
  - Includes environment metadata from T008
  - Matches `contracts/benchmark-result-schema.yaml` schema

### Implementation for User Story 3

- [x] T025 [US3] Add benchmark target to `Makefile` as `make benchmark` in `src/nfw/Makefile`
- [x] T026 [US3] Add benchmark documentation to `src/nfw/docs/BENCHMARK.md` with usage, interpretation, and CI integration guidance

**Checkpoint**: US3 delivers automated performance validation against SC-001 target.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Final consistency checks, CI integration, and acceptance validation.

- [x] T027 [P] Add smoke test CI configuration for PR and merge gates in `.github/workflows/smoke-tests.yml`
- [x] T028 [P] Add benchmark CI configuration for merge-to-main runs in `.github/workflows/benchmarks.yml`
- [x] T029 Align quickstart documentation with actual test and benchmark commands in `src/nfw/specs/005-build-test-workflows/quickstart.md`
- [x] T030 Run full smoke test suite and document acceptance results
- [x] T031 Run benchmark harness on available hardware and record baseline results

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies.
- **Phase 2 (Foundational)**: Depends on Phase 1; blocks all user stories.
- **Phase 3 (US1)**: Depends on Phase 2 (shared test utilities, runner).
- **Phase 4 (US2)**: Depends on Phase 2 (fixtures); depends on US1 completion for workspace generation validation.
- **Phase 5 (US3)**: Depends on Phase 2 (benchmark harness entry point, env metadata).
- **Phase 6 (Polish)**: Depends on completion of selected user stories.

### User Story Dependencies

- **US1 (P1)**: First MVP slice; depends only on foundational test infrastructure.
- **US2 (P1)**: Depends on US1 command baseline (workspace generation must work before build/test can be validated).
- **US3 (P2)**: Independent of US1/US2 implementation; depends only on foundational benchmark infrastructure.

### Within Each User Story

- Tests first (or early), then implementation/wiring.
- Validation logic before final integration.
- Story checkpoint must pass independently.

## Parallel Opportunities

- Setup: T002, T003, T004 can run in parallel after T001 directory created.
- Foundational: T005, T006, T007, T008, T009, T010 can run in parallel.
- US1: T011, T012, T013 can run in parallel; T014, T015, T016 sequential after tests.
- US2: T017, T018 can run in parallel; T019 after T017/T018; T020, T021 after T019.
- US3: T022, T023 can run in parallel; T024 after T022/T023; T025, T026 after T024.
- Polish: T027, T028 can run in parallel.

---

## Parallel Example: User Story 1

```bash
# Parallelizable US1 test tasks
Task: "T011 [US1] Implement template selection smoke test in tests/smoke/template_selection_test.sh"
Task: "T012 [US1] Implement workspace generation smoke test in tests/smoke/workspace_generation_test.sh"
Task: "T013 [US1] Implement service scaffolding smoke test in tests/smoke/service_scaffolding_test.sh"
```

## Parallel Example: User Story 2

```bash
# Parallelizable US2 test tasks
Task: "T017 [US2] Implement build workflow integration test in tests/integration/nframework-nfw/features/workspace_new/build_test_workflow_test.rs"
Task: "T018 [US2] Implement test workflow integration test in tests/integration/nframework-nfw/features/workspace_new/build_test_workflow_test.rs"
```

## Parallel Example: User Story 3

```bash
# Parallelizable US3 tasks
Task: "T022 [US3] Implement benchmark harness timing logic in scripts/benchmark/run_benchmark.sh"
Task: "T023 [US3] Implement benchmark statistics computation in scripts/benchmark/run_benchmark.sh"
```

---

## Implementation Strategy

### MVP First (User Story 1)

1. Complete Phase 1 (Setup)
2. Complete Phase 2 (Foundational)
3. Complete Phase 3 (US1)
4. Validate US1 independently via `make smoke-tests`

### Incremental Delivery

1. Deliver US1 (smoke tests for core CLI workflows)
2. Deliver US2 (single-command build/test validation)
3. Deliver US3 (benchmark harness with SC-001 target)
4. Complete Polish for CI integration and documentation

### Team Parallel Strategy

1. One engineer handles smoke test infrastructure and US1 tests.
2. One engineer handles integration tests and US2 build/test workflows.
3. One engineer handles benchmark harness and US3 timing/statistics.
4. Merge at story checkpoints only.

---

## Notes

- All tasks follow strict checklist format with IDs, labels, and file paths.
- `[P]` marks tasks that can run independently on disjoint files.
- Story phases preserve independent testability and incremental delivery.
- Suggested MVP scope: **Phase 1 + Phase 2 + Phase 3 (US1)**.
- Smoke tests require `nfw` CLI to be built and available before execution.
- Benchmark results are machine-readable JSON suitable for CI artifact storage and trend analysis.
