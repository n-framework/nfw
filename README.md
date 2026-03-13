# nfw CLI

![GitHub stars](https://img.shields.io/github/stars/n-framework/nfw?style=social)
![GitHub forks](https://img.shields.io/github/forks/n-framework/nfw?style=social)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

The official command-line interface for [NFramework](https://github.com/n-framework/n-framework) — a compile-time-first application framework for building clean architecture microservices.

---

`nfw` streamlines the creation and management of NFramework workspaces and services. Generate production-ready scaffolding, enforce architecture boundaries, and validate your codebase.

**Features:**

- Instant scaffolding — Create workspaces and services in seconds
- Code generation — Generate entities, commands, queries with full CRUD flows
- Architecture validation — Detect and report boundary violations
- Multi-language support — .NET (full), Go & Rust (structure)
- CI-ready — All commands work non-interactively for automation

---

## 🚀 Quick Start

### Installation

**Installation scripts (coming soon)**

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

## 📚 Documentation

- [Full PRD](docs/PRD.md) — Product requirements
- [NFramework Docs](https://docs.nframework.com) — Framework documentation

---

## ⚖️ License

This project is licensed under the [MIT License](LICENSE).
