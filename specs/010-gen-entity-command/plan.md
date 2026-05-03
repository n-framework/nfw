# Implementation Plan: nfw gen entity Command

**Branch**: `010-gen-entity-command` | **Date**: 2026-04-30 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/src/nfw/specs/010-gen-entity-command/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/plan-template.md` for the execution workflow.

## Summary

Generate a CLI command that creates entity schema files with language-agnostic general types and invokes the template engine for code generation. The command accepts property definitions via CLI arguments, creates/reads YAML schema files, and provides entity parameters to templates. Templates in `nfw-templates` determine target language, output directory, base class, and type mappings. Entity generation requires the persistence module to be added first to the target service.

## Technical Context

**Language/Version**: Rust 1.85+ (2024 edition) for nfw CLI; templates target multiple languages (C#, Go, Rust) via `nfw-templates`
**Primary Dependencies**: clap (CLI parsing), template-engine-rust, serde_yaml (YAML parsing), cli-abstraction crates
**Storage**: File system (YAML schema files in configured `entitySpecsPath`, generated code location determined by templates)
**Testing**: cargo test with integration tests for schema creation and template invocation
**Target Platform**: Linux/macOS/Windows (CLI tool)
**Project Type**: CLI command
**Performance Goals**: <3 seconds for schema creation and template invocation (FR-013, SC-002)
**Constraints**: Schema uses general types only; templates handle language-specific decisions
**Scale/Scope**: Entity schema generation for services in monorepo workspace

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Single-Step Build And Test

✅ **PASS**: nfw CLI workspace uses `cargo build --workspace` and `cargo test --workspace` from `src/nfw/`

### II. CLI I/O And Exit Codes

✅ **PASS**: Spec requires structured logs to stdout (FR-030), errors to stderr, exit codes for failure scenarios (SC-005)

### III. No Suppression

✅ **PASS**: Integration tests must validate generated code compiles without warnings (SC-003, SC-006)

### IV. Deterministic Tests

✅ **PASS**: Unit tests use fakes/mocks; integration tests use temporary test workspaces

### V. Documentation Is Part Of Delivery

✅ **PASS**: Quickstart will be generated in Phase 1 covering build/run/test for entity generation

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/010-gen-entity-command/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
└── contracts/           # Phase 1 output
    └── cli-schema.md    # CLI command interface definition
```

### Source Code (nfw CLI workspace)

```text
src/nfw/src/nframework-nfw/
├── core/
│   ├── nframework-nfw-domain/
│   │   └── features/entity_generation/
│   │       ├── entities/           # AddEntityCommand, PropertyDefinition, etc.
│   │       ├── value_objects/      # EntityGenerationParameters, PropertySyntax
│   │       └── errors/             # EntityGenerationError variants
│   └── nframework-nfw-application/
│       └── features/entity_generation/
│           ├── commands/           # AddEntityCommandHandler
│           ├── services/           # PropertySyntaxParser, EntitySchemaGenerator, ServiceModuleValidator
│           └── abstractions/       # EntityTypeResolver, EntityNameValidator
├── infrastructure/
│   └── nframework-nfw-infrastructure-filesystem/
│       └── features/entity_generation/
│           └── adapters/           # File system operations for schema/code generation
└── presentation/
    └── n-framework-nfw-cli/
        └── commands/
            └── gen_entity.rs       # CLI command adapter (clap)

src/nfw/tests/integration/
└── entity_generation/
    ├── success_tests.rs            # Successful entity generation scenarios
    ├── error_tests.rs              # Error scenarios and validation
    └── schema_tests.rs             # Schema file creation and validation
```

**Structure Decision**: Feature-based organization within clean architecture layers. Domain entities and value objects define the core concepts. Application services handle validation and orchestration. Infrastructure adapters manage file I/O. Presentation layer handles CLI interaction.

## Complexity Tracking

> **No violations** - Constitution check passed without issues

## Phase 0: Research ✅

**Output**: [research.md](./research.md)

**Resolved Unknowns**:

- Type mapping from C# CLI syntax to general types
- Schema file format and storage location
- Template integration approach
- Service module validation logic
- Property validation rules

**Key Decisions**:

1. CLI uses C#-like syntax; schema uses general types; templates handle language-specific mapping
2. YAML schema files stored in configurable `entitySpecsPath` (default: `specs/entities/`)
3. Persistence module validation required before entity generation
4. Primitive types only; no collections or complex types
5. Hybrid workflow: quick-start (CLI args) and schema-first (YAML editing)

## Phase 1: Design & Contracts ✅

**Output**:

- [data-model.md](./data-model.md) - Entity definitions and relationships
- [contracts/cli-schema.md](./contracts/cli-schema.md) - CLI command interface
- [quickstart.md](./quickstart.md) - Developer quickstart guide

**Artifacts Created**:

1. **Data Model**: Core entities (AddEntityCommand, PropertyDefinition, EntityGenerationParameters), error types, value objects
2. **CLI Contract**: Command signature, options, validation rules, exit codes, output format
3. **Quickstart**: Prerequisites, common workflows, examples, error handling

**Design Validation**:

- ✅ Single-step build: `cargo build --workspace`
- ✅ Single-step test: `cargo test --workspace`
- ✅ CLI I/O: stdout for logs, stderr for errors, stable exit codes
- ✅ Deterministic tests: Unit tests use fakes; integration tests use temp workspaces
- ✅ Documentation: Quickstart covers build/run/test workflow

## Next Steps

**Phase 2**: Run `/speckit.tasks` to generate task breakdown

The implementation will follow this structure:

1. Parse CLI arguments and validate
2. Check persistence module presence
3. Map C# types to general types
4. Create or read schema file
5. Invoke template engine
6. Generate entity code in Domain layer
7. Write integration tests

**Key Files to Implement**:

- `src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/`
- `src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/`
- `src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/commands/gen_entity.rs`
- `src/nfw/tests/integration/entity_generation/`
