# Feature Specification: Gen Endpoint Command

## User Scenarios & Testing

### User Story 1 - Generate Endpoint for Existing Command/Query (Priority: P1)

As a .NET developer, I want to generate a Minimal API HTTP endpoint for an existing MediatR command or query so I can easily expose application logic over HTTP without writing boilerplate routing and mapping code.

**Why this priority**: Exposing application features over HTTP is a core and frequent workflow.

**Independent Test**: Can be tested by running `nfw gen endpoint GET GetInventoryItem Inventory` in a workspace containing the specified MediatR query, and verifying that the endpoint file is created and the route is registered.

**Acceptance Scenarios**:

1. **Given** a service with an existing query `GetInventoryItem` in feature `Inventory`, **When** I run `nfw gen endpoint GET GetInventoryItem Inventory`, **Then** a GET endpoint file is created in the API layer.
2. **Given** a service with an existing command `CreateInventoryItem` in feature `Inventory`, **When** I run `nfw gen endpoint POST CreateInventoryItem Inventory`, **Then** a POST endpoint file is created in the API layer.

---

### User Story 2 - Validation of Referenced Command/Query (Priority: P1)

As a developer, I want the CLI to validate that the referenced command or query actually exists before generating the endpoint so I don't scaffold broken code that references missing application logic.

**Why this priority**: Generating code that doesn't compile due to missing references degrades the developer experience.

**Independent Test**: Can be tested by running the command for a nonexistent feature or command and asserting that it fails with a validation error.

**Acceptance Scenarios**:

1. **Given** a service without the feature `NonExistentFeature`, **When** I run `nfw gen endpoint GET GetItem NonExistentFeature`, **Then** the command fails with a "Feature not found" error.
2. **Given** a service with feature `Inventory` but missing the `UpdateItem` command, **When** I run `nfw gen endpoint PUT UpdateItem Inventory`, **Then** the command fails with a "Command/Query not found" error.

---

### Edge Cases

- **Operation Type Validation**: Specifying an invalid operation type (e.g., `PATCH` if not supported, or `FETCH`) should fail argument validation.
- **Service Selection**: If multiple services exist and no service is specified, it should prompt the user to select one.
- **Endpoint Already Exists**: If the endpoint file already exists, the command should fail or warn instead of overwriting custom code.
- **Missing API Layer**: If the target service does not have an API layer (WebAPI module), the command should fail with a helpful error suggesting `nfw add webapi`.

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST provide an `nfw gen endpoint <OPERATION_TYPE> <NAME> <FEATURE>` command to generate HTTP endpoint boilerplate.
- **FR-002**: The `<OPERATION_TYPE>` MUST support and validate standard HTTP verbs: GET, POST, PUT, DELETE.
- **FR-003**: The command MUST discover and validate that the `<NAME>` references an existing Command or Query in the target `<FEATURE>` folder.
- **FR-004**: The `<FEATURE>` MUST reference an existing feature folder in the application layer.
- **FR-005**: The generated endpoint MUST use Minimal API route definitions with proper attribute routing (e.g., `app.MapGet(...)`). The default route pattern MUST be `"/api/{feature.ToLower()}/{name.ToLower()}"`, avoiding assumptions about specific path parameters.
- **FR-006**: The generated endpoint MUST include OpenAPI/Swagger documentation annotations (e.g., `Produces`, `WithTags`, `WithName`).
- **FR-007**: The generated files MUST be placed in the API layer following project conventions (e.g., `Api/Endpoints/{Feature}/{Name}Endpoint.cs` or similar).
- **FR-008**: The command MUST complete in under 3 seconds.
- **FR-009**: The command MUST include integration tests covering the generation and validation behavior.

### Key Entities

- **Endpoint Artifacts**: Minimal API endpoint files, route registrations, and request/response mappers if necessary.
- **Application Command/Query**: The existing MediatR requests that the endpoints will map HTTP requests to.

## Success Criteria

### Measurable Outcomes

- **SC-001**: The command completes the entire generation process in under 3 seconds.
- **SC-002**: 100% of integration tests pass correctly.
- **SC-003**: A generated endpoint compiles immediately and clearly calls `ISender.Send(...)` with the mapped request.
- **SC-004**: Operation type correctly dictates the generated HTTP verb (GET -> `MapGet`, POST -> `MapPost`, etc.).

## Assumptions

- The target service has an API layer (WebAPI) using Minimal APIs (e.g., native endpoint groups).
- The Application layer uses MediatR for commands and queries, organized by Feature folders.
- Template rendering engine is available to handle endpoint file generation.

## Non-Goals

- Refactoring or altering the existing Command/Query.
- Implementing the actual mapping logic properties within the endpoint (only the boilerplate is generated).
- Supporting GraphQL or gRPC endpoints.
