# Implementation Plan: Build & Test Workflows

**Branch**: `005-build-test-workflows` | **Date**: 2026-04-07 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `src/nfw/specs/005-build-test-workflows/spec.md`

## Summary

Implement CLI smoke tests for template selection, workspace generation, and service scaffolding; define single-command build and test workflows for generated workspaces; and create a benchmark harness validating the <1 second performance target for workspace + service creation on baseline hardware.

## Technical Context

**Language/Version**: Rust 1.85+ (2024 edition)
**Primary Dependencies**: clap (CLI parsing), serde + serde_yaml (config I/O), tempfile (isolated test directories), custom shell benchmark wrapper
**Storage**: File system (temporary test workspaces, benchmark results as JSON artifacts)
**Testing**: cargo test (unit + integration), shell-based smoke tests, benchmark harness
**Target Platform**: Linux, macOS, Windows (CLI)
**Project Type**: CLI application with test suite and benchmark tooling
**Performance Goals**: Workspace + service creation < 1 second on baseline hardware (2 CPU cores, 4GB RAM)
**Constraints**: Smoke tests must be isolated, deterministic, and clean up after themselves; benchmarks must report machine-readable JSON results
**Scale/Scope**: Test suite covers core CLI workflows; benchmark harness measures generation performance

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

| Principle                            | Status | Notes                                                                              |
| ------------------------------------ | ------ | ---------------------------------------------------------------------------------- |
| I. Single-Step Build And Test        | Pass   | Smoke tests validate single-command build/test workflows for generated workspaces. |
| II. CLI I/O And Exit Codes           | Pass   | Smoke tests verify stdout/stderr separation and deterministic exit codes.          |
| III. No Suppression                  | Pass   | All test failures and benchmark violations reported; no suppression introduced.    |
| IV. Deterministic Tests              | Pass   | Smoke tests use isolated temp directories; unit tests mock network operations.     |
| V. Documentation Is Part Of Delivery | Pass   | Quickstart documents smoke test and benchmark execution.                           |

No constitutional violations identified before Phase 0.

**Note:** Constitutional Principles refer to the core architectural and quality standards defined in the NFramework development guidelines (single-step operations, proper error handling, deterministic behavior, comprehensive documentation).

## Phase 0: Research

Research questions resolved in [research.md](./research.md):

1. Smoke test isolation strategy (temp directories with cleanup traps)
2. Benchmark harness design (shell script wrapper with JSON output)
3. Machine-readable benchmark output format (JSON with timing, stats, environment metadata)
4. CI integration for smoke tests and benchmarks (PR gates vs main-only)
5. Template cache management for tests (pre-populated for smoke tests, mocked for unit tests)

Outcome: all identified ambiguities resolved and encoded as implementation decisions.

## Phase 1: Design & Contracts

Design artifacts produced:

- [data-model.md](./data-model.md)
- [contracts/smoke-test-contract.md](./contracts/smoke-test-contract.md)
- [contracts/benchmark-result-schema.yaml](./contracts/benchmark-result-schema.yaml)
- [quickstart.md](./quickstart.md)

Post-design constitution re-check: **Pass** (all gates remain satisfied).

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/005-build-test-workflows/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── smoke-test-contract.md
│   └── benchmark-result-schema.yaml
└── tasks.md
```

### Source Code (in `src/nfw/`)

```text
src/nfw/
├── tests/
│   ├── smoke/
│   │   ├── template_selection_test.sh
│   │   ├── workspace_generation_test.sh
│   │   └── service_scaffolding_test.sh
│   ├── integration/
│   │   └── nframework-nfw/
│   │       └── features/
│   │           ├── workspace_new/
│   │           │   └── build_test_workflow_test.rs
│   │           └── service_add/
│   │               └── build_test_workflow_test.rs
│   └── fixtures/
│       ├── valid-workspace/
│       └── invalid-workspace/
├── scripts/
│   └── benchmark/
│       ├── run_benchmark.sh
│       └── benchmark_result_schema.json
└── Cargo.toml (benchmark dependencies)
```

**Structure Decision**: Smoke tests live as shell scripts under `tests/smoke/` for end-to-end CLI validation. Integration tests use Rust test harness for programmatic assertions. Benchmark harness is a shell script wrapper around CLI invocations with JSON result output. This matches existing repository conventions where shell utility tests live in `packages/acore-scripts/tests/` and Rust tests under `src/nfw/tests/`.

### CI Integration

```text
.github/workflows/
├── smoke-tests.yml       # Runs on every PR and merge to main
└── benchmarks.yml        # Runs on merge to main only (baseline hardware)
```

**CI Strategy**: Smoke tests execute on every PR and merge gate to catch regressions early. Benchmarks run only on merge to main to avoid noise from variable CI runner hardware. Benchmark results are stored as CI artifacts for trend analysis. Dedicated CI runners with baseline hardware profile (2 CPU cores, 4GB RAM) are required for meaningful benchmark comparisons.

## Phase 2 Preview (for `/speckit.tasks`)

Expected task decomposition themes:

1. Smoke test infrastructure setup (temp directories, cleanup, isolation)
2. Template selection smoke tests (interactive and non-interactive modes)
3. Workspace generation smoke tests (structure validation, YAML config)
4. Service scaffolding smoke tests (layer structure, compilability)
5. Single-command build workflow definition and validation
6. Single-command test workflow definition and validation
7. Benchmark harness implementation (timing, iteration, reporting)
8. CI integration and artifact storage for benchmark results

## Complexity Tracking

No constitution violations requiring justification.
