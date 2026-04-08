# Smoke Test Contract

## Purpose

Defines the expected input, execution behavior, and output contract for CLI smoke tests validating template selection, workspace generation, and service scaffolding.

## Test Execution Contract

### Environment Requirements

- `nfw` CLI installed and available in PATH
- Template cache pre-populated with at least one valid template
- Required toolchain available (Rust for CLI, .NET SDK for service compilation)
- Temporary directory writable (`TMPDIR` or system default)

### Test Isolation

Each smoke test MUST:

1. Create a unique temporary directory using `mktemp -d`
2. Execute all CLI commands within that directory
3. Register a cleanup trap to remove the directory on exit:
   ```bash
   cleanup() { rm -rf "$TEST_DIR"; }
   trap cleanup EXIT
   ```
4. Never depend on state from other tests

### Exit Code Semantics

| Exit Code | Meaning                                                          |
| --------- | ---------------------------------------------------------------- |
| 0         | All smoke test scenarios passed                                  |
| 1         | One or more smoke test scenarios failed                          |
| 2         | Environment setup failure (missing toolchain, no template cache) |

### Output Contract

**Success output (stdout)**:

```
Smoke Test Suite: Build & Test Workflows
=========================================
[PASS] Template selection (non-interactive)
[PASS] Workspace generation
[PASS] Service scaffolding
=========================================
3/3 tests passed
```

**Failure output (stderr)**:

```
Smoke Test Suite: Build & Test Workflows
=========================================
[PASS] Template selection (non-interactive)
[FAIL] Workspace generation
  Expected: src/ directory exists
  Actual: src/ not found
  Workspace path: /tmp/nfw-smoke-abc123/
[PASS] Service scaffolding
=========================================
2/3 tests passed, 1 failed
```

## Test Scenarios

### T1: Template Selection (Non-Interactive)

**Input**: `nfw new TestWorkspace --template <id> --no-input`
**Validation**:

- Command exits with status 0
- Workspace directory created
- `nfw.yaml` exists at workspace root
- Template identifier recorded in configuration

### T2: Workspace Generation

**Input**: `nfw new TestWorkspace --template <id> --no-input`
**Validation**:

- Command exits with status 0
- Workspace contains documented required directories (`src/`, `tests/`, `docs/`)
- Baseline configuration files present
- No manual edits required for build

### T3: Service Scaffolding

**Input**: `nfw add service TestService --lang dotnet` (within generated workspace)
**Validation**:

- Command exits with status 0
- Service directory created with four layers (Domain, Application, Infrastructure, Api)
- Service compiles successfully
- Project references follow layer dependency rules

### T4: Build Workflow

**Input**: `make build` (within generated workspace)
**Validation**:

- Command exits with status 0
- All projects compile without errors
- No warnings suppressed to achieve success

### T5: Test Workflow

**Input**: `make test` (within generated workspace)
**Validation**:

- Command exits with status 0
- All tests pass
- Test results reported with counts
