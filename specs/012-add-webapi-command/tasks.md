# Implementation Tasks: Add WebAPI Command

## Phase 1: Setup

**Goal**: Project initialization and basic command registration.

- [x] T001 [P] Create command registration in `src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/webapi/registration.rs`
- [x] T002 [P] Create command module definition in `src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/webapi/mod.rs`
- [x] T003 Update parent registration in `src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/registration.rs`
- [x] T004 Update parent module in `src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/mod.rs`
- [x] T005 [P] Create application command payload in `src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/add_webapi/add_webapi_command.rs`
- [x] T006 [P] Create application command module in `src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/add_webapi/mod.rs`

## Phase 2: Foundational

**Goal**: Define template files and core handler skeletons.

- [x] T007 [P] Create Minimal API startup and route registration extension templates in `src/nfw-templates/src/dotnet-service/webapi/`
- [x] T008 [P] Create CORS/ProblemDetails middleware templates in `src/nfw-templates/src/dotnet-service/webapi/`
- [x] T009 [P] Create OpenAPI/Swagger templates in `src/nfw-templates/src/dotnet-service/webapi/`
- [x] T010 [P] Create HealthCheck endpoint templates in `src/nfw-templates/src/dotnet-service/webapi/`
- [x] T011 [P] Register `webapi` generator in `src/nfw-templates/src/dotnet-service/template.yaml`

## Phase 3: User Story 2 - Add WebAPI via Automation

**Goal**: Support head-less adding of the WebAPI module for CI/CD via `--service` and `--no-input`.
**Independent Test**: Run `nfw add webapi --service <Name> --no-input` and verify templates are copied and `nfw.yaml` is updated correctly.

- [x] T012 [US2] Implement template rendering logic in `src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/add_webapi/add_webapi_command_handler.rs`
- [x] T013 [US2] Implement `nfw.yaml` comment-preserving update logic in `src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/add_webapi/add_webapi_command_handler.rs`
- [x] T014 [US2] Implement non-interactive execution logic in `src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/webapi/handler.rs`
- [x] T015 [US2] Register `AddWebApiCommandHandler` in `src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/startup/cli_service_collection_factory.rs`

## Phase 4: User Story 1 - Add WebAPI Interactively

**Goal**: Provide interactive service selection prompts for a better developer experience.
**Independent Test**: Run `nfw add webapi` without arguments and interactively select the service.

- [x] T016 [US1] Implement interactive prompt logic for service selection using `cliclack` in `src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/webapi/handler.rs`
- [x] T017 [US1] Add spinner and success/error logging to `src/nfw/src/n-framework-nfw/presentation/n-framework-nfw-cli/src/commands/add/webapi/handler.rs`

## Phase 5: User Story 3 - Safe Rollback on Failure

**Goal**: Guarantee workspace integrity if template rendering or configuration updates fail.
**Independent Test**: Force a failure (e.g., missing template) and verify no dangling files or modified configurations are left behind.

- [x] T018 [US3] Implement `GenerationTransaction` file tracking in `src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/add_webapi/add_webapi_command_handler.rs`
- [x] T019 [US3] Implement configuration backup and restore logic on error in `src/nfw/src/n-framework-nfw/core/n-framework-nfw-core-application/src/features/template_management/commands/add_webapi/add_webapi_command_handler.rs`

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: Complete test coverage and finalize execution.

- [x] T020 [P] Write integration tests for successful interactive execution in `src/nfw/tests/integration/n-framework-nfw/features/artifact/webapi_add_test.rs`
- [x] T021 [P] Write integration tests for `--no-input` automated execution in `src/nfw/tests/integration/n-framework-nfw/features/artifact/webapi_add_test.rs`
- [x] T022 [P] Write integration tests for rollback behavior in `src/nfw/tests/integration/n-framework-nfw/features/artifact/webapi_add_test.rs`
- [x] T023 [P] Write integration tests for YAML comment preservation in `src/nfw/tests/integration/n-framework-nfw/features/artifact/webapi_add_test.rs`

## Dependencies

- Phase 1 & 2 must be completed before User Story implementation begins.
- US2 (Automation) provides the core application logic and handler structure.
- US1 (Interactive) builds upon the handler logic provided by US2.
- US3 (Rollback) wraps the previously built logic with transaction guarantees.
- Phase 6 (Tests) validates all completed user stories.

## Implementation Strategy

Start by scaffolding the CLI command registration and the templates (Phases 1 and 2). Then move to building the core generation logic for automated usage (US2) because this provides the essential backend implementation. Once the core generation works, add the interactive UI (US1) to wrap it. Finally, add the transaction/rollback mechanism (US3) to handle edge cases safely.
