# Implementation Plan: Generate CRUD Command

**Branch**: `main` | **Date**: 2026-05-16 | **Spec**: [src/nfw/specs/014-gen-crud-command/spec.md](./spec.md)

**Input**: Feature specification from `src/nfw/specs/014-gen-crud-command/spec.md`

## Summary

Implement the `nfw gen crud <ENTITY_NAME>` CLI command in Rust to orchestrate the rapid generation of DTOs, CQRS commands/queries, handlers, repository contracts, and API endpoints for a given entity. The command must operate in <2 seconds, support interactive and non-interactive workflows, and output fully compilable `.NET` code into standard NFramework clean architecture layers.

## Technical Context

**Language/Version**: Rust 1.85+ (CLI), C# 14/15 (.NET 11 target for generated code)

**Primary Dependencies**: `clap` (CLI argument parsing), `dialoguer` (interactive prompts), NFramework `nfw-templates`

**Storage**: Local Filesystem (Workspace directories)

**Testing**: `cargo test` (unit and integration tests), `dotnet build` (compilation validation of generated code)

**Target Platform**: CLI tool (Linux, macOS, Windows)

**Project Type**: CLI command extension

**Performance Goals**: <2 seconds execution time end-to-end

**Constraints**: Generated C# code must be strictly trimmable/Native AOT compatible per NFramework rules. Must fail fast in non-interactive mode.

**Scale/Scope**: Operates on a single entity at a time, generating ~15-20 files per run.

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

- **I. Clean Architecture First**: Passes. Generated artifacts are placed into correct layers (Domain, Application, Presentation) without cross-layer dependency violations.
- **II. CLI Interface Standard**: Passes. Exposes `nfw gen crud` with consistent POSIX flags and supports both interactive and non-interactive modes.
- **III. Test-First**: Passes. Integration tests (T006, T012) are defined before core orchestration logic implementation.
- **IV. Integration Testing for Generated Code**: Passes. T012 explicitly verifies generated code compiles via `dotnet build`.
- **V. Performance and Observability**: Passes. T018 audits execution time against the <2s target; `--verbose` flag supported via CLI framework.
- **Native AOT Compatibility**: Passes. Generated C# code follows framework conventions for trimmable/AOT-compatible output.
- **Zero-Dependency Core**: Passes. Command orchestrates templates without introducing new library dependencies into the CLI itself.

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/014-gen-crud-command/
├── spec.md              # Feature specification
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (cli.md)
```

### Source Code (repository root)

```text
src/nfw/
├── src/
│   ├── commands/
│   │   └── gen/
│   │       ├── mod.rs
│   │       └── crud.rs        # Main command entry point and logic
│   └── lib.rs                 # Registration of new command
├── tests/
│   └── integration/
│       └── gen_crud_tests.rs  # Integration tests running `dotnet build`
```

**Structure Decision**: The implementation will integrate natively into the existing Rust CLI workspace (`src/nfw`). We will add the `crud.rs` command parser and orchestrator inside `src/commands/gen/`.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

_No violations detected. Standard CLI sub-command expansion._
