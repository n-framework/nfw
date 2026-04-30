---

description: "Task list for nfw add persistence command implementation"
---

# Tasks: nfw add persistence Command

**Input**: Design documents from `/specs/009-add-persistence-command/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-command.md

**Tests**: Integration tests are included per feature specification (FR-008, SC-005).

**Organization**: Tasks are grouped by implementation phase to enable systematic development of the CLI command.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1 = Add Persistence to Service)
- Include exact file paths in descriptions

## Path Conventions

- **nfw CLI workspace**: `src/nfw/src/n-framework-nfw/`
- **Application layer**: `core/n-framework-nfw-core-application/src/features/template_management/commands/add_persistence/`
- **Presentation layer**: `presentation/n-framework-nfw-cli/src/commands/add/persistence/`
- **Integration tests**: `tests/integration/n-framework-nfw/features/module/`

---

## Phase 1: Setup (Module Structure)

**Purpose**: Create the basic module structure for the add persistence command

- [X] T001 Create add_persistence command module in src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/add_persistence/mod.rs
- [X] T002 Create add_persistence command module in src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/persistence/mod.rs

---

## Phase 2: Foundational (Command Registration)

**Purpose**: Register the CLI command and establish basic structure

**⚠️ CRITICAL**: Command must be registered before implementation can be tested

- [X] T003 Create CliCommandSpec registration in src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/persistence/registration.rs with command name "persistence", --service option, and --no-input flag
- [X] T004 Register add persistence command in src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/mod.rs to expose the persistence subcommand

**Checkpoint**: Command is registered and appears in CLI help

---

## Phase 3: User Story 1 - Add Persistence to Service (Priority: P1) 🎯 MVP

**Goal**: Enable developers to add the Persistence module to existing services through the CLI, with automatic template rendering and YAML updates.

**Independent Test**: Run `nfw add persistence --service <Service>` and verify: (1) persistence module added to nfw.yaml, (2) templates rendered in service directory, (3) YAML comments preserved.

### Integration Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [X] T005 [P] [US1] Create test support module in tests/integration/n-framework-nfw/features/module/persistence_add_test.rs with sandbox workspace setup utilities
- [X] T006 [P] [US1] Write test_add_persistence_updates_nfw_yaml_and_renders_template in tests/integration/n-framework-nfw/features/module/persistence_add_test.rs verifying successful addition
- [X] T007 [P] [US1] Write test_add_persistence_rolls_back_yaml_if_template_execution_fails in tests/integration/n-framework-nfw/features/module/persistence_add_test.rs verifying atomic rollback
- [X] T008 [P] [US1] Write test_add_persistence_fails_if_service_not_found in tests/integration/n-framework-nfw/features/module/persistence_add_test.rs verifying error handling
- [X] T009 [P] [US1] Write test_add_persistence_preserves_comments_in_nfw_yaml in tests/integration/n-framework-nfw/features/module/persistence_add_test.rs verifying comment preservation
- [X] T010 [P] [US1] Write test_add_persistence_detects_existing_persistence_module in tests/integration/n-framework-nfw/features/module/persistence_add_test.rs verifying duplicate detection

### Application Layer Implementation

- [X] T011 [US1] Create AddPersistenceCommand in src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/add_persistence/add_persistence_command.rs with service_info and workspace_context fields
- [X] T012 [US1] Create AddPersistenceCommandHandler in src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/add_persistence/add_persistence_command_handler.rs with ArtifactGenerationService dependency and handle() method
- [X] T013 [US1] Implement get_workspace_context() method in AddPersistenceCommandHandler delegating to ArtifactGenerationService
- [X] T014 [US1] Implement extract_services() method in AddPersistenceCommandHandler delegating to ArtifactGenerationService
- [X] T015 [US1] Implement handle() method in AddPersistenceCommandHandler to load template context, execute templates generating Infrastructure/Persistence layer with DbContext and repository base classes including dependency injection setup (without duplicate detection - added in T016)
- [X] T016 [US1] Add duplicate module detection check in AddPersistenceCommandHandler.handle() BEFORE template execution to check for existing "persistence" module and return early if present

### Presentation Layer Implementation

- [X] T017 [P] [US1] Create AddPersistenceRequest struct in src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/persistence/handler.rs with no_input, is_interactive_terminal, and service_name fields
- [X] T018 [US1] Create AddPersistenceCliCommand in src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/persistence/handler.rs with generic parameters W, R, E, P
- [X] T019 [US1] Implement execute() method in AddPersistenceCliCommand with intro, service selection logic, spinner progress, and success/error reporting
- [X] T020 [US1] Implement handle() static method in AddPersistenceCliCommand to parse Command options and invoke execute()
- [X] T021 [US1] Add service selection logic in execute() for interactive mode (multiple services) using InteractivePrompt::select()
- [X] T022 [US1] Add auto-selection logic in execute() for single service with --no-input flag
- [X] T023 [US1] Add service name validation in execute() to check service exists before proceeding
- [X] T024 [US1] Add duplicate persistence module detection in execute() and report info message if already present
- [X] T025 [US1] Integrate AddPersistenceCommandHandler in AddPersistenceCliCommand with CliServiceCollectionFactory
- [X] T026 [US1] Export AddPersistenceCliCommand in src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/persistence/mod.rs

### Error Handling and Exit Codes

- [X] T027 [US1] Map AddArtifactError variants to ExitCodes via ExitCodes::from_add_artifact_error() in execute() method covering: InvalidIdentifier→4, WorkspaceError→2, ConfigError→2, TemplateNotFound→3, ExecutionFailed→3, NfwYamlReadError/ParseError/WriteError→2; ensure permission errors map to exit code 5
- [X] T028 [US1] Add error message formatting for workspace errors (no services, service not found) with actionable guidance
- [X] T029 [US1] Add error message formatting for template errors (template not found, execution failed) with specific failure details
- [X] T030 [US1] Ensure rollback behavior on template execution failure by catching errors before nfw.yaml modification

**Checkpoint**: At this point, User Story 1 should be fully functional - developers can run `nfw add persistence` to add the persistence module to services with template rendering and atomic rollback.

---

## Phase 4: Polish & Cross-Cutting Concerns

**Purpose**: Final validation, documentation, and quality assurance

- [X] T031 [P] Run `cargo clippy --workspace -- -D warnings` from src/nfw/ and fix all warnings
- [X] T032 [P] Run `cargo fmt --all` from src/nfw/ to ensure consistent formatting
- [X] T033 [P] Run `cargo test --workspace persistence_add` from src/nfw/ and verify all tests pass
- [X] T034 [P] Run `cargo build --workspace` from src/nfw/ and verify compilation succeeds
- [X] T035 Validate quickstart.md commands are executable and produce expected results
- [X] T036 Verify integration tests achieve 90% code path coverage per SC-005
- [X] T037 Manual test: Create test workspace and run `nfw add persistence --service TestService --no-input` to verify end-to-end flow
- [X] T038 Manual test: Verify YAML comment preservation by checking nfw.yaml before and after command execution
- [X] T039 Performance test: Verify command completes in <5 seconds for typical workspaces (per SC-003)
- [X] T040 Performance test: Verify rollback completes in <1 second when template execution fails (per SC-004)
- [X] T041 Concurrency test: Verify concurrent command execution handles nfw.yaml conflicts safely (no corruption) per SC-007

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1 (Setup)**: No dependencies - can start immediately
- **Phase 2 (Foundational)**: Depends on Phase 1 completion - command must be registered before testing
- **Phase 3 (User Story 1)**: Depends on Phase 2 completion - tests require command registration
- **Phase 4 (Polish)**: Depends on Phase 3 completion - all features must be implemented

### User Story Dependencies

- **User Story 1 (P1)**: The complete add persistence feature - all acceptance scenarios are part of this story

### Within User Story 1

- Integration tests (T005-T010) MUST be written and FAIL before implementation
- Application layer (T011-T016) before presentation layer (T017-T026)
- Core implementation (T011-T026) before error handling refinements (T027-T030)
- Polish phase (T031-T041) after all implementation complete

### Parallel Opportunities

- All integration tests (T005-T010) can be written in parallel
- Application layer tasks (T011-T016) have internal dependencies based on the mediator pattern
- Presentation layer tasks (T017-T026) have some internal dependencies based on the mediator pattern
- All polish phase tasks (T031-T041) can run in parallel after implementation

### Key Dependencies from Mediator Pattern

Based on the research.md analysis, the implementation follows this dependency chain:

```text
AddPersistenceCommand (T011)
    ↓
AddPersistenceCommandHandler (T012)
    ↓
get_workspace_context() (T013)
    ↓
extract_services() (T014)
    ↓
handle() with template loading (T015)
    ↓
Duplicate detection (T016)
    ↓
Presentation layer (T017-T026)
```

---

## Parallel Example: Integration Tests

```bash
# Write all integration tests together (they use the same test file structure):
Task: "Create test support module in tests/integration/n-framework-nfw/features/module/persistence_add_test.rs"
Task: "Write test_add_persistence_updates_nfw_yaml_and_renders_template"
Task: "Write test_add_persistence_rolls_back_yaml_if_template_execution_fails"
Task: "Write test_add_persistence_fails_if_service_not_found"
Task: "Write test_add_persistence_preserves_comments_in_nfw_yaml"
Task: "Write test_add_persistence_detects_existing_persistence_module"
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup (module creation)
2. Complete Phase 2: Foundational (command registration)
3. Complete Phase 3: User Story 1 (all implementation)
4. **STOP and VALIDATE**: Run all integration tests and manual tests
5. Test with real workspace to verify end-to-end flow

### Incremental Delivery

Since this is a single command (not multiple independent features), delivery is all-or-nothing:

1. Implement core command following mediator pattern
2. Add integration tests covering all acceptance scenarios
3. Validate with real workspaces
4. Polish and optimize based on testing results

### Parallel Team Strategy

With multiple developers, work can be parallelized at the file level:

1. Developer A: Integration tests (T005-T010)
2. Developer B: Application layer (T011-T016)
3. Developer C: Presentation layer (T017-T026)

Integration happens through the established mediator pattern interfaces.

---

## Notes

- [P] tasks = different files, no dependencies
- [US1] label maps task to User Story 1 (Add Persistence to Service)
- The command follows the exact same pattern as `nfw add mediator` (spec 008) - configuration-based module addition
- Reuses existing types: ArtifactGenerationService, AddArtifactError, ServiceInfo, WorkspaceContext
- Integration tests use sandbox workspaces per the mediator test pattern
- YAML comment preservation is handled by ArtifactGenerationService (not new code needed)
- Atomic rollback is ensured by executing templates BEFORE modifying nfw.yaml
- Generated files create Infrastructure/Persistence layer with DbContext, repository base classes, and dependency injection setup (same pattern as mediator)
- nfw-templates repository must include persistence template at `templates/dotnet-service/persistence/` with template.yaml and .tera files for DbContext, repositories, and configuration
- All file paths are absolute from repository root: `/home/ac/Code/n-framework/src/nfw/`
- Total tasks: 41
- Test tasks: 6 (integration tests per feature specification requirement)
- Implementation tasks: 24 (setup, foundational, and core implementation)
- Polish tasks: 11 (validation, formatting, and quality assurance)

**MVP Scope**: Phase 3 (User Story 1) is the complete feature - all phases are required for a functional command.
