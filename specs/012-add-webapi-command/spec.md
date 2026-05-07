# Feature Specification: Add WebAPI Command

## User Scenarios & Testing

### User Story 1 - Add WebAPI Interactively (Priority: P1)

As a .NET developer, I want to interactively add a WebAPI module to an existing service so I can expose my application layer through HTTP endpoints without manual wiring.

**Why this priority**: Interactive usage is the primary developer experience for discovering and extending services in the CLI.

**Independent Test**: Can be tested by running the command without arguments in a workspace with multiple services, selecting a service, and verifying the generated API layer.

**Acceptance Scenarios**:

1. **Given** a workspace with at least one service, **When** I run `nfw add webapi`, **Then** I am prompted to select a target service from a list.
2. **Given** a workspace with at least one service, **When** I complete the interactive prompts, **Then** the WebAPI module artifacts are generated and `nfw.yaml` is updated.

---

### User Story 2 - Add WebAPI via Automation (Priority: P1)

As a platform engineer, I want to add a WebAPI module to a specific service using command-line arguments so I can automate service scaffolding in CI/CD or template scripts.

**Why this priority**: Headless/automated execution is critical for platform tooling and non-interactive scripts.

**Independent Test**: Can be tested by running the command with `--service <name>` and `--no-input` and verifying the artifacts are generated without any prompts.

**Acceptance Scenarios**:

1. **Given** a workspace with a service named "Inventory", **When** I run `nfw add webapi --service Inventory --no-input`, **Then** the WebAPI module is generated immediately without prompts.
2. **Given** a workspace with no services, **When** I run `nfw add webapi --no-input`, **Then** the command fails with a clear error indicating a target service is required.

---

### User Story 3 - Safe Rollback on Failure (Priority: P2)

As a developer, I want the command to cleanly roll back changes if generation fails so my workspace is not left in a broken or partially updated state.

**Why this priority**: Preserving workspace integrity is crucial for developer trust and avoiding manual cleanup of broken scaffolds.

**Independent Test**: Can be tested by forcing a failure during template rendering and verifying the filesystem and configuration revert to their exact original state.

**Acceptance Scenarios**:

1. **Given** a service with custom YAML comments in `nfw.yaml`, **When** the `add webapi` command fails midway, **Then** all partially generated files are removed and `nfw.yaml` is restored exactly, preserving all comments.
2. **Given** an invalid template configuration, **When** I run `nfw add webapi`, **Then** the command aborts, reports the specific error, and leaves the workspace untouched.

---

## Edge Cases

- **Service does not exist**: When the specified `--service` does not exist in the workspace, the command should fail with a "Service not found" error.
- **WebAPI already exists**: When the target service already has a WebAPI module, the command should fail or warn instead of overwriting existing custom code.
- **Invalid workspace**: When run outside a valid NFramework workspace, the command should fail immediately.
- **File permission error**: When the CLI lacks write permissions to the service directory, it should cleanly abort and trigger a rollback.
- **Interrupt signal (Ctrl+C)**: When the user sends Ctrl+C during the interactive prompt or generation, the process should abort safely.

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST provide an `nfw add webapi` command to scaffold the API layer for an existing service.
- **FR-002**: The command MUST prompt interactively for the target service if not provided as an argument.
- **FR-003**: The command MUST support a `--service <name>` argument to target a specific service directly.
- **FR-004**: The command MUST support a `--no-input` flag to disable interactive prompts for automation.
- **FR-005**: The command MUST update the `nfw.yaml` configuration to register the new WebAPI module for the targeted service.
- **FR-006**: The YAML configuration update MUST preserve existing structural comments upon successful update or rollback.
- **FR-007**: The command MUST execute template rendering to generate API layer artifacts.
- **FR-008**: Generated API artifacts MUST include Minimal API startup configuration and route registration extensions.
- **FR-009**: Generated API artifacts MUST include CORS middleware and problem details middleware.
- **FR-010**: Generated API artifacts MUST include health check endpoints.
- **FR-011**: Generated API artifacts MUST include OpenAPI/Swagger configuration.
- **FR-012**: The command MUST perform an automatic rollback (removing new files and restoring config) if template rendering or configuration updates fail.
- **FR-013**: The command MUST include integration tests covering successful addition, rollback on failure, service validation, and YAML comment preservation.

### Key Entities

- **WebAPI Artifacts**: The set of source code templates defining the Minimal API entry point, middleware pipeline, routing, and Swagger configuration.
- **Service Configuration**: The entry in `nfw.yaml` representing a service and its attached modules.

## Success Criteria

### Measurable Outcomes

- **SC-001**: The command completes the entire generation process (excluding user prompt time) in under 5 seconds.
- **SC-002**: 100% of integration tests for success, rollback, and YAML preservation pass reliably.
- **SC-003**: A generated WebAPI module builds successfully without any manual code modifications required.
- **SC-004**: After a simulated failure rollback, `git status` shows zero modified or untracked files related to the attempted command.

## Assumptions

- The target service has a standard NFramework structure (Domain, Application, Infrastructure) that the WebAPI layer can reference.
- Template rendering engine is already available within the CLI infrastructure to handle the file generation.
- The `nfw.yaml` file exists at the root of the workspace and follows the standard schema.

## Dependencies

- CLI core routing and prompt infrastructure.
- Template rendering engine capable of processing the API layer templates.
- YAML parser capable of preserving comments.

## Non-Goals

- Generating controller-based APIs (strictly Minimal APIs).
- Generating entity-specific CRUD endpoints (this is handled by `nfw add crud` or similar commands, not the base webapi module setup).
- Configuring deployment platforms or containers (Dockerfiles, etc.) as part of this command.
