# Quickstart: `nfw new` Workspace Initialization

## 1. Create a workspace in non-interactive mode

```bash
nfw new BillingPlatform --template official/microservice --no-input
```

Expected behavior:

- Creates workspace with layered root: `src/`, `tests/`, `docs/`
- Creates root-level solution/config files
- Creates per-service solution files according to naming rules
- Uses YAML baseline configuration files only

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
- root-level solution/config files

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
