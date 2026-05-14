# Implementation Plan: Gen Endpoint Command

**Branch**: `013-gen-endpoint-command` | **Date**: 2026-05-07 | **Spec**: `src/nfw/specs/013-gen-endpoint-command/spec.md`
**Input**: Feature specification from `specs/013-gen-endpoint-command/spec.md`

## Summary

Implement `nfw gen endpoint` command to generate an HTTP Minimal API endpoint for an existing MediatR command or query. The generated file should use attribute routing based on the HTTP operation type (GET/POST/PUT/DELETE) and include OpenAPI annotations, completing in under 3 seconds.

## Technical Context

**Language/Version**: Rust 1.85+ (2024 edition)  
**Primary Dependencies**: clap (CLI), serde/serde_yaml (templates), NFramework templates store  
**Storage**: File system (templates and generated C# code)  
**Testing**: cargo test (unit, integration)  
**Target Platform**: CLI binary (Linux/macOS/Windows)  
**Project Type**: CLI Tool  
**Performance Goals**: < 3 seconds execution time per endpoint generation  
**Constraints**: Must validate C# Application layer structure before generation  
**Scale/Scope**: Generates single C# file in Api layer per invocation  

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- **I. Library-First (N/A)**: Command implementation is an extension of existing CLI application codebase
- **II. CLI Interface (PASS)**: Built entirely as a CLI command via `clap`, reading args format mapping
- **III. Test-First (PASS)**: Integration tests planned covering failure validation and successful boilerplate generation
- **IV. Integration Testing (PASS)**: Tests will invoke CLI to analyze scaffolded file structures
- **V. Observability (PASS)**: Uses existing nfw CLI error handling structured tracing

## Project Structure

### Documentation (this feature)

```text
specs/013-gen-endpoint-command/
├── spec.md              # Feature specification
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
└── quickstart.md        # Phase 1 output
```

### Source Code (repository root)

```text
src/nfw/src/commands/gen/
├── mod.rs               # (Update to register endpoint command)
└── endpoint.rs          # (New command implementation)

src/nfw/tests/integration/
└── gen_endpoint_tests.rs # (New test file)

src/nfw-templates/src/dotnet-service/
├── template.yaml        # (Update to register endpoint generator)
└── endpoint/
    └── Endpoint.cs.tera # (New template)
```

**Structure Decision**: Extending existing CLI command logic. Adding `commands/gen/endpoint.rs` and the generator definition in `nfw-templates/src/dotnet-service/`.
