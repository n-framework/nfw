# Quickstart: `nfw new` Workspace Initialization

## 1. Create a workspace in non-interactive mode

```bash
nfw new BillingPlatform --template official/blank-workspace --no-input
```

Expected behavior:

- Creates workspace with layered root: `src/`, `tests/`, `docs/`
- Renders template-defined files/directories from selected template content
- Uses YAML baseline configuration files only

## CLI help contract

```bash
nfw new --help
```

Expected output includes:

- `Usage: nfw new [OPTIONS] [workspace-name]`
- `--template <template>`
- `--no-input`

## 2. Create a workspace in interactive mode

```bash
nfw new
```

Expected behavior:

- Prompts for required missing values
- Resolves template and namespace values
- Generates workspace only after validation is complete

## 3. Verify structure quickly

```bash
cd BillingPlatform
ls
```

Root should include:

- `src/`
- `tests/`
- `docs/`
- template-defined root-level artifacts (for blank template: `README.md`, `nfw.yaml`, manifests)

## 4. Verify build and test commands

Run the workspace-documented one-command flows:

```bash
# build command documented in generated workspace
# test command documented in generated workspace
```

## 5. Validate strict failure cases

### Case A: Non-empty target directory

`nfw new` must fail immediately if destination exists and contains files.

### Case B: Missing required input with `--no-input`

Command must fail before generation and list missing required values.

### Case C: Invalid template

Command must fail with actionable message and template selection guidance.

## 6. Acceptance verification commands

Commands used to validate this feature implementation:

```bash
cd src/nfw
cargo test -p nframework-nfw-domain --test workspace_blueprint_tests
cargo test -p nframework-nfw-application --test input_resolution_service_tests
cargo test -p nframework-nfw-infrastructure-filesystem --test workspace_layout_test --test reproducible_generation_test
cargo test -p n-framework-nfw-cli --test new_command_routing_tests --test no_input_validation_test --test cli_routing_errors_test
make format
make lint
```
