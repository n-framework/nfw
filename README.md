# nfw CLI

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Buy A Coffee](https://img.shields.io/badge/Buy%20a%20Coffee-ffdd00?logo=buy-me-a-coffee&logoColor=black&style=flat)](https://ahmetcetinkaya.me/donate)

The `nfw` CLI is the command-line entry point for the NFramework toolchain. It creates workspaces from templates, scaffolds services and CRUD flows, and validates architecture boundaries.

---

## Quick Start

### Installation

```bash
git clone https://github.com/n-framework/n-framework.git
cd n-framework
make build
```

### Basic Usage

```bash
# List available templates
nfw templates

# Create a new workspace
nfw new my-workspace

# Create non-interactively in CI
nfw new --no-input my-workspace --template blank
```

---

## Commands

### Workspace & Templates

```bash
nfw templates                                # List available templates
nfw new [name]                               # Create a new workspace from a template
nfw new [name] --template <id>               # Create with an explicit template
nfw new --no-input [name] --template <id>    # Non-interactive (for CI)
```

### Service & Code Generation

```bash
nfw add service <name> --lang go             # Add a Go service scaffold
nfw add service <name> --lang rust           # Add a Rust service scaffold
nfw add entity <name> --props <properties>   # Generate entity class only
nfw add crud <entity-name>                   # Generate full CRUD for an existing entity
nfw add command <name> <feature>             # Generate standalone command handler
nfw add query <name> <feature>               # Generate standalone query handler
```

### Generation Options (entity, crud, command, query)

- `--cached` — Enable caching behavior in generated handlers.
- `--logged` — Enable logging behavior in generated handlers.
- `--transactional` — Wrap handler execution in a transaction.
- `--secured` — Mark operation as requiring authentication.
- `--no-api` — Skip HTTP endpoint generation.

### Architecture Validation

```bash
nfw check                                    # Validate architecture boundaries
nfw check --verbose                          # Validate with detailed diagnostics
```

---

### Template Catalog

| Identifier | Display Name    | Description                      |
| ---------- | --------------- | -------------------------------- |
| `blank`    | Blank Workspace | Minimal starter workspace        |
| `minimal`  | Minimal API     | Starter with a minimal API focus |

---

## Templates Source

- Debug builds use `packages/nfw-templates` when the submodule exists.
- If the debug submodule is missing, debug builds fall back to release behavior.
- Release builds fetch templates from `github.com/n-framework/nfw-templates` at tag `v{cliVersion}`.

---

## Configuration

- Optional config file: `nfw.yaml` in the current working directory.
- Environment variables with `NFW_` prefix override file keys.
- Invalid YAML prints an error and the CLI continues with defaults.

---

## Exit Codes

| Code | Meaning              |
| ---- | -------------------- |
| 0    | Success              |
| 1    | Runtime failure      |
| 2    | Usage error          |
| 130  | Interrupted (SIGINT) |

---

## License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.
