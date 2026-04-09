# Research: Build & Test Workflows

## 1. Smoke Test Isolation Strategy

**Decision**: Use isolated temporary directories for each smoke test execution.

**Rationale**:

- Prevents cross-test contamination from leftover files
- Enables parallel test execution
- Matches existing test patterns in `src/nfw/tests/`
- Shell tests can use `mktemp -d` for directory creation and `trap` for cleanup

**Implementation**: Each smoke test creates a unique temp directory, runs CLI commands within it, validates outputs, and cleans up on exit (success or failure).

## 2. Benchmark Harness Design

**Decision**: Implement as a shell script wrapper around CLI invocations with JSON result output.

**Rationale**:

- Shell scripts can capture wall-clock time accurately using `date +%s%N` (nanoseconds)
- JSON output enables CI artifact storage and trend analysis
- Simpler than integrating with `cargo bench` which is designed for microbenchmarks
- Can run multiple iterations and compute statistics (median, p95)

**Alternative considered**: Rust-based benchmark using `criterion` crate - rejected because we need end-to-end CLI timing including process startup, not just function-level benchmarks.

## 3. Machine-Readable Benchmark Output Format

**Decision**: JSON schema with timing metrics, environment metadata, and pass/fail status.

**Schema fields**:

- `test_name`: Identifier for the benchmark (e.g., "workspace_creation", "service_creation")
- `iterations`: Number of runs performed
- `results`: Array of individual run times in milliseconds
- `statistics`: Object with `median_ms`, `p95_ms`, `min_ms`, `max_ms`, `mean_ms`
- `target_ms`: The performance target in milliseconds (1000ms for SC-001)
- `passed`: Boolean indicating whether target was met
- `environment`: Object with `cpu_cores`, `ram_mb`, `os`, `disk_type`, `cli_version`
- `timestamp`: ISO 8601 timestamp of benchmark execution

## 4. CI Integration for Smoke Tests and Benchmarks

**Decision**:

- Smoke tests run on every PR and merge to main
- Benchmarks run on merge to main only (to avoid noise from variable CI hardware)
- Benchmark results stored as CI artifacts for trend analysis
- Dedicated CI runner with baseline hardware profile (2 CPU cores, 4GB RAM) for benchmarks

**Rationale**: Smoke tests are fast and deterministic, suitable for PR gates. Benchmarks require consistent hardware for meaningful comparisons.

## 5. Template Cache Management for Tests

**Decision**:

- Smoke tests use pre-populated template cache to avoid network-dependent timing
- Unit tests mock template operations for fast feedback
- Integration tests use real git clone but are clearly isolated and labeled

**Rationale**: Network operations introduce variability and flakiness. Pre-populated cache ensures deterministic test execution. Unit tests with mocks provide fast feedback during development.

## Implementation Decisions Summary

| Question         | Decision                                             |
| ---------------- | ---------------------------------------------------- |
| Test isolation   | Temporary directories with cleanup traps             |
| Benchmark tool   | Shell script wrapper with JSON output                |
| Benchmark format | JSON with timing, stats, environment metadata        |
| CI smoke tests   | Run on every PR and merge                            |
| CI benchmarks    | Run on merge to main only                            |
| Template cache   | Pre-populated for smoke tests, mocked for unit tests |
| Build command    | `make build` (consistent with repo conventions)      |
| Test command     | `make test` (consistent with repo conventions)       |
