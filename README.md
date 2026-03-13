# nfw CLI 🚀

The official command-line interface for [NFramework](https://github.com/n-framework/n-framework) — a compile-time-first application framework for building clean architecture microservices.

---

## Overview 🔍

`nfw` streamlines the creation and management of NFramework workspaces and services. Generate production-ready scaffolding, enforce architecture boundaries, and validate your codebase — all from a fast, opinionated CLI.

## Features ✨

- **Instant Scaffolding** — Create workspaces and services in seconds
- **Code Generation** — Generate entities, commands, queries with full CRUD flows
- **Architecture Validation** — Detect and report boundary violations
- **Multi-Language Support** — .NET (full), Go & Rust (structure)
- **CI-Ready** — All commands work non-interactively for automation
- **Template-Based** — Customizable starter templates

---

## Quick Start 🏁

### Installation 📦

```bash
# macOS/Linux (Homebrew)
brew install n-framework/tap/nfw

# macOS/Linux (curl)
curl -fsSL https://get.nframework.com | sh

# Windows (winget)
winget install n-framework.nfw

# Build from source
git clone https://github.com/n-framework/nfw.git
cd nfw
go build -o nfw ./cmd/nfw
```

### Create Your First Workspace

```bash
# List available templates
nfw templates

# Create a new workspace
nfw new MyMicroservices

# Navigate to workspace
cd MyMicroservices

# Add a .NET service
nfw add service Orders --lang dotnet

# Add an entity with CRUD
nfw add entity Product --props Name:string,Price:decimal,Stock:int

# Validate architecture
nfw check

# Build and run
dotnet build
dotnet run --project src/Orders/Api
```

---

## Commands ⚡

### 📋 `nfw templates`

List available starter templates.

```bash
nfw templates
nfw templates --lang dotnet
nfw templates --category minimal
```

### 🆕 `nfw new`

Create a new NFramework workspace.

```bash
nfw new <workspace-name> [--template <id>] [--force]
```

**Example:**

```bash
nfw new MyShop --template minimal
```

Creates:

```
MyShop/
├── src/
├── tests/
├── .nfw/
├── MyShop.sln
└── README.md
```

### 🏗️ `nfw add service`

Add a new service to the workspace.

```bash
nfw add service <name> --lang <dotnet|go|rust>
```

**Example (.NET):**

```bash
nfw add service Orders --lang dotnet
```

Generates four-layer architecture:

- `Orders.Domain` — Entities, value objects, domain events
- `Orders.Application` — Commands, queries, handlers, interfaces
- `Orders.Infrastructure` — Repository implementations, adapters
- `Orders.Api` — HTTP endpoints, configuration

### 📦 `nfw add entity`

Generate an entity with full CRUD flow.

```bash
nfw add entity <name> --props <properties>
```

**Property Syntax:** `Name:Type[:modifier]`

**Modifiers:**

- `?` — nullable
- `!` — required (non-default)
- `[]` — collection

**Example:**

```bash
nfw add entity Product --props Name:string!,Price:decimal,Stock:int?,Tags:string[]
```

Generates:

- Entity class with identity
- Create/Update/Response DTOs
- Repository interface
- Create, Update, Delete commands
- GetById, GetAll, GetPaged queries
- HTTP endpoints for all operations

### 📝 `nfw add command`

Generate a standalone command handler.

```bash
nfw add command <name> <feature> [options]
```

**Options:**

- `--cached` — Enable response caching
- `--logged` — Enable request/response logging
- `--transactional` — Wrap in database transaction
- `--secured` — Require authentication
- `--no-api` — Skip HTTP endpoint generation

**Example:**

```bash
nfw add command ApproveOrder Orders --secured --transactional
```

### 🔍 `nfw add query`

Generate a standalone query handler.

```bash
nfw add query <name> <feature> [options]
```

**Options:** Same as `add command`

**Example:**

```bash
nfw add query GetPendingOrders Orders --cached
```

### ✅ `nfw check`

Validate Clean Architecture boundaries.

```bash
nfw check [--verbose]
```

Detects:

- Forbidden project references
- Infrastructure dependencies in Domain/Application
- Incorrect namespace imports
- Layer boundary violations

**Exit codes:**

- `0` — No violations
- `3` — Architecture violations found

### ⚡ `nfw up`

Start local development environment (post-beta).

```bash
nfw up [--detached]
```

---

## Project Structure 📁

A generated NFramework workspace follows this structure:

```
workspace-name/
├── src/
│   ├── ServiceName.Domain/
│   │   ├── Entities/
│   │   ├── ValueObjects/
│   │   └── Events/
│   ├── ServiceName.Application/
│   │   ├── Commands/
│   │   ├── Queries/
│   │   ├── Handlers/
│   │   └── Interfaces/
│   ├── ServiceName.Infrastructure/
│   │   ├── Persistence/
│   │   ├── Messaging/
│   │   └── Adapters/
│   └── ServiceName.Api/
│       ├── Controllers/
│       └── Configuration/
├── tests/
│   └── ServiceName.Tests/
├── .nfw/
│   └── config.yaml
├── workspace-name.sln
└── README.md
```

---

## Configuration ⚙️

### Workspace Config (`.nfw/config.yaml`) ⚙️

```yaml
workspace:
  name: MyWorkspace
  version: 1.0.0

defaults:
  language: dotnet
  template: minimal

validation:
  strict: true
  ignore:
    - "// Auto-generated"
```

### 👤 User Config (`~/.nfw/config.yaml`)

```yaml
preferences:
  author: Your Name
  license: MIT

templates:
  source: local
  path: ~/.nfw/templates
```

---

## Architecture Rules 🏛️

The CLI enforces these layer rules:

| Layer              | Can Reference       |
| ------------------ | ------------------- |
| **Domain**         | Nothing (pure)      |
| **Application**    | Domain only         |
| **Infrastructure** | Domain, Application |
| **Api**            | All layers          |

Run `nfw check` to validate compliance.

---

## CI/CD Integration 🔄

### GitHub Actions

```yaml
name: Validate Architecture

on: [push, pull_request]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install nfw
        run: curl -fsSL https://get.nframework.com | sh
      - name: Check architecture
        run: nfw check
```

### 🔷 Azure Pipelines

```yaml
trigger:
  - main

pool:
  vmImage: "ubuntu-latest"

steps:
  - script: |
      curl -fsSL https://get.nframework.com | sh
    displayName: "Install nfw"
  - script: |
      nfw check
    displayName: "Validate Architecture"
```

---

## Examples 💡

### 🛒 E-Commerce Service

```bash
# Create workspace
nfw new Shop

# Add services
nfw add service Catalog --lang dotnet
nfw add service Orders --lang dotnet
nfw add service Payments --lang dotnet

# Add entities
nfw add entity Product --props Name:string,Price:decimal,SKU:string
nfw add entity Order --props CustomerId:guid,Status:OrderStatus,Total:decimal
nfw add entity Payment --props OrderId:guid,Amount:decimal,Method:PaymentMethod

# Add custom commands
nfw add command ProcessPayment Payments --secured --transactional
nfw add command CancelOrder Orders --secured --logged

# Validate
nfw check

# Run
dotnet run --project src/Orders/Api
```

### ⚡ Minimal API Service

```bash
nfw new MyApi --template minimal
nfw add service Core --lang dotnet
nfw add entity User --props Email:string,Name:string
nfw check
dotnet build
```

---

## Troubleshooting 🔧

### "Template not found"

```bash
# Update template cache
nfw templates --refresh
```

### "Architecture validation failed"

```bash
# Run with verbose output
nfw check --verbose

# Check specific rules
nfw check --rule layer-dependencies
```

### "Generation failed"

```bash
# Dry-run to see what would be generated
nfw add entity Test --props Name:string --dry-run

# Check for conflicting files
nfw add entity Test --props Name:string --force
```

---

## Versioning 🏷️

`nfw` follows [Semantic Versioning](https://semver.org/).

- **Major** — Breaking changes to commands or workspace structure
- **Minor** — New features, templates, or language support
- **Patch** — Bug fixes, performance improvements

Check your version:

```bash
nfw --version
```

---

## Contributing 🤝

Contributions are welcome! Please see:

- [Contributing Guide](CONTRIBUTING.md)
- [Development Setup](DEVELOPMENT.md)
- [Code of Conduct](CODE_OF_CONDUCT.md)

---

## Documentation 📚

- [Full PRD](docs/PRD.md) — Product requirements
- [Architecture Guide](docs/ARCHITECTURE.md) — CLI design
- [Template Authoring](docs/TEMPLATES.md) — Creating templates
- [NFramework Docs](https://nframework.com) — Framework documentation

---

## License ⚖️

MIT © NFramework Contributors

---

## Support 💬

- **Issues**: [GitHub Issues](https://github.com/n-framework/nfw/issues)
- **Discussions**: [GitHub Discussions](https://github.com/n-framework/nfw/discussions)
- **Discord**: [discord.gg/n-framework](https://discord.gg/n-framework)

---

**Built with love by the NFramework community**
