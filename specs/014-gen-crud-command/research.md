# Research: Generate CRUD Command

## Domain Context

The `nfw gen crud` command belongs to the `nfw` CLI tool built in Rust, part of the NFramework ecosystem. It orchestrates the generation of a complete CRUD (Create, Read, Update, Delete) scaffolding for a `.NET` service entity. The CLI relies on source generators and abstract repositories for its scaffolding output.

## Extracted Unknowns and Resolutions

### Framework/CLI Ecosystem

**Decision**: Use `clap` for command parsing and `nfw-templates` for generator logic.
**Rationale**: `nfw` is already built using Rust 1.85+ with `clap`. The template/generator execution logic must seamlessly integrate with the existing `src/nfw-generators` schema used by `nfw gen endpoint` and `nfw add entity`.
**Alternatives considered**: Writing a completely new templating engine was rejected since `nfw-templates` already exists and maintains workspace generation conventions.

### File/Layer Generation Rules

**Decision**: Command will orchestrate existing generators or templates to produce DTOs, Commands, Queries, Handlers, Repositories, and Endpoints under `Application/Features/[Entity]` and Presentation layers.
**Rationale**: The spec mandates placing files into NFramework's standard Clean Architecture layers. The command must automatically create the target feature directory if missing.
**Alternatives considered**: Hardcoding paths instead of using the generator configuration. Rejected as it violates the dynamic template schema defined in `.specify/template.schema.json`.

### Interactive Prompts vs. Flags

**Decision**: Use a prompt library (like `dialoguer` in Rust) to interactively ask for missing options when a TTY is present. Use `--no-api`, `--secured`, `--cached` flags for non-interactive automation.
**Rationale**: Satisfies the spec's P2 stories for both developer experience (interactive) and CI/CD pipelines (non-interactive automation).
**Alternatives considered**: Only supporting flags. Rejected because it directly violates User Story 2.

### Handling Missing Entities

**Decision**: The command will interactively prompt to run the equivalent of `nfw add entity` if the entity is missing. In non-interactive mode, it will fail fast.
**Rationale**: Resolved during the clarification phase. It balances user experience in terminals with safe, predictable CI/CD failure modes.
**Alternatives considered**: Automatically generating the entity without asking, which was rejected during clarification.

### Performance Target

**Decision**: Generation must complete in <2 seconds.
**Rationale**: Rust's execution speed coupled with pre-compiled templates or fast string interpolation (e.g., `minijinja` or `tera` if used by `nfw-templates`) easily supports sub-second generation.
**Alternatives considered**: N/A, strict requirement.

### Testing Strategy

**Decision**: Integration tests using `cargo test` that invoke the command on a temporary workspace and verify `dotnet build` passes.
**Rationale**: Matches the standard `src/nfw/tests/integration/` approach used by other `nfw` commands, ensuring generated code is syntactically valid in C#.
**Alternatives considered**: Only unit testing the Rust logic. Rejected because it wouldn't guarantee the generated `.NET` code compiles, violating SC-002.
