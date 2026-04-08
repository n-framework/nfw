# Data Model: Build & Test Workflows

## Entities

### SmokeTestResult

Represents the outcome of a single smoke test execution.

```yaml
test_name: string # e.g., "workspace_generation_non_interactive"
status: enum # "passed", "failed", "skipped"
duration_ms: integer # Wall-clock time for test execution
output: string # Captured stdout/stderr from CLI commands
artifacts: # Generated artifacts (for debugging failures)
  workspace_path: string # Path to generated workspace (if applicable)
  error_details: string # Failure details (if failed)
timestamp: string # ISO 8601 timestamp
```

### BenchmarkResult

Represents the outcome of a benchmark execution with statistical analysis.

```yaml
test_name: string # e.g., "workspace_and_service_creation"
iterations: integer # Number of benchmark iterations
results: # Individual run times
  - iteration: integer
    duration_ms: integer
statistics:
  median_ms: integer
  p95_ms: integer
  min_ms: integer
  max_ms: integer
  mean_ms: integer
target_ms: integer # Performance target (1000ms for SC-001)
passed: boolean # Whether target was met
environment:
  cpu_cores: integer
  ram_mb: integer
  os: string
  disk_type: string # "ssd", "hdd", "nvme"
  cli_version: string
timestamp: string # ISO 8601 timestamp
```

### BuildTestWorkflow

Defines the single-command build and test workflow for a generated workspace.

```yaml
workspace_root: string # Absolute path to workspace root
build_command: string # e.g., "make build"
test_command: string # e.g., "make test"
expected_exit_code: integer # 0 for success
expected_artifacts: # Files/directories that must exist after build
  - string
expected_test_results: # Test outcome expectations
  total_tests: integer
  passed_tests: integer
  failed_tests: integer
```

### TestFixture

Represents a test workspace fixture for validation.

```yaml
name: string # Fixture identifier
type: enum # "valid", "invalid", "edge_case"
workspace_path: string # Path to fixture directory
description: string # What this fixture validates
expected_behavior: # Expected CLI behavior
  exit_code: integer
  output_contains: # Strings that must appear in output
    - string
  output_not_contains: # Strings that must NOT appear in output
    - string
```
