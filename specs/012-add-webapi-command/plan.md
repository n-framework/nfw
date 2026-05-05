# Implementation Plan: Add WebAPI Command

**Branch**: `012-add-webapi-cmd` | **Date**: 2026-05-05 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `src/nfw/specs/012-add-webapi-cmd/spec.md`

## Summary

Implement the `nfw add webapi` command within the Rust CLI to scaffold the Minimal API layer for an existing .NET service. This includes interactive service selection, template rendering for Minimal API configuration (CORS, health checks, OpenAPI, problem details), and updating the `nfw.yaml` configuration with full rollback capability and comment preservation on failure.

## Technical Context

**Language/Version**: Rust 1.85+ (CLI framework) and .NET 11 (Template output)
**Primary Dependencies**: `clap`, `serde`, `serde_yaml` (with comment preservation support), `nfw-templates`
**Storage**: File system (template reading, code generation, config modification)
**Testing**: `cargo test --workspace` (unit and integration tests)
**Target Platform**: CLI (Linux, macOS, Windows)
**Project Type**: CLI command
**Performance Goals**: Command completes entirely in under 5 seconds (excluding interactive prompt time)
**Constraints**: Must support full safe rollback on template rendering or file writing failure; must preserve YAML comments.
**Scale/Scope**: New CLI subcommand with interactive prompt, multi-file template rendering, and configuration file modification.

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- [x] **I. Single-Step Build And Test**: Can be verified via `cd src/nfw && make build` and `cd src/nfw && make test`.
- [x] **II. CLI I/O And Exit Codes**: Standard `nfw` exit codes will be used (0 for success, non-zero for failures, stderr for diagnostics).
- [x] **III. No Suppression**: All errors in parsing, generation, or rollback will be surfaced. Tests will not be skipped.
- [x] **IV. Deterministic Tests**: Integration tests will use isolated mock filesystems or temporary directories, no network access required.
- [x] **V. Documentation Is Part Of Delivery**: The feature quickstart and contract definitions are provided.

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/012-add-webapi-cmd/
├── spec.md              
├── plan.md              
├── research.md          
├── data-model.md        
├── quickstart.md        
├── contracts/           
│   └── cli-interface.md 
└── tasks.md             
```

### Source Code (repository root)

```text
src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/
└── add_webapi/
    ├── add_webapi_command.rs         # The command payload
    ├── add_webapi_command_handler.rs # App layer: fetches templates, renders them, updates nfw.yaml safely
    └── mod.rs

src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/
├── webapi/                 
│   ├── handler.rs          # Presentation layer: Prompts for service selection, handles spinners/logging, delegates to AddWebApiCommandHandler
│   ├── mod.rs              
│   └── registration.rs     # Command registration (CLI arguments: --service, --no-input)
├── registration.rs         # Update to include webapi_register()
└── mod.rs                  # Update to include `pub mod webapi;`

src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/startup/
└── cli_service_collection_factory.rs # Register AddWebApiCommandHandler in the DI container

src/nfw/tests/integration/n-framework-nfw/features/artifact/
└── webapi_add_test.rs      # Integration tests covering success and rollback

src/nfw-templates/src/dotnet-service/
└── webapi/                 # Templates for Minimal API, Swagger, HealthCheck, etc.
```

**Structure Decision**: Extending the existing architecture. The presentation layer (`n-framework-nfw-cli`) will handle interactive prompts (`cliclack`), spinner states, and error formatting, mirroring `persistence` and `mediator`. It will delegate to the application layer (`n-framework-nfw-core-application`) which is responsible for resolving the template root, reading the workspace configuration (`nfw.yaml`), executing the template engine to render the `webapi` templates, and applying configuration updates with safe rollback on failure.

## Complexity Tracking

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| None | N/A | N/A |
