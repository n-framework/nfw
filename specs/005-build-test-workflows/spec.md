# Feature Specification: Build & Test Workflows

## User Scenarios & Testing

### User Story 1 - Smoke Test End-to-End Workflows (Priority: P1)

As a developer, I want CLI smoke tests that validate template selection, workspace generation, and service scaffolding so that I can verify the CLI works correctly after installation or updates.

**Why this priority**: Smoke tests are the first line of defense against regressions in core user journeys. Without them, broken releases can reach users undetected.

**Independent Test**: Run the smoke test suite and verify it exercises interactive and non-interactive template selection, workspace creation, and service scaffolding without manual intervention.

**Acceptance Scenarios**:

1. **Given** a fresh CLI installation, **When** smoke tests run, **Then** template selection works in both interactive and non-interactive modes
2. **Given** a clean test environment, **When** the workspace generation smoke test runs, **Then** `nfw new` creates a valid workspace with documented structure
3. **Given** a generated workspace, **When** the service scaffolding smoke test runs, **Then** `nfw add service` creates a compilable service with correct layer structure
4. **Given** smoke test results, **When** all scenarios pass, **Then** the test suite exits with status 0 and reports success

---

### User Story 2 - Single-Command Build and Test for Generated Workspaces (Priority: P1)

As a developer, I want generated workspaces to build and test with a single command from the workspace root so that I can verify correctness immediately after generation.

**Why this priority**: Generated code must compile and pass tests on first run. This is the core "it works out of the box" guarantee that makes the framework trustworthy.

**Independent Test**: Generate a workspace, run the documented build command, then run the documented test command, and verify both succeed without manual file edits.

**Acceptance Scenarios**:

1. **Given** a freshly generated workspace, **When** I run the documented build command from the workspace root, **Then** all projects compile successfully without manual edits
2. **Given** a freshly generated workspace, **When** I run the documented test command from the workspace root, **Then** all tests pass without manual configuration
3. **Given** a generated workspace with services, **When** I run build and test commands, **Then** both commands succeed on first run 100% of the time
4. **Given** a build or test failure, **When** the command exits, **Then** the error output identifies the failing project and provides actionable guidance

---

### User Story 3 - Performance Benchmark for Workspace and Service Creation (Priority: P2)

As a platform engineer, I want a benchmark harness that validates workspace and service creation performance so that I can ensure the CLI meets the <1 second target on baseline hardware.

**Why this priority**: Performance is a measurable success criterion (SC-001). Without automated benchmarking, regressions in generation speed can go unnoticed.

**Independent Test**: Run the benchmark harness on baseline hardware (2 CPU cores, 4GB RAM) and verify workspace + service creation completes in under 1 second.

**Acceptance Scenarios**:

1. **Given** the benchmark harness, **When** it runs workspace creation, **Then** it measures and reports the elapsed time for `nfw new` end-to-end
2. **Given** the benchmark harness, **When** it runs service creation, **Then** it measures and reports the elapsed time for `nfw add service` end-to-end
3. **Given** baseline hardware (2 CPU cores, 4GB RAM), **When** the combined workspace + service creation benchmark runs, **Then** the total time is under 1 second
4. **Given** a benchmark result exceeding the target, **When** the harness completes, **Then** it reports a failure with timing details for investigation

## Edge Cases

- **Slow disk I/O**: When disk performance is degraded, benchmark results include environment metadata so that hardware-specific slowdowns can be distinguished from CLI regressions
- **Concurrent test execution**: When smoke tests run in parallel, each test uses an isolated temporary directory to avoid cross-test contamination
- **Partial workspace generation**: When workspace creation is interrupted during smoke testing, the test cleans up partial artifacts before reporting results
- **Template cache miss**: When smoke tests run without a pre-populated template cache, the test accounts for initial clone time separately from generation time
- **Non-deterministic timing**: When benchmark results vary due to system load, the harness runs multiple iterations and reports median and p95 values

## Requirements

### Functional Requirements

#### CLI Smoke Tests

- **FR-001**: The system MUST provide CLI smoke tests that validate template selection in non-interactive mode (`--no-input`); interactive mode validation is covered by the prompt service tests in spec `002-workspace-structure-new-command`
- **FR-002**: The system MUST provide CLI smoke tests that validate workspace generation produces documented folder structure and baseline configuration
- **FR-003**: The system MUST provide CLI smoke tests that validate service scaffolding creates compilable services with correct layer structure
- **FR-004**: Smoke tests MUST run without manual intervention and MUST exit with deterministic pass/fail status
- **FR-005**: Smoke tests MUST use isolated temporary directories to avoid cross-test contamination
- **FR-006**: Smoke tests MUST clean up generated artifacts after execution (success or failure)

#### Single-Command Build and Test Workflows

- **FR-007**: Generated workspaces MUST support a single build command from the workspace root that compiles all projects
- **FR-008**: Generated workspaces MUST support a single test command from the workspace root that runs all tests
- **FR-009**: Build and test commands MUST succeed on first run without manual file edits or configuration
- **FR-010**: Build and test command failures MUST identify the failing project and provide actionable error output to stderr
- **FR-011**: Generated workspace documentation MUST clearly indicate the build and test commands to run

#### Performance Benchmark Harness

- **FR-012**: The system MUST provide a benchmark harness that measures workspace creation time
- **FR-013**: The system MUST provide a benchmark harness that measures service creation time
- **FR-014**: The benchmark harness MUST validate the SC-001 performance target (workspace + service creation < 1 second on baseline hardware: 2 CPU cores, 4GB RAM)
- **FR-015**: The benchmark harness MUST report timing results including median and percentile values across multiple iterations
- **FR-016**: The benchmark harness MUST include environment metadata (CPU cores, RAM, disk type) in results for context
- **FR-017**: Benchmark failures (exceeding target) MUST be reported clearly with timing details for investigation

### Key Entities

- **Smoke Test Suite**: A collection of end-to-end CLI tests validating template selection, workspace generation, and service scaffolding
- **Build Command**: The documented single command that compiles all projects in a generated workspace
- **Test Command**: The documented single command that runs all tests in a generated workspace
- **Benchmark Harness**: A tool that measures and reports CLI generation performance against defined targets
- **Baseline Hardware Profile**: The reference hardware specification (2 CPU cores, 4GB RAM) used for performance targets

## Success Criteria

### Measurable Outcomes

- **SC-001**: CLI smoke tests pass for template selection, workspace generation, and service scaffolding scenarios with 100% success rate
- **SC-002**: Generated workspaces build successfully on first run 100% of the time without manual file edits
- **SC-003**: Generated workspaces pass all tests on first run 100% of the time without manual configuration
- **SC-004**: Workspace and service creation completes with p95 under 1 second on baseline hardware (2 CPU cores, 4GB RAM)
- **SC-005**: Benchmark harness reports consistent results (within 10% variance) across repeated runs on identical hardware

## Assumptions

- Smoke tests run in CI environments that have the required toolchain (Rust, .NET SDK) installed
- Template cache is pre-populated for smoke tests to avoid network-dependent timing
- Benchmark hardware profile (2 CPU cores, 4GB RAM) is available as a CI runner or dedicated test machine
- Generated workspaces use `make build` and `make test` as the documented single-command workflows (consistent with repository conventions)
- Performance targets are measured end-to-end including CLI startup, file generation, and command completion

## Dependencies

- `specs/001-phase1-foundations-core-contracts/spec.md` for SC-001, SC-002, SC-003, SC-005, SC-007 success criteria
- `src/nfw/specs/001-nfw-template-system/spec.md` for template discovery and caching behavior
- `src/nfw/specs/002-workspace-structure-new-command/spec.md` for workspace generation behavior and structure
- `src/nfw/specs/003-add-service-dotnet-template-based/spec.md` for service scaffolding behavior
- `src/nfw/specs/004-nfw-check-validation/spec.md` for architecture validation command behavior

## Clarifications

- Q: Should smoke tests cover all CLI commands or only core workflows? → A: Core workflows only (template selection, workspace creation, service scaffolding). Individual command specs cover their own edge cases.
- Q: Should benchmark results be stored for trend analysis? → A: Yes. Benchmark results should be output in a machine-readable format (JSON) suitable for CI artifact storage and trend analysis.
- Q: Should smoke tests run against real templates or mocked templates? → A: Both. Core smoke tests use real templates for end-to-end validation; isolated unit tests mock template operations for fast feedback.

## Non-Goals

- Defining individual CLI command behavior (covered by respective command specs)
- Defining template authoring workflows or template validation CI
- Defining distributed CI pipeline orchestration or multi-environment deployment
- Defining performance targets for commands other than workspace and service creation
- Defining load testing or stress testing for the CLI under concurrent usage
