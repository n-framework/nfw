# Tasks: Gen Endpoint Command

**Input**: Design documents from `/specs/013-gen-endpoint-command/`
**Prerequisites**: plan.md (required), spec.md (required for user stories), research.md, data-model.md, quickstart.md

**Organization**: Tasks are grouped by user story to enable independent implementation and testing of each story.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (e.g., US1, US2)
- Include exact file paths in descriptions

## Path Conventions

- Paths shown below assume single project repository logic in `src/nfw/` and generator directory `src/nfw-generators/`

---

## Phase 1: Setup & Foundational (Shared Infrastructure)

**Purpose**: Command registration, project setup

- [x] T001 Initialize endpoint generator configuration `src/nfw-generators/src/dotnet-service/endpoint/nfw.generator.yaml`
- [x] T002 In `src/nfw-generators/src/dotnet-service/nfw.generator.yaml`, register the new generator under `generators: endpoint: ./endpoint/`
- [x] T003 Create Tera generator for the endpoint in `src/nfw-generators/src/dotnet-service/endpoint/Endpoint.cs.tera` (including Map{OperationType}, ISender, OpenAPI mapping)

**Checkpoint**: Foundation ready - generator and config scaffolded natively.

---

## Phase 2: User Story 1 - Generate Endpoint for Existing Command/Query (Priority: P1) 🎯 MVP

**Goal**: As a .NET developer, I want to generate a Minimal API HTTP endpoint for an existing MediatR command or query so I can easily expose application logic over HTTP without writing boilerplate routing and mapping code.

**Independent Test**: Can be tested by running `nfw gen endpoint GET GetInventoryItem Inventory` in a workspace containing the specified MediatR query, and verifying that the endpoint file is created and the route is registered.

### Implementation for User Story 1

- [x] T004 [US1] Create command handler logic via clean architecture layout: Application layer command `GenEndpointCommand` and `GenEndpointCommandHandler`
- [x] T005 [US1] Create presentation endpoint in `src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/gen/endpoint/` via `GenEndpointCliCommand` to capture operation bounds over GET/POST/PUT/DELETE
- [x] T006 [US1] Hook `endpoint` submodule to `src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/gen/registration.rs` Command mapping
- [x] T007 [US1] Integration tests for happy path GET, POST, PUT, DELETE in test directories

**Checkpoint**: At this point, User Story 1 should be fully functional provided the application is structured normally.

---

## Phase 3: User Story 2 - Validation of Referenced Command/Query (Priority: P1)

**Goal**: As a developer, I want the CLI to validate that the referenced command or query actually exists before generating the endpoint so I don't scaffold broken code that references missing application logic.

**Independent Test**: Can be tested by running the command for a nonexistent feature or command and asserting that it fails with a validation error.

### Implementation for User Story 2

- [x] T008 [US2] In `GenEndpointCommandHandler`, implement validation checking that target application handles Minimal APIs
- [x] T009 [US2] In `GenEndpointCommandHandler`, implement validation checking that `Application/Features/{feature}` directory exists using Workspace utilities
- [x] T010 [US2] In `GenEndpointCommandHandler`, implement validation checking that specified command/query actually exists in the target feature directory
- [x] T011 [US2] Add integration test for nonexistent feature error case
- [x] T012 [US2] Add integration test for nonexistent command/query error case
- [x] T013 [US2] Add edge case validation to block overwriting an existing endpoint file
- [x] T014 [US2] Implement interactive service selection prompt if multiple services exist and no service is specified

**Checkpoint**: At this point, User Story 2 validation constraints will safeguard CLI users.

---

## Expected Dependencies & Execution Order

### Phase Dependencies

- **Setup (Phase 1)**: No dependencies - can start immediately
- **Foundational (Phase 1)**: Depends on Setup completion - BLOCKS all user stories
- **User Stories (Phase 2+)**: All depend on Foundational phase completion
  - Sequential in priority order (US1 → US2)

### Within Each User Story

- Arg structures before business logic
- Business logic implementation before testing

### Parallel Opportunities

- Within Phase 1: T002 and T003 can be built essentially decoupled from Rust code (T001 is a base)
- Within User Story 2: Validation features (T008, T009, T010) are sequentially independent logic chunks in the `handler` func.

---

## Implementation Strategy

### MVP First (User Story 1 Only)

1. Complete Phase 1: Setup
2. Complete Phase 2: User Story 1
3. **STOP and VALIDATE**: Test User Story 1 independently against existing generators

### Incremental Delivery

1. Verify User Story 1 MVP works
2. Add validations for User Story 2
3. Each validation block narrows down scaffold edge-case failures
