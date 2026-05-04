# Implementation Plan: Generate Repository Command

**Branch**: `011-gen-repository-command` | **Date**: 2026-05-03 | **Spec**: [spec.md](../spec.md)

**Input**: Feature specification from `src/nfw/specs/011-gen-repository-command/spec.md`

**Note**: This plan follows the `/speckit.plan` workflow. The CLI (Rust) reads template configuration from `src/nfw-templates` and applies it.

## Summary

Implement `nfw gen repository <NAME> [--feature <FEATURE>]` CLI command in Rust that:

- Parses CLI arguments using `clap`
- Validates entity exists, persistence configured in `nfw.yaml`
- Reads repository template configuration from `src/nfw-templates/src/dotnet-service/repository/`
- Applies template steps (render, inject, run_command) to generate files
- Completes in <2 seconds for valid inputs

## Technical Context

**Language/Version**: Rust 1.85+ (2024 edition) for CLI  
**Primary Dependencies**: `clap` (CLI parsing), `serde`/`serde_yaml` (template config parsing)  
**Storage**: N/A (CLI tool, reads templates from `src/nfw-templates`)  
**Testing**: `cargo test --workspace` (Rust integration tests)  
**Target Platform**: Linux/macOS/Windows (CLI tool)  
**Project Type**: CLI tool (nfw CLI)  
**Performance Goals**: <2 seconds command execution for valid inputs  
**Constraints**: Must validate entity exists, persistence configured in `nfw.yaml`, template config readable  
**Scale/Scope**: Single entity repository generation per command invocation  

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Constitution Article | Status | Notes |
|---------------------|--------|-------|
| I. Single-Step Build And Test | ✅ PASS | `cd src/nfw && make build` and `make test` |
| II. CLI I/O And Exit Codes | ✅ PASS | stdout output, stderr errors, exit 0/non-zero/130 for SIGINT |
| III. No Suppression | ✅ PASS | No warning suppression, no swallowing exceptions |
| IV. Deterministic Tests | ✅ PASS | Integration tests use CLI test utilities, no real network |
| V. Documentation Is Part Of Delivery | ✅ PASS | quickstart.md created |
| Additional: Repository conventions | ✅ PASS | Template config defines paths, CLI applies them |

**GATE RESULT**: ✅ ALL PASSED

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/011-gen-repository-command/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (minimal, no clarifications needed)
├── data-model.md        # Phase 1 output (template config structure)
├── quickstart.md        # Phase 1 output (how to use the CLI command)
├── contracts/           # Phase 1 output (CLI contract, not applicable - skip)
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (src/nfw - Rust CLI implementation)

```text
src/nfw/
├── src/
│   ├── commands/
│   │   └── gen/
│   │       └── repository.rs          # NEW: Repository generation command (Rust)
│   └── template/
│       └── processor.rs               # EXISTING: Template config reader/applier (Rust)
├── tests/
│   ├── integration/
│   │   └── gen_repository_tests.rs  # NEW: Integration tests (Rust)
│   └── unit/
│       └── repository_command_tests.rs # NEW: Unit tests (Rust)
└── specs/
    └── 011-gen-repository-command/   # This feature spec
```

### Template Store (src/nfw-templates - configuration the CLI reads)

```text
src/nfw-templates/src/dotnet-service/
├── template.yaml          # MODIFY: Add `repository: ./repository/` to generators (YAML)
└── repository/           # NEW: Repository template config and files (YAML + .tera templates)
    ├── template.yaml     # Repository generation steps (YAML - CLI reads this)
    └── content/         # Template files (CLI renders these)
```

**Structure Decision**: Follow existing nfw CLI patterns. Template configuration drives all file generation; CLI Rust code does NOT hardcode paths or class names.

## Complexity Tracking

> **No violations** - Constitution gates all passed.

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| (None) | - | - |

## Phase 0: Research

### Prerequisites: None (no NEEDS CLARIFICATION in spec)

Research completed during spec phase. Key decisions already made:

### Decision: Template-Driven Generation

- **Rationale**: CLI reads repository template configuration from `src/nfw-templates/src/dotnet-service/repository/template.yaml` and applies steps (render, inject). This follows existing patterns (entity, persistence generators).
- **Alternatives considered**: Hardcoding file paths in CLI code → Rejected: Reduces flexibility, harder to maintain.

### Decision: Entity-Specific File Generation

- **Rationale**: CLI generates entity-specific repository files (e.g., `IUserRepository.cs`, `UserRepository.cs`) by applying templates. Template defines the structure.
- **Alternatives considered**: Generic repository → Rejected: User requested entity-specific files.

### Decision: DI Registration via Template

- **Rationale**: CLI injects DI registration into Infrastructure layer's service registration file by applying template configuration.
- **Alternatives considered**: Hardcoding DI patterns in CLI → Rejected: Violates template-driven approach.

**Output**: [research.md](research.md) (minimal, see above decisions)

## Phase 1: Design & Contracts

### Prerequisites: research.md complete

### Data Model (data-model.md)

Template configuration structure (YAML) that the CLI reads and applies:

```yaml
# src/nfw-templates/src/dotnet-service/repository/template.yaml
id: dotnet-service/repository
name: Repository Generator
steps:
  - action: render
    source: "content/interface/IEntityRepository.cs.tera"
    destination: "src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Repositories/I{{ Entity }}Repository.cs"
  # ... more steps
```

**Key Points**:

- CLI reads this YAML configuration
- CLI substitutes placeholders: `{{ Service }}`, `{{ Feature }}`, `{{ Entity }}`
- CLI applies actions: `render`, `inject`, `run_command`

### Contracts

*Skip*: CLI tool with no external API. Command contract is defined by `clap` in Rust code.

### Agent Context Update

Run agent context update script (after Phase 1 artifacts complete):

```bash
bash .specify/scripts/bash/update-agent-context.sh
```

**Output**: data-model.md, quickstart.md, agent-specific context file

## Phase 2: Implementation Tasks

*Note: tasks.md is created by `/speckit.tasks` command, NOT by `/speckit.plan`.*

Planned artifact: `tasks.md` (created in separate step)

---

**Status**: Phase 0 and Phase 1 complete. Ready for `/speckit.tasks` to generate implementation tasks.
