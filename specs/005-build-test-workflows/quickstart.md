# Quickstart: Build & Test Workflows

This guide helps you run existing tests and add new tests to the NFramework codebase.

## Running Tests

### Smoke Tests

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

## Adding Tests for Contributors

### Adding a New Smoke Test

Smoke tests validate core CLI workflows in `tests/smoke/`.

1. **Create test script**:

```bash
cd tests/smoke
cp workspace_generation_test.sh your_feature_test.sh
chmod +x your_feature_test.sh
```

### Implement test logic

```bash
#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
source "$SCRIPT_DIR/common.sh"

TEST_NAME="Your Feature Test"

main() {
    echo "Running: $TEST_NAME"

    setup_test_dir
    cd "$TEST_DIR"

    # Your test logic here
    nfw new TestWorkspace --template "official/blank-workspace" --no-input
    assert_directory_exists "TestWorkspace"

    log_pass "Feature test passed"
    cleanup_test_dir
}

main "$@"
```

**Available helpers** from `common.sh`

- `setup_test_dir` / `cleanup_test_dir` - Directory management
- `assert_file_exists` / `assert_directory_exists` - Assertions
- `assert_file_contains` - Pattern matching
- `log_pass` / `log_fail` / `log_info` - Logging

### Adding an Integration Test

Integration tests are Rust tests in `tests/integration/`.

**Create test file** in feature directory:

```bash
cd tests/integration/nframework-nfw/features/your_feature
touch your_test.rs
```

**Add documented test function**:

```rust
/// Tests that your feature works correctly.
///
/// This validates feature behavior under normal conditions.
/// The test:
/// 1. Sets up test environment
/// 2. Executes the feature
/// 3. Validates expected outcome
///
/// # Success Criteria
/// - Feature completes successfully
/// - Expected side effects occur
#[test]
fn test_your_feature_works() {
    let workspace_root = support::create_workspace_root("test-name");

    // Test implementation
    let output = std::process::Command::new("nfw")
        .args(["your", "command"])
        .current_dir(&workspace_root)
        .output()
        .expect("Command should execute");

    assert!(output.status.success(), "Command should succeed");
    support::cleanup_sandbox_directory(&workspace_root);
}
```

**Use support modules** when available:

```rust
#[path = "support.rs"]
mod support;

use support::{create_workspace_root, cleanup_sandbox_directory};
```

### Test Best Practices

**Smoke Tests:**

- Isolate: Clean up after yourself
- Validate: Test both success and failure paths
- Document: Add clear comments
- Idempotent: Same results on repeated runs
- Fast: Complete in < 30 seconds

**Integration Tests:**

- Document: Add doc comments with purpose and success criteria
- Cleanup: Always clean up temporary directories
- Assert: Use descriptive assertion messages
- Organize: Group related tests in feature directories

### Common Patterns

**Setup and Cleanup:**

```rust
let sandbox = create_sandbox_directory("test-name");
// ... test code ...
cleanup_sandbox_directory(&sandbox);
```

**Command Execution:**

```rust
let output = std::process::Command::new("nfw")
    .args(["new", "Workspace", "--template", "official/blank-workspace"])
    .current_dir(&sandbox)
    .output()
    .expect("Command should execute");
```

**File Validation:**

```rust
assert!(path.is_file(), "File should exist: {}", path.display());
let content = fs::read_to_string(path)?;
assert!(content.contains("expected"), "Should contain expected content");
```

### Troubleshooting

**Tests fail locally:**

1. Build CLI: `cargo build --workspace`
2. Add to PATH: `export PATH="$PATH:$PWD/target/debug"`
3. Check templates: `nfw templates`
4. Check .NET: `dotnet --version`

**Cleanup failures:**

```bash
# Manual cleanup
rm -rf /tmp/nfw-smoke-*
rm -rf /tmp/nfw-test-*
```
