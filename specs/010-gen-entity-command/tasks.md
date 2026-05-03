# Tasks: nfw gen entity Command

**Input**: Design documents from `/src/nfw/specs/010-gen-entity-command/`
**Prerequisites**: plan.md, spec.md, research.md, data-model.md, contracts/cli-schema.md
**Tests**: Integration tests are REQUIRED per FR-023

**Organization**: Tasks are grouped by user story to enable independent implementation and testing.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (US1, US2)
- Include exact file paths in descriptions

## Path Conventions

nfw CLI workspace structure:

- Source: `src/nfw/src/nframework-nfw/`
- Tests: `src/nfw/tests/integration/`

---

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create feature directory structure in nfw workspace

- [x] T001 Create entity_generation feature directory structure in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/
- [x] T002 [P] Create entity_generation feature directory structure in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/
- [x] T003 [P] Create entity_generation feature directory structure in src/nfw/src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/features/entity_generation/
- [x] T004 Create entity_generation test directory structure in src/nfw/tests/integration/entity_generation/

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Core domain entities and error types that ALL user stories depend on

**⚠️ CRITICAL**: No user story work can begin until this phase is complete

- [x] T005 [P] Create AddEntityCommand entity in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/entities/add_entity_command.rs
- [x] T006 [P] Create PropertyDefinition value object in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/value_objects/property_definition.rs
- [x] T007 [P] Create GeneralType enum in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/value_objects/general_type.rs
- [x] T008 [P] Create EntityGenerationParameters value object in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/value_objects/entity_generation_parameters.rs
- [x] T009 [P] Create EntityGenerationError enum in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/errors/entity_generation_error.rs
- [x] T010 [P] Create EntitySchema entity in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/entities/entity_schema.rs
- [x] T011 [P] Create ServiceInfo value object in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/value_objects/service_info.rs
- [x] T012 [P] Create WorkspaceContext value object in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/value_objects/workspace_context.rs

**Checkpoint**: Foundation ready - user story implementation can now begin in parallel

---

## Phase 3: User Story 1 - Generate Entity with Properties (Priority: P1) 🎯 MVP

**Goal**: CLI command that creates entity schema files and invokes template engine for code generation

**Independent Test**: Run `nfw gen entity Product --props Name:string,Price:decimal --no-input` and verify that (1) schema file is created in entity-specs_path, (2) template engine is invoked, (3) entity code is generated in configured output directory

### Integration Tests for User Story 1

> **NOTE: Write these tests FIRST, ensure they FAIL before implementation**

- [x] T013 [P] [US1] Integration test for successful entity generation in src/nfw/tests/integration/entity_generation/success_tests.rs
- [x] T014 [P] [US1] Integration test for error scenarios in src/nfw/tests/integration/entity_generation/error_tests.rs
- [x] T015 [P] [US1] Integration test for schema file operations in src/nfw/tests/integration/entity_generation/schema_tests.rs

### Implementation for User Story 1

#### Application Layer (Services & Commands)

- [x] T016 [P] [US1] Create EntityTypeResolver trait in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/abstractions/entity_type_resolver.rs
- [x] T017 [P] [US1] Create EntityNameValidator trait in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/abstractions/entity_name_validator.rs
- [x] T018 [P] [US1] Implement PropertySyntaxParser service in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/services/property_syntax_parser.rs
- [x] T019 [P] [US1] Implement ServiceModuleValidator service in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/services/service_module_validator.rs
- [x] T020 [US1] Implement EntitySchemaGenerator service in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/services/entity_schema_generator.rs
- [x] T021 [US1] Implement EntitySchemaReader service in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/services/entity_schema_reader.rs
- [x] T022 [US1] Implement AddEntityCommandHandler in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/commands/add_entity_command_handler.rs (depends on T016-T021)

#### Infrastructure Layer (File System Adapters)

- [x] T023 [P] [US1] Implement schema file write adapter in src/nfw/src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/features/entity_generation/adapters/schema_file_writer.rs
- [x] T024 [P] [US1] Implement schema file read adapter in src/nfw/src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/features/entity_generation/adapters/schema_file_reader.rs

#### Presentation Layer (CLI Command)

- [x] T025 [US1] Implement gen_entity CLI command with clap in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/commands/gen_entity.rs (depends on T022, T023, T024)
- [x] T026 [US1] Register gen_entity command in CLI mod.rs in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/mod.rs
- [x] T027 [US1] Add --props, --service, --id-type, --entity-type, --schema-only, --from-schema, --no-input flags to gen_entity command in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/commands/gen_entity.rs

**Checkpoint**: At this point, User Story 1 should be fully functional - users can run `nfw gen entity` to create schemas and invoke template engine

---

## Phase 4: User Story 2 - Entity Schema File Management (Priority: P2)

**Goal**: Language-agnostic schema files for polyglot code generation

**Independent Test**: Run `nfw gen entity Product --from-schema --no-input` and verify that entity code is generated from existing schema file; manually edit schema file and verify regeneration uses updated values

### Implementation for User Story 2

**Note**: User Story 2 builds on US1 foundation. Schema creation/reading services are implemented in US1; this phase focuses on schema-first workflow refinements.

- [x] T028 [P] [US2] Add schema validation logic in EntitySchemaReader service in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/services/entity_schema_reader.rs
- [x] T029 [P] [US2] Implement schema conflict detection in EntitySchemaGenerator service in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/services/entity_schema_generator.rs
- [x] T030 [US2] Add --schema-only flag handling in AddEntityCommandHandler in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/commands/add_entity_command_handler.rs
- [x] T031 [US2] Add --from-schema flag handling in AddEntityCommandHandler in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/commands/add_entity_command_handler.rs
- [x] T032 [US2] Implement auto-creation of entity-specs-path directory in EntitySchemaGenerator service in src/nfw/src/nframework-nfw/core/nframework-nfw-application/features/entity_generation/services/entity_schema_generator.rs
- [x] T033 [US2] Add integration test for schema conflict scenario in src/nfw/tests/integration/entity_generation/schema_tests.rs
- [x] T034 [US2] Add integration test for --schema-only mode in src/nfw/tests/integration/entity_generation/schema_tests.rs
- [x] T035 [US2] Add integration test for --from-schema mode in src/nfw/tests/integration/entity_generation/schema_tests.rs

**Checkpoint**: At this point, User Stories 1 AND 2 should both work - users can use both quick-start and schema-first workflows

---

## Phase 5: Polish & Cross-Cutting Concerns

**Purpose**: Documentation, validation, and cross-cutting improvements

- [x] T036 [P] Verify quickstart.md examples end-to-end in terminal
- [x] T037 Refactor CLI command layer for robustness in src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/commands/gen/entity_command.rs
- [x] T038 Extract common file path processing functions (if any exist) into utility modules
- [x] T039 Add inline documentation to public APIs in core-domain and core-application features/entity_generation/
- [x] T040 Add diagnostic error messages for all EntityGenerationError variants in src/nfw/src/nframework-nfw/core/nframework-nfw-domain/features/entity_generation/errors/entity_generation_error.rs
- [x] T041 Verify performance target (command completes in <50ms natively)
- [x] T042 Update project CLAUDE.md guidelines if applicable with new patterns from this feature

---

## Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 2)**: Depends on Setup completion - BLOCKS all user stories
- **User Story 1 (Phase 3)**: Depends on Foundational phase completion
- **User Story 2 (Phase 4)**: Depends on User Story 1 completion (builds on US1 services)
- **Polish (Phase 5)**: Depends on US1 and US2 completion

### User Story Dependencies

- **User Story 1 (P1)**: Can start after Foundational (Phase 2) - No dependencies on other stories
- **User Story 2 (P2)**: Depends on User Story 1 completion (extends US1 services)

### Within Each User Story

- Integration tests MUST be written and FAIL before implementation (T013-T015 before T016-T027)
- Services (T016-T021) before Handler (T022)
- Infrastructure adapters (T023-T024) before Handler (T022)
- CLI command (T025-T027) depends on Handler (T022)

### Parallel Opportunities

**Setup Phase (Phase 1)**:

```bash
# These can all run in parallel:
Task: T001 - Domain directories
Task: T002 - Application directories
Task: T003 - Infrastructure directories
```

**Foundational Phase (Phase 2)**:

```bash
# All domain entities and value objects can run in parallel:
Task: T005 - AddEntityCommand
Task: T006 - PropertyDefinition
Task: T007 - GeneralType
Task: T008 - EntityGenerationParameters
Task: T009 - EntityGenerationError
Task: T010 - EntitySchema
Task: T011 - ServiceInfo
Task: T012 - WorkspaceContext
```

**User Story 1 Tests (Phase 3)**:

```bash
# All tests can be written in parallel:
Task: T013 - Success tests
Task: T014 - Error tests
Task: T015 - Schema tests
```

**User Story 1 Application Services (Phase 3)**:

```bash
# All traits/services can be created in parallel:
Task: T016 - EntityTypeResolver trait
Task: T017 - EntityNameValidator trait
Task: T018 - PropertySyntaxParser service
Task: T019 - ServiceModuleValidator service
Task: T020 - EntitySchemaGenerator service (partial)
Task: T021 - EntitySchemaReader service (partial)
```

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: Foundational (CRITICAL - blocks US1)
3. Complete Phase 3: User Story 1
4. **STOP and VALIDATE**: Test `nfw gen entity Product --props Name:string,Price:decimal --no-input` independently
5. Verify schema creation and template invocation
6. Deploy/demo if ready

### Incremental Delivery

1. Complete Setup + Foundational → Foundation ready
2. Add User Story 1 → Test independently → Deploy/Demo (MVP: basic entity generation)
3. Add User Story 2 → Test independently → Deploy/Demo (schema-first workflow)

### Parallel Team Strategy

With multiple developers:

1. Team completes Setup + Foundational together
2. Once Foundational is done:
   - Developer A: US1 Integration tests (T013-T015)
   - Developer B: US1 Application services (T016-T021)
   - Developer C: US1 Infrastructure adapters (T023-T024)
3. Once US1 services are complete:
   - Developer A: US1 CLI command (T025-T027)
   - Developer B: Begin US2 enhancements (T028-T035)

---

## Format Validation

✅ All tasks follow the checklist format:

- Checkbox: `- [ ]`
- Task ID: Sequential (T001-T042)
- [P] marker: Present for parallelizable tasks
- [Story] label: Present for US1 and US2 tasks
- File paths: Included in all task descriptions

✅ Total task count: 42 tasks

- Phase 1 (Setup): 4 tasks
- Phase 2 (Foundational): 8 tasks
- Phase 3 (US1): 15 tasks (3 tests + 12 implementation)
- Phase 4 (US2): 8 tasks (3 tests + 5 implementation)
- Phase 5 (Polish): 7 tasks

✅ MVP scope (US1 only): 27 tasks (Phase 1 + Phase 2 + Phase 3)

✅ Parallel opportunities identified:

- 3 tasks in Setup phase can run in parallel
- 8 tasks in Foundational phase can run in parallel
- 3 tests in US1 can run in parallel
- 6 services/adapters in US1 can run in parallel
- US1 tests and services can run in parallel with infrastructure
