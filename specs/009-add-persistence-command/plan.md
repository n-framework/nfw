# Implementation Plan: nfw add persistence Command

**Branch**: `009-add-persistence-command` | **Date**: 2026-04-29 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/009-add-persistence-command/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Implement the `nfw add persistence` CLI command that enables developers to add the Persistence module to existing NFramework services. The command updates `nfw.yaml` to register the persistence module and executes template rendering to generate DbContext, repository base classes, and database configuration artifacts. The implementation follows the same architectural pattern as the existing `nfw add mediator` command (spec 008), using the ArtifactGenerationService for workspace operations, template execution, and service module registration with atomic rollback on failure.

## Technical Context

**Language/Version**: Rust 1.85+ (2024 edition)
**Primary Dependencies**:

- `template-engine-rust` (from `src/core-template-rust/`)
- `n_framework_core_cli_abstractions` (CLI traits)
- `n_framework_core_cli_cliclack` (interactive prompts)
- `n_framework_nfw_core_application` (ArtifactGenerationService)
- YAML handling with comment preservation
- Entity Framework Core (generated artifacts target .NET)

**Storage**: File system operations (nfw.yaml, service directories, template files)
**Testing**: `cargo test` with integration tests using sandbox workspaces
**Target Platform**: Cross-platform (Linux, macOS, Windows) - same as nfw CLI
**Project Type**: CLI command (nfw workspace tool)
**Performance Goals**:

- <5 seconds total execution time for typical workspaces
- <1 second rollback time on template failure
- Support workspaces with up to 10 services

**Constraints**:

- Must preserve YAML comments 100% when updating nfw.yaml
- Must roll back nfw.yaml changes atomically if template execution fails
- Must detect existing persistence modules and skip redundant operations
- Must support both interactive and automated (--no-input) modes

**Scale/Scope**:

- Workspaces with 1-10 services
- Template artifacts targeting .NET services (DbContext, repository base classes)
- Single service per command execution (no batch operations)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Single-Step Build And Test

- ✅ **Build**: `cargo build --workspace` from `src/nfw/`
- ✅ **Test**: `cargo test --workspace` from `src/nfw/`
- ✅ **Commands documented**: nfw has established build/test workflow

### II. CLI I/O And Exit Codes

- ✅ **Normal output**: stdout via Logger trait
- ✅ **Diagnostics**: stderr via Logger trait
- ✅ **Exit codes**: ExitCodes enum with specific error mappings (AddArtifactError → ExitCodes)
- ✅ **SIGINT handling**: 130 for Ctrl+C (inherited from CLI abstractions)

### III. No Suppression

- ✅ **No compiler warning suppression**: Rust `cargo clippy -- -D warnings` enforced
- ✅ **No test skipping**: All integration tests must pass
- ✅ **Error surfacing**: AddArtifactError provides actionable messages

### IV. Deterministic Tests

- ✅ **Unit tests**: No network access, use in-memory fakes
- ✅ **Integration tests**: Sandbox workspaces with temporary directories (see mediator_add_test.rs pattern)

### V. Documentation Is Part Of Delivery

- ✅ **Quickstart**: Generated in Phase 1
- ✅ **No contradictions**: Spec is internally consistent

**GATE STATUS**: ✅ PASSED - No constitution violations

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/009-add-persistence-command/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output (CLI command interface)
└── checklists/
    └── requirements.md  # Quality checklist (already created)
```

### Source Code (nfw CLI workspace)

Following the same pattern as `008-add-mediator-command`:

```text
src/nfw/src/n-framework-nfw/
├── core/
│   └── nframework-nfw-application/
│       └── features/
│           └── template_management/
│               ├── commands/
│               │   └── add_persistence/
│               │       ├── add_persistence_command.rs
│               │       └── add_persistence_command_handler.rs
│               └── models/
│                   └── errors/
│                       └── add_artifact_error.rs (already exists)
└── presentation/
    └── n-framework-nfw-cli/
        └── src/
            └── commands/
                └── add/
                    └── persistence/
                        ├── mod.rs
                        ├── registration.rs
                        └── handler.rs

tests/integration/n-framework-nfw/
└── features/
    └── module/
        └── persistence_add_test.rs
```

**Structure Decision**: Single project (nfw CLI) with clean architecture layers. The command pattern follows the established `add mediator` implementation, reusing:

- `ArtifactGenerationService` for core workflow
- `AddArtifactError` for error handling
- CLI abstractions for consistent user interaction
- Integration test pattern with sandbox workspaces

## Complexity Tracking

> **No violations - table omitted**

This implementation introduces no constitution violations. It follows an established, proven pattern from the mediator command with minimal complexity:

- Reuses existing `ArtifactGenerationService`
- Reuses existing `AddArtifactError` type
- Follows same CLI command structure as `add mediator`
- No new architectural patterns or abstractions required
