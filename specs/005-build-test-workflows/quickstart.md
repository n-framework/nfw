# Quickstart: Build & Test Workflows

## Smoke Tests

Run all smoke tests:

```bash
cd src/nfw
make smoke-tests
```

Run individual smoke test suites:

```bash
# Template selection smoke test
./tests/smoke/template_selection_test.sh

# Workspace generation smoke test
./tests/smoke/workspace_generation_test.sh

# Service scaffolding smoke test
./tests/smoke/service_scaffolding_test.sh
```

Expected output on success:

```bash
Smoke Test Suite: Build & Test Workflows
=========================================
[PASS] Template selection (non-interactive)
[PASS] Workspace generation
[PASS] Service scaffolding
=========================================
3/3 tests passed
```

### Prerequisites

- `nfw` CLI built and available in PATH
- Template cache pre-populated (`nfw templates` shows at least one template)
- .NET SDK installed (for service compilation validation)
- `make` available

## Build & Test Workflow Validation

After generating a workspace, verify it builds and tests with single commands:

```bash
# Generate a workspace
nfw new TestWorkspace --template <id> --no-input
cd TestWorkspace

# Build all projects
make build

# Run all tests
make test
```

Both commands must succeed on first run without manual file edits.

## Benchmark Harness

Run the performance benchmark:

```bash
cd src/nfw
./scripts/benchmark/run_benchmark.sh
```

Run with custom iteration count:

```bash
./scripts/benchmark/run_benchmark.sh --iterations 20
```

Run specific benchmark:

```bash
./scripts/benchmark/run_benchmark.sh --test workspace_creation
./scripts/benchmark/run_benchmark.sh --test service_creation
./scripts/benchmark/run_benchmark.sh --test combined
```

### Benchmark Output

Results are printed to stdout in JSON format and saved to `benchmark-results.json`:

```json
{
  "test_name": "workspace_and_service_creation",
  "iterations": 10,
  "statistics": {
    "median_ms": 850,
    "p95_ms": 887,
    "min_ms": 823,
    "max_ms": 891,
    "mean_ms": 853
  },
  "target_ms": 1000,
  "passed": true,
  "environment": {
    "cpu_cores": 2,
    "ram_mb": 4096,
    "os": "linux-x86_64",
    "disk_type": "ssd",
    "cli_version": "0.1.0"
  },
  "timestamp": "2026-04-07T14:30:00Z"
}
```

### Performance Target

The SC-001 target is **workspace + service creation in under 1 second** on baseline hardware (2 CPU cores, 4GB RAM). The benchmark passes when `p95_ms <= 1000`.

### Interpreting Results

- **passed: true**: Performance target met. No action needed.
- **passed: false**: Performance target exceeded. Check `statistics` for which percentile failed. Compare `environment` to baseline hardware. Investigate if running on equivalent hardware.
