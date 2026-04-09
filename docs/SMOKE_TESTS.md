# Smoke Tests

## Overview

The nfw CLI includes a comprehensive smoke test suite that validates core workflows: template selection, workspace generation, and service scaffolding. Smoke tests are designed to be deterministic, isolated, and fast.

## Running Smoke Tests

### Prerequisites

Before running smoke tests, ensure:

- `nfw` CLI is built and available in PATH
- Template cache is populated (`nfw templates` shows at least one template)
- `.NET SDK` is installed (for service compilation validation)
- `make` is available

### Run All Tests

```bash
cd src/nfw
make smoke-tests
```

### Run Individual Tests

```bash
# Template selection smoke test
./tests/smoke/template_selection_test.sh

# Workspace generation smoke test
./tests/smoke/workspace_generation_test.sh

# Service scaffolding smoke test
./tests/smoke/service_scaffolding_test.sh
```

### Run with Verbose Output

```bash
cd src/nfw
bash tests/smoke/run_smoke_tests.sh --verbose
```

## Expected Output

### Success

```bash
Smoke Test Suite: Build & Test Workflows
=========================================
[PASS] Template selection (non-interactive)
[PASS] Workspace generation
[PASS] Service scaffolding
=========================================
3/3 tests passed
```

### Failure

```bash
Smoke Test Suite: Build & Test Workflows
=========================================
[PASS] Template selection (non-interactive)
[FAIL] Workspace generation
  Expected: src/ directory exists
  Actual: src/ not found
  Workspace path: /tmp/nfw-smoke-abc123/
=========================================
2/3 tests passed, 1 failed
```

## Exit Codes

| Code | Meaning                   |
| ---- | ------------------------- |
| 0    | All smoke tests passed    |
| 1    | One or more tests failed  |
| 2    | Environment setup failure |

## Troubleshooting

### "nfw CLI not found in PATH"

**Error**: `nfw CLI not found in PATH`

**Solution**: Build the CLI first:
```bash
cd src/nfw
cargo build --workspace
export PATH="$PATH:$PWD/target/debug"
```

### "Template cache is empty or inaccessible"

**Error**: Template-related failures during smoke tests

**Solution**: Populate the template cache:
```bash
nfw templates refresh
```

### ".NET SDK not found"

**Error**: `.NET SDK not found` or similar

**Solution**: Install .NET 8.0 SDK:
```bash
# macOS
brew install dotnet-sdk

# Linux
wget https://dot.net/v1/dotnet-install.sh
bash dotnet-install.sh --channel 8.0
```

### "No test files found"

**Error**: `No test files found in tests/smoke/`

**Solution**: Ensure smoke test scripts exist:
```bash
ls tests/smoke/*_test.sh
# Should show: error_path_test.sh, service_scaffolding_test.sh, etc.
```

### Cleanup Failures

**Warning**: `Failed to clean up test directory`

**Solution**: Manual cleanup may be required:
```bash
# Remove test directories manually
rm -rf /tmp/nfw-smoke-*
```

### Test Timeout

**Error**: Tests hang or timeout

**Solution**:
1. Check network connectivity (template downloads)
2. Verify template cache is populated
3. Check disk space availability
4. Run with `--verbose` flag for more details

## Test Details

### Template Selection Test

Validates that `nfw new` with `--template` and `--no-input` flags works correctly:

- Workspace directory is created
- `nfw.yaml` exists at workspace root
- Template identifier is recorded in configuration

### Workspace Generation Test

Validates that workspace generation produces the documented structure:

- `src/` directory created
- `tests/` directory created
- `docs/` directory created
- `nfw.yaml` configuration present
- `Makefile` present

### Service Scaffolding Test

Validates that `nfw add service` creates a valid service structure:

- Service directory created under `src/services/`
- Service project file present
- Four-layer structure (if applicable)

## Test Isolation

Each smoke test:

1. Creates a unique temporary directory using `mktemp -d`
2. Executes CLI commands within that directory
3. Registers a cleanup trap to remove the directory on exit
4. Never depends on state from other tests

This ensures:

- No cross-test contamination
- Parallel test execution is safe
- Clean environment for each test run

## CI Integration

Smoke tests run automatically on:

- Every pull request to main
- Every merge to main

See [`.github/workflows/test-suite.yml`](../../.github/workflows/test-suite.yml) for configuration.
