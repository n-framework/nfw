# Quickstart: nfw add persistence Command

**Feature**: 009-add-persistence-command
**Date**: 2026-04-29

## Overview

This quickstart provides executable commands for building, running, and testing the `nfw add persistence` command in the nfw CLI workspace.

## Prerequisites

- Rust 1.85+ (2024 edition)
- Cargo workspace: `src/nfw/`
- Valid nfw workspace for testing

## Build Commands

### Build the entire workspace

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo build --workspace
```

### Build only the CLI binary

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo build --bin n-framework-nfw-cli
```

### Build with optimizations (release mode)

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo build --workspace --release
```

**Expected Output**:

```Compiling n-framework-nfw-core-domain v0.1.0
   Compiling n-framework-nfw-core-application v0.1.0
   Compiling n-framework-nfw-cli v0.1.0
    Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

## Test Commands

### Run all tests

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo test --workspace
```

### Run only persistence command tests

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo test --package n-framework-nfw-cli persistence_add
```

### Run specific test

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo test --package n-framework-nfw-cli test_add_persistence_updates_nfw_yaml_and_renders_generator
```

### Run tests with output

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo test --workspace -- --nocapture
```

**Expected Output**:

```Compiling n-framework-nfw-cli v0.1.0
    Finished test [unoptimized + debuginfo] target(s)
     Running unittests src/...

running 5 tests
test add_persistence_updates_nfw_yaml_and_renders_generator ... ok
test add_persistence_rolls_back_yaml_if_generator_execution_fails ... ok
test add_persistence_fails_if_service_not_found ... ok
test add_persistence_preserves_comments_in_nfw_yaml ... ok
test add_persistence_detects_existing_persistence_module ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Run Commands

### Run from workspace root (development)

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo run --bin n-framework-nfw-cli -- add persistence --help
```

### Run compiled binary directly

```bash
# After building
/home/ac/Code/n-framework/src/nfw/target/debug/n-framework-nfw-cli add persistence --help
```

### Run with explicit service

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo run --bin n-framework-nfw-cli -- add persistence --service MyService
```

### Run in automated mode

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo run --bin n-framework-nfw-cli -- add persistence --service MyService --no-input
```

## Development Workflow

### 1. Make code changes

Edit files in:

```text
src/nfw/src/n-framework-nfw/
├── core/n-framework-nfw-core-application/src/features/generator_management/
│   └── commands/add_persistence/
└── presentation/n-framework-nfw-cli/src/commands/add/persistence/
```

### 2. Build and test

```bash
cd /home/ac/Code/n-framework/src/nfw

# Build
cargo build --workspace

# Run tests
cargo test --workspace persistence_add

# Run linter
cargo clippy --workspace -- -D warnings

# Format code
cargo fmt --all
```

### 3. Manual testing

```bash
# Create a test workspace
mkdir -p /tmp/test-nfw-workspace
cd /tmp/test-nfw-workspace

# Initialize workspace (if you have nfw new command)
# Or manually create nfw.yaml

# Run the command
/home/ac/Code/n-framework/src/nfw/target/debug/n-framework-nfw-cli add persistence --service MyService
```

## Integration Test Setup

### Create test sandbox manually

```bash
# Create test directory
mkdir -p /tmp/persistence-test/{generators/dotnet-service/persistence,src/MyService}

# Create nfw.yaml
cat > /tmp/persistence-test/nfw.yaml << 'EOF'
workspace:
  name: Test
  namespace: TestApp
services:
  MyService:
    path: src/MyService
    generator:
      id: dotnet-service
generator_sources:
  local: "generators"
EOF

# Create nfw.generator.yaml
cat > /tmp/persistence-test/generators/dotnet-service/persistence/nfw.generator.yaml << 'EOF'
id: dotnet-service/persistence
steps:
  - action: render
    source: DbContext.cs.tera
    destination: Infrastructure/Persistence/{{ Name }}DbContext.cs
EOF

# Create minimal generator
echo "// DbContext for {{ Name }}" > /tmp/persistence-test/generators/dotnet-service/persistence/DbContext.cs.tera

# Run command from test directory
cd /tmp/persistence-test
/home/ac/Code/n-framework/src/nfw/target/debug/n-framework-nfw-cli add persistence --service MyService --no-input
```

### Verify results

```bash
# Check nfw.yaml was updated
cat /tmp/persistence-test/nfw.yaml
# Should show "persistence" in MyService.modules array

# Check generator was rendered
ls -la /tmp/persistence-test/src/MyService/Infrastructure/Persistence/
# Should show MyServiceDbContext.cs
```

## Debugging

### Enable debug logging

```bash
cd /home/ac/Code/n-framework/src/nfw
RUST_LOG=debug cargo run --bin n-framework-nfw-cli -- add persistence --service MyService
```

### Run with debugger

```bash
cd /home/ac/Code/n-framework/src/nfw
rust-lldb -- cargo run --bin n-framework-nfw-cli -- add persistence --service MyService
```

### Check for clippy warnings

```bash
cd /home/ac/Code/n-framework/src/nfw
cargo clippy --workspace -- -D warnings
```

## File Locations After Build

### Binary

```text
src/nfw/target/debug/n-framework-nfw-cli # Debug build
src/nfw/target/release/n-framework-nfw-cli         # Release build
```

### Test Artifacts

```text
src/nfw/target/debug/deps/libn_framework_nfw_cli-*.rlib

```

## Common Issues

### Issue: "Generator not found"

**Solution**: Ensure persistence generator exists in configured generator sources.

```bash
# Check generator sources in nfw.yaml
grep -A 10 "generator_sources:" nfw.yaml

# Verify generator directory exists
ls generators/dotnet-service/persistence/
```

### Issue: "Service not found"

**Solution**: Verify service is defined in nfw.yaml.

```bash
# Check services
grep -A 20 "services:" nfw.yaml
```

### Issue: Tests fail with "workspace not found"

**Solution**: Tests use sandbox directories. Ensure test setup is correct.

```bash
# Check test support module
cat tests/integration/n-framework-nfw/features/service_add/support.rs
```

## Continuous Integration

### Run full CI locally

```bash
cd /home/ac/Code/n-framework/src/nfw

# Format check
cargo fmt --all -- --check

# Linter check
cargo clippy --workspace -- -D warnings

# Build check
cargo build --workspace

# Test check
cargo test --workspace
```

All commands must exit with code 0 for CI to pass.

## Next Steps

After implementing the command:

1. ✅ Verify all tests pass
2. ✅ Check clippy warnings: `cargo clippy --workspace -- -D warnings`
3. ✅ Run integration tests manually
4. ✅ Update this quickstart if any commands change
5. ✅ Commit changes with conventional commit message

## Performance Validation

### Measure execution time

```bash
cd /home/ac/Code/n-framework/src/nfw
time cargo run --bin n-framework-nfw-cli -- add persistence --service MyService --no-input
```

**Expected**: <5 seconds for typical workspaces

### Measure rollback time

```bash
# Create a failing generator (invalid syntax)
# Run command
time cargo run --bin n-framework-nfw-cli -- add persistence --service MyService --no-input
```

**Expected**: <1 second to fail and rollback
