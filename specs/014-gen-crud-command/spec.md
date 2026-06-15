# Feature Specification: Generate CRUD Command

**Created**: 2026-05-16

**Status**: Draft

**Input**: User description: "Implement the `nfw gen crud <ENTITY_NAME>` command with optinal parameters and interactive prompts that generates complete CRUD scaffolding by orchestrating, repository, commands, queries, handlers, DTOs, and API endpoints; create feature folder structure automatically; generate all required files in correct layers; validate generated code compiles as a unit; complete in <2 seconds; include integration tests for full CRUD flow. Don't forget, you have to work in @[src/nfw/] only for creating branch, spec files etc., if needed update speckit scripts in @.specify ."

## User Scenarios & Testing

### User Story 1 - Full CRUD Command Execution (Priority: P1)

As an application developer using the `nfw` CLI, I want to run `nfw gen crud <EntityName>` so that I can quickly scaffold an entire Create, Read, Update, Delete feature flow without writing the boilerplate manually.

**Why this priority**: Scaffolding full CRUD features represents a large chunk of typical development work. Automating it saves immense time and enforces the architecture consistently.

**Independent Test**: Can be tested by running the command against a known entity and validating that all expected layers and files are generated and compile.

**Acceptance Scenarios**:

1. **Given** a valid `.NET` microservice workspace, **When** I run `nfw gen crud Product`, **Then** the CLI generates DTOs, Commands, Queries, Handlers, Repository contracts, and Endpoints for the `Product` entity in the correct layers, and the code compiles without manual edits.
2. **Given** the command executes, **When** I time the execution, **Then** the CLI completes the generation process in under 2 seconds.

---

### User Story 2 - Interactive Prompts for Missing Options (Priority: P2)

As a developer using the `nfw` CLI, I want the tool to prompt me interactively if I don't supply all necessary options, so I don't have to memorize every command line flag.

**Why this priority**: Greatly improves developer experience and discoverability.

**Independent Test**: Can be tested by running the command without specifying optional parameters and verifying the CLI asks interactive questions.

**Acceptance Scenarios**:

1. **Given** I am in an interactive terminal, **When** I run `nfw gen crud Product` without flags, **Then** the CLI prompts me for options like "Generate API Endpoints? (y/n)" or "Include caching? (y/n)".

---

### User Story 3 - Optional Parameters (Priority: P2)

As an automated build script or power user, I want to supply all options via command-line parameters (e.g. `--param no-api=true`) so that I can bypass interactive prompts.

**Why this priority**: Required for CI/CD environments and power-user speed.

**Independent Test**: Can be tested by running the command with the `--no-input` or specific feature flags and verifying it doesn't prompt for input.

**Acceptance Scenarios**:

1. **Given** I am in a non-interactive shell or provide parameters via `--param`, **When** I run `nfw gen crud Product --param no-api=true`, **Then** the CLI generates the handlers and contracts but skips generating the API endpoint files.

---

## Edge Cases

- **Entity Does Not Exist**: What happens if the referenced entity class does not exist in the domain layer? The CLI interactively prompts to create the entity (calling `nfw add entity` under the hood). If in non-interactive mode, it fails fast with a clear error message.
- **Files Already Exist**: When a developer runs the command for an entity that already has some CRUD files generated, the CLI interactively prompts whether to overwrite or skip the existing files. In non-interactive mode, the command fails gracefully unless a `--force` flag is provided.
- **Invalid Entity Name**: Running the command with an invalid C# identifier (e.g., `nfw gen crud 123Product`). The CLI should validate and provide an actionable error message.
- **Performance Constraints**: Running the generation process on a slow machine must still prioritize completing within the 2-second target where possible.

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST provide an `nfw gen crud <ENTITY_NAME>` command.
- **FR-002**: The command MUST generate the following artifacts:
  - DTOs (e.g., `Create[Entity]Request`, `[Entity]Response`)
  - Commands (e.g., `Create[Entity]Command`, `Update[Entity]Command`, `Delete[Entity]Command`)
  - Queries (e.g., `Get[Entity]ByIdQuery`, `List[Entity]sQuery`)
  - CQRS Handlers for all generated Commands and Queries
  - Repository contract (e.g., `I[Entity]Repository`)
  - API Endpoints (Minimal API mappings)
- **FR-003**: The command MUST automatically create the feature folder structure (e.g., `Application/Features/[Entity]`).
- **FR-004**: The command MUST place generated files into the correct NFramework architectural layers (Domain, Application, Presentation/Api).
- **FR-005**: The command MUST support optional parameters (e.g., `--param no-api=true`, `--param secured=true`) to customize generation.
- **FR-006**: The command MUST provide interactive prompts for generation options if run in an interactive terminal and not supplied via flags.
- **FR-007**: The generated code MUST compile successfully as a unit without requiring manual developer edits.
- **FR-008**: The command MUST include integration tests that cover the execution of the full CRUD flow generation.
- **FR-009**: The command execution MUST complete in less than 2 seconds.

### Key Entities

- **Feature Directory Structure**: The physical folder representation defining module boundaries.
- **CRUD Operations**: The set of Create, Read, Update, Delete actions associated with an entity.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Execution time for `nfw gen crud` is consistently under 2 seconds on a standard developer machine.
- **SC-002**: 100% of generated CRUD scaffolding compiles successfully immediately after command execution.
- **SC-003**: Integration tests covering the CRUD generation command pass 100% of the time in CI.

## Assumptions

- The target workspace is a valid `.NET` NFramework workspace with the expected layer structure (`Domain`, `Application`, `Api`, etc.).
- The execution environment meets the minimum hardware requirements to allow sub-2-second command completion.
- The user has the necessary permissions to create directories and write files in the workspace.

## Clarifications

- Q: What happens if the entity class itself hasn't been created yet using `nfw add entity`? → A: The CLI interactively prompts to create the entity (calling `nfw add entity` under the hood). If in non-interactive mode, it fails fast with a clear error message.
- Q: Which specific optional parameters are required? → A: Following standard CLI generator patterns using `--param`: `no-api=true` (skip endpoints), `secured=true` (add auth markers), `cached=true` (add cache markers), `force=true` (overwrite existing).
- Q: How should we test that the generated code compiles? → A: The integration tests will invoke `dotnet build` or equivalent on the temporary generated workspace to verify successful compilation.

## Non-Goals

- Replacing the existing `nfw add entity` or `nfw add command` commands. This command orchestrates them or functions alongside them.
- Generating the concrete implementation of the Repository (e.g., Entity Framework Core `DbContext` updates). This command focuses on the contract and application layers.
- Generating a frontend UI for the CRUD operations.
