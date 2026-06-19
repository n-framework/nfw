# nfw CLI

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Buy A Coffee](https://img.shields.io/badge/Buy%20a%20Coffee-ffdd00?logo=buy-me-a-coffee&logoColor=black&style=flat)](https://ahmetcetinkaya.me/donate)

The `nfw` CLI is the command-line entry point for the NFramework toolchain. It creates workspaces from generators, scaffolds services and modules, generates mediator commands/queries and full CRUD flows, and validates architecture boundaries.

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
# List available generators
nfw generators list

# Create a new workspace (interactive)
nfw new my-workspace

# Create non-interactively in CI
nfw new my-workspace --generator blank-workspace --no-input
```

---

## Commands

### Workspace

```bash
nfw new [name]                                # Create a new workspace from a generator
nfw new [name] --generator <id>               # Create with an explicit generator
nfw new [name] --generator <id> --no-input    # Non-interactive (for CI)
```

### Generator Sources

```bash
nfw generators list                           # List discovered generators
nfw generators add <name> <url>               # Register a generator source (git URL)
nfw generators remove <name>                  # Unregister a generator source
nfw generators refresh                        # Refresh generator catalogs from sources
```

### Services & Modules

```bash
nfw add service <name>                         # Generate a service from a service generator
nfw add service <name> --generator <id>       # Generate with an explicit service generator
nfw add mediator --service <name>             # Add the mediator module to a service
nfw add persistence --service <name>          # Add the persistence module to a service
nfw add webapi --service <name>               # Add the webapi module to a service
```

### Code Generation

```bash
nfw gen entity <name> --properties <props>    # Generate an entity (schema + files)
nfw gen crud <entity-name> --feature <name>   # Generate complete CRUD scaffolding
nfw gen command <name> --feature <name>       # Generate a mediator command
nfw gen query <name> --feature <name>         # Generate a mediator query
nfw gen endpoint <GET|POST|PUT|DELETE> <name> # Generate a minimal API endpoint mapping
nfw gen repository <name> --feature <name>    # Generate a repository for an entity
```

### Architecture Validation

```bash
nfw check                                      # Validate workspace architecture boundaries
```

---

## Command Options

Most generation commands accept `--no-input` to disable interactive prompts (required for CI).

### `nfw gen entity`

- `--properties <list>` — Comma-separated `Name:Type` pairs (e.g. `Name:string,Price:decimal?,Active:bool`).
- `--id-type <type>` — Primary key type (default `Guid`). Supported: `int`, `long`, `Guid`, `string`.
- `--entity-type <type>` — Base type (default `entity`). Options: `entity`, `auditable_entity`, `soft_deletable_entity`.
- `--from-schema <path>` — Generate from an existing entity schema YAML file.
- `--schema-only` — Only emit the schema YAML; skip generator execution.
- `--service <name>` — Target service for the entity.
- `--feature <name>` — Target feature within the service.

### `nfw gen crud`, `gen command`, `gen query`, `gen endpoint`, `gen repository`

- `--feature <name>` — Target feature or module.
- `--param <list>` — Comma-separated generator parameters (e.g. `secured=true,cached=true,no-api=true`).
- `--param-json <json>` — Generator parameters as a JSON string (e.g. `'{"secured": true, "cached": true}'`).
- `--service <name>` — Target service (`gen repository` only).

Common CRUD parameters: `secured=true`, `cached=true`, `no-api=true`, `force=true`.

```bash
# CRUD with security and caching markers, no API endpoints
nfw gen crud Order --feature Orders --param secured=true,cached=true,no-api=true

# Non-interactive CRUD for CI
nfw gen crud Product --feature Products --no-input
```

---

## Generator Catalog

| Identifier        | Display Name    | Description               |
| ----------------- | --------------- | ------------------------- |
| `blank-workspace` | Blank Workspace | Minimal starter workspace |

Run `nfw generators list` to see all generators discovered from registered sources.

---

## Generator Sources

Generators are resolved in this order:

1. Local path from `generator_sources.local` in the workspace `nfw.yaml`.
2. `~/.nfw/generators` (the primary cache/config directory).
3. `~/.cache/nfw/generators` (legacy cache fallback).

Use `nfw generators add <name> <url>` to register a remote source and `nfw generators refresh` to fetch the latest catalogs.

---

## Exit Codes

| Code | Meaning                      |
| ---- | ---------------------------- |
| 0    | Success                      |
| 1    | Internal error              |
| 2    | Validation error            |
| 3    | Not found                   |
| 4    | Conflict                    |
| 10   | External dependency failure |
| 130  | Interrupted (SIGINT)        |

---

## License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.
