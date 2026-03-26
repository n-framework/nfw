# nfw CLI

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Buy A Coffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-ffdd00?logo=buy-me-a-coffee&logoColor=black&style=flat)](https://ahmetcetinkaya.me/donate)

The `nfw` CLI is the command-line entry point for the n-framework toolchain. It creates workspaces from templates and manages the development environment.

**Core Techs:**
[![.NET 11](https://img.shields.io/badge/.NET%2011-512BD4?style=flat&logo=dotnet&logoColor=white)](https://dotnet.microsoft.com)
[![C#](https://img.shields.io/badge/C%23-68217A?style=flat&logo=csharp&logoColor=white)](https://learn.microsoft.com/dotnet/csharp/)
[![Spectre.Console](https://img.shields.io/badge/Spectre.Console-000000?style=flat)](https://spectreconsole.net)
[![Scriban](https://img.shields.io/badge/Scriban-0050A0?style=flat)](https://github.com/scriban/scriban)

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
dotnet run --project src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- templates

# Create a new workspace
dotnet run --project src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- new my-workspace

# Create non-interactively in CI
dotnet run --project src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -- new --no-input my-workspace --template blank
```

---

## Commands

```bash
nfw templates                    # List available templates
nfw new [name]                   # Create a new workspace from a template
nfw new [name] --template <id>   # Create with an explicit template
nfw new --no-input [name]        # Non-interactive (for CI)
```

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

## Publish

```bash
dotnet publish src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -c Release -r linux-x64 -p:PublishSingleFile=true
dotnet publish src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -c Release -r osx-x64 -p:PublishSingleFile=true
dotnet publish src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj -c Release -r win-x64 -p:PublishSingleFile=true
```

---

## Related Packages

- [NFramework](https://github.com/n-framework/n-framework) — The parent framework
- [NFramework.Core.CLI](../n-framework-core-cli) — CLI framework library
- [NFramework.Core.Template](../n-framework-core-template) — Template engine library

---

## 📄 License

This project is licensed under the **Apache License 2.0** - see the [LICENSE](LICENSE) file for details.
