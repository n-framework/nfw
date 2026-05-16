# Task Breakdown: Generate CRUD Command

**Spec**: [src/nfw/specs/014-gen-crud-command/spec.md](./spec.md)
**Plan**: [src/nfw/specs/014-gen-crud-command/plan.md](./plan.md)

## Implementation Strategy

This feature will be implemented in three main increments:

1. **Foundational Parsing & Validation**: Setting up the `clap` CLI arguments, flags, and handling validation (like missing entities) so the input layer is robust.
2. **Interactive Prompts**: Adding the interactive layer for missing entities and flags when run in a TTY.
3. **Template Orchestration & Generation**: Wiring the validated inputs to the `nfw-templates` engine to produce all required C# artifacts.

MVP Scope is User Story 1 & 3 (CLI flags + orchestration). Interactive features (User Story 2) are layered on top.

## Phase 1: Setup

- [ ] T001 Define `GenCrudArgs` struct with `entity_name`, `--no-api`, `--secured`, `--cached`, `--force`, and `--no-input` flags using `clap` in `src/nfw/src/commands/gen/crud.rs`
- [ ] T002 Register the new `crud` subcommand in `src/nfw/src/commands/gen/mod.rs` to wire it into the `nfw gen` command tree

## Phase 2: Foundational

- [ ] T003 Implement initial argument validation logic to ensure `entity_name` is a valid C# identifier in `src/nfw/src/commands/gen/crud.rs`
- [ ] T004 Implement workspace-checking logic to verify the target directory exists and determine if the target Entity already exists in `src/nfw/src/commands/gen/crud.rs`
- [ ] T005 Implement logic to detect if target CRUD files already exist to support the `--force` flag workflow in `src/nfw/src/commands/gen/crud.rs`

## Phase 3: P1 - Full CRUD Command Execution (MVP)

_Goal: Scaffold an entire Create, Read, Update, Delete feature flow using flags._

- [ ] T006 [P] [US1] Create integration test scaffolding in `src/nfw/tests/integration/gen_crud_tests.rs` to execute `nfw gen crud` against a temp workspace
- [ ] T007 [US1] Map `GenCrudArgs` properties to the `nfw-templates` engine payload in `src/nfw/src/commands/gen/crud.rs`
- [ ] T008 [US1] Implement orchestration for DTOs and Commands generation in `src/nfw/src/commands/gen/crud.rs`
- [ ] T009 [US1] Implement orchestration for Queries and Handlers generation in `src/nfw/src/commands/gen/crud.rs`
- [ ] T010 [US1] Implement orchestration for Repository contract generation in `src/nfw/src/commands/gen/crud.rs`
- [ ] T011 [US1] Ensure the orchestration logic skips API Endpoint generation when `--no-api` is true in `src/nfw/src/commands/gen/crud.rs`
- [ ] T012 [US1] Ensure the orchestration logic injects `--secured` and `--cached` markers into the template payload in `src/nfw/src/commands/gen/crud.rs`
- [ ] T013 [US1] Finalize output logging to match the contract: `✓ Created Application/Features/...` followed by completion time tracking in `src/nfw/src/commands/gen/crud.rs`
- [ ] T014 [US1] Add `dotnet build` verification to the integration test in `src/nfw/tests/integration/gen_crud_tests.rs` to prove generated code compiles

## Phase 4: P2 - Interactive Prompts for Missing Options

_Goal: Provide an interactive wizard when the command is run without specific flags in a terminal._

- [ ] T015 [US2] Implement `dialoguer` prompt for "Generate API Endpoints? (y/n)" if `--no-api` flag is not explicitly passed and TTY is active in `src/nfw/src/commands/gen/crud.rs`
- [ ] T016 [US2] Implement `dialoguer` prompt for "Include caching markers? (y/N)" and "Include security markers? (y/N)" in `src/nfw/src/commands/gen/crud.rs`
- [ ] T017 [US2] Implement the missing entity prompt: "Entity [ENTITY_NAME] not found. Create it now? (Y/n)" and wire it to invoke `add entity` logic internally in `src/nfw/src/commands/gen/crud.rs`
- [ ] T018 [US2] Implement the overwrite prompt: "Files for [ENTITY_NAME] already exist. Overwrite? (y/N)" when files exist and `--force` is missing in `src/nfw/src/commands/gen/crud.rs`
- [ ] T019 [US2] Ensure all interactive prompts are bypassed (and fail fast if missing required states) when `--no-input` is passed in `src/nfw/src/commands/gen/crud.rs`

## Final Phase: Polish & Cross-Cutting

- [ ] T020 Audit command execution time in `gen_crud_tests.rs` to guarantee end-to-end execution completes in < 2 seconds
- [ ] T021 Update CLI help documentation (`--help`) with examples of generating CRUD with flags vs interactive mode

## Dependencies

- **Phase 1** must be completed before anything else.
- **Phase 2** must be completed before implementing the main orchestration logic.
- **Phase 3 (US1/3)** provides the core engine using flags.
- **Phase 4 (US2)** layers interactive prompts on top of the core engine.

## Parallel Execution

- Integration test scaffolding (T006) can be built independently of the Rust command structure.
- The `clap` struct definition (T001) can be done while someone else researches the exact `nfw-templates` payload requirements for T007.
