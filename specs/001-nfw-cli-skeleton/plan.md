# Implementation Plan: nfw CLI Skeleton

**Branch**: `001-nfw-cli-skeleton` | **Date**: 2026-03-14 | **Spec**: [`src/nfw/specs/001-nfw-cli-skeleton/spec.md`](./spec.md)  
**Spec Type**: Project-Based  
**Project**: nfw  
**Input**: Feature specification from [`src/nfw/specs/001-nfw-cli-skeleton/spec.md`](./spec.md)

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Deliver the initial `nfw` CLI skeleton as the foundational entry point for the nfw toolchain: argument parsing, DI-backed command routing (Spectre.Console.Cli), `--help`/`--version`/`templates`, YAML config loading from `nfw.yaml` (CWD) with env var overrides, predictable exit codes, and an opt-in `--verbose` mode for diagnostic stderr logging.

## Technical Context

**Language/Version**: C# (.NET 11)  
**Primary Dependencies**: Spectre.Console, Spectre.Console.Cli, Microsoft.Extensions.DependencyInjection; YamlDotNet (config)  
**Storage**: N/A (project-scoped config file + env vars only)  
**Testing**: xUnit (+ FluentAssertions), `dotnet test`  
**Target Platform**: Linux, macOS, Windows (developer workstations)  
**Project Type**: CLI tool  
**Performance Goals**: Help output in <100ms; startup/output in <200ms on typical dev hardware  
**Constraints**: Prefer a Spectre.Console.Cli + Microsoft DI integration pattern using `TypeRegistrar`/`TypeResolver`; stdout for normal output, stderr for diagnostics/errors; `templates` uses a submodule in Debug builds and a remote release tag in Release builds; distribution target is a single-file publish for `linux-x64`, `osx-x64`, and `win-x64`  
**Scale/Scope**: Single-user CLI invocation; small config surface (no required keys in skeleton phase)

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

Gates from `.specify/memory/constitution.md`:

- Must: Single-step build and test commands exist and are documented (`make build`, `make test`).
- Must: CLI I/O and exit code conventions are followed (stdout vs stderr; stable exit codes).
- Must: No warning/test suppression is introduced.
- Must: Tests remain deterministic (no real network access in unit tests).

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/001-nfw-cli-skeleton/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
   └── cli.md
```

### Source Code (repository root)

```text
src/nfw/
├── NFramework.Nfw.slnx
├── packages/
│  ├── n-framework-core-codegen/        # git submodule (separate repository)
│  │  └── src/NFramework.NFW.Core.CodeGen/
│  │     └── NFramework.NFW.Core.CodeGen.csproj
│  └── n-framework-nfw-templates/       # git submodule (debugging environments)
├── src/
│  └── NFramework.NFW/
│     ├── core/
│     │  ├── NFramework.NFW.Application/
│     │  │  ├── NFramework.NFW.Application.csproj
│     │  │  ├── ApplicationServiceRegistration.cs
│     │  │  └── Features/
│     │  │     ├── Cli/
│     │  │     └── Templates/
│     │  └── NFramework.NFW.Domain/
│     │     ├── NFramework.NFW.Domain.csproj
│     │     └── Features/
│     │        ├── Templates/
│     │        └── Version/
│     └── presentation/
│        └── NFramework.NFW.CLI/
│           ├── NFramework.NFW.CLI.csproj
│           ├── Program.cs
│           └── Features/
│              ├── Help/
│              ├── Version/
│              └── Templates/
└── tests/
   └── NFramework.NFW.CLI.Tests/
      ├── NFramework.NFW.CLI.Tests.csproj
      ├── Cli/
      └── Configuration/
```

**Structure Decision**: Use a layered structure under `src/NFramework.NFW/core/{NFramework.NFW.Application,NFramework.NFW.Domain}` plus `src/NFramework.NFW/presentation/NFramework.NFW.CLI` for the executable. Use `src/nfw/packages/n-framework-nfw-templates` as a submodule for debugging, and fetch `github.com/n-framework/nfw-templates` by matching release tag in production. Keep `n-framework-core-codegen` as the only code package submodule.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

No constitution violations required for this feature.

## Phase 0: Research Output

Artifacts:

- `/home/ac/Code/n-framework/n-framework/src/nfw/specs/001-nfw-cli-skeleton/research.md`

All previously open technical unknowns (CLI framework, config format, flag conventions, exit codes) are resolved in `research.md`.

## Phase 1: Design & Contracts Output

Artifacts:

- `/home/ac/Code/n-framework/n-framework/src/nfw/specs/001-nfw-cli-skeleton/data-model.md`
- `/home/ac/Code/n-framework/n-framework/src/nfw/specs/001-nfw-cli-skeleton/contracts/cli.md`
- `/home/ac/Code/n-framework/n-framework/src/nfw/specs/001-nfw-cli-skeleton/quickstart.md`

### Post-Design Constitution Check (Re-evaluation)

- Pass: Plan and contracts keep stdout vs stderr responsibilities clear.
- Pass: Exit codes are defined and testable (including SIGINT 130).
- Pass: Build/test commands are single-step and documented (`make build`, `make test`).
- Pass: Template listing validation does not require network in unit tests.

## Phase 2: Planning Notes (Stop Point)

This plan intentionally stops before generating `tasks.md` (handled by `/speckit.tasks`). Next steps should translate the contracts and data model into:

- CLI parsing + routing implementation
- Help/version/templates command implementations
- Config loader (`nfw.yaml` + env overrides)
- Unit and integration tests around exit codes and error messaging
- Dev workflow parity with the old project: `dotnet tool restore`, `dotnet csharpier .`, `dotnet roslynator analyze`
