# Feature Specification: Generator-Based `nfw add service`

## User Scenarios & Testing

### User Story 1 - Generate a Service from Generator (Priority: P1)

As a developer, I want to run `nfw add service <name> --generator <id>` so that I can create a new service from an NFramework service generator without manual project setup.

**Why this priority**: Service scaffolding is a beta release gate workflow. If this path is not deterministic and fast, downstream command/query generation and architecture checks cannot be trusted.

**Independent Test**: In a valid workspace, run `nfw add service Orders --generator official/dotnet-service --no-input` and verify the service folder and projects are generated from the selected generator and compile immediately.

**Acceptance Scenarios**:

1. **Given** an NFramework workspace and available service generators, **When** I run `nfw add service Orders --generator official/dotnet-service`, **Then** a new service named `Orders` is generated using the selected service generator.
2. **Given** successful generation, **When** I inspect created projects, **Then** the service is created at `src/Orders/` and contains `Domain`, `Application`, `Infrastructure`, and `Api` layers.
3. **Given** successful generation, **When** I build the service solution, **Then** it builds without manual edits.

---

### User Story 2 - Enforce Layer Dependency Rules (Priority: P1)

As a tech lead, I want generated generators to already contain the intended layer dependencies so that clean architecture boundaries are preserved from the first commit.

**Why this priority**: The value of scaffold generation depends on enforcing architectural direction; invalid defaults create long-term coupling debt.

**Independent Test**: Generate from the official service generator and inspect project references in generated output.

**Acceptance Scenarios**:

1. **Given** a generated service from the official service generator, **When** I inspect project references, **Then** the generated baseline matches the generator's intended clean architecture dependency layout.
2. **Given** a generated service, **When** I inspect CLI behavior, **Then** the command does not perform language-specific project-reference validation.

---

### User Story 3 - Include Baseline Health Endpoints (Priority: P2)

As an operator, I want generated services to expose health endpoints so that I can quickly validate runtime readiness.

**Why this priority**: Health endpoints are required for operational baseline but are secondary to basic scaffold correctness.

**Independent Test**: Generate from the official service generator and inspect API source files for baseline health endpoint mappings.

**Acceptance Scenarios**:

1. **Given** a generated service from the official service generator, **When** I inspect generated API source files, **Then** health endpoints are present by default.
2. **Given** the CLI scope, **When** generation completes, **Then** endpoint runtime behavior verification is owned by the generated service tests, not by `nfw` command logic.

---

### User Story 4 - Reuse Existing Generator System (Priority: P1)

As a platform engineer, I want `nfw add service` to use the same generator source and versioning model as `nfw new` so that generator governance remains consistent.

**Why this priority**: Maintaining separate generator paths increases drift risk and duplicated implementation complexity.

**Independent Test**: Register or refresh generator sources once, run `nfw add service`, and verify service generation resolves generators through the same generator catalog and cache used by `nfw new`.

**Acceptance Scenarios**:

1. **Given** service generators exist in `nfw-generators`, **When** `nfw add service <name> --generator <id>` is executed, **Then** the command resolves the generator via the existing generator catalog and cache workflow.
2. **Given** generator version metadata is available, **When** service generation selects a generator, **Then** resolution follows the same version rules as other generator-backed commands.
3. **Given** the service generator is unavailable or invalid, **When** generation starts, **Then** the command exits non-zero with an actionable generator-resolution error.
4. **Given** an interactive terminal and no `--generator` flag, **When** I run `nfw add service <name>`, **Then** the CLI prompts me to select a service generator before generation.
5. **Given** non-interactive mode and no `--generator` flag, **When** I run `nfw add service <name> --no-input`, **Then** the command fails before generation with an actionable missing-generator error.

## Edge Cases

- **Missing workspace marker**: If `nfw add service` runs outside a workspace (no `nfw.yaml` in current or parent path), the command fails with remediation guidance.
- **Duplicate service name**: If a service folder already exists for `<name>`, the command fails before writing files.
- **Missing generator selection**: If no generator is provided in non-interactive mode, the command fails and explains how to pass `--generator <id>`.
- **Unsupported generator type**: If the provided generator ID exists but is not a service generator (`type=service`), the command fails with a generator-type error.
- **Generator render failure**: If placeholder rendering fails, generation aborts and partial output is cleaned up.
- **Interrupted generation**: If interrupted with Ctrl+C, partially created files for the target service are removed and the process exits with code 130 on Unix-like systems.

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST implement `nfw add service <name>` for workspace-local service generation.
- **FR-002**: The command MUST execute only inside an NFramework workspace identified by `nfw.yaml`.
- **FR-003**: The command MUST generate exactly four layer projects for the initial service baseline generator: `Domain`, `Application`, `Infrastructure`, and `Api`.
- **FR-003b**: The generated service root directory MUST be `src/<ServiceName>/`.
- **FR-004**: The generated layer structure MUST be created from a selected service generator resolved through the existing generator system (catalog, source registration, cache, metadata validation, and version resolution).
- **FR-004b**: The command MUST support `--generator <id>` to select a service generator explicitly.
- **FR-004c**: In interactive terminals, if `--generator` is not provided, the CLI MUST prompt for service generator selection.
- **FR-004d**: In non-interactive execution (including `--no-input`), `--generator <id>` MUST be provided or the command MUST fail before file generation.
- **FR-004e**: The command MUST allow only generators explicitly marked as service generators (`type=service`) and MUST reject other generator types before rendering.
- **FR-005**: Service generators MUST be sourceable from the `nfw-generators` repository through the configured generator sources and rendered into the target workspace.
- **FR-006**: The command MUST remain language-agnostic and MUST NOT implement language-specific project-reference validation logic.
- **FR-007**: The generated API project baseline (including health endpoints) MUST come from the selected service generator content.
- **FR-008**: The generated service MUST compile on first build without manual edits.
- **FR-009**: The command MUST validate service names using documented naming rules before file generation starts.
- **FR-010**: The command MUST fail with actionable errors for generator resolution failure, invalid inputs, invalid generator selection, and invalid workspace context.
- **FR-011**: The command MUST clean up partially generated service files when generation is interrupted or fails after filesystem writes begin.
- **FR-012**: Service generator rendering MUST support placeholder substitution for service name and namespace values consistent with the existing generator placeholder conventions.
- **FR-013**: After successful generation, the CLI MUST create a per-service generator provenance record that includes selected generator ID and resolved generator version.
- **FR-014**: Service generator provenance records MUST be stored per service under the workspace `nfw.yaml` file.

### Key Entities

- **Service Generation Request**: Parsed command input containing service name, selected generator, and execution mode.
- **Service Generator Selection**: Resolved generator identity and version used for service scaffolding.
- **Layer Dependency Matrix**: Allowed reference graph between generated `Domain`, `Application`, `Infrastructure`, and `Api` projects.
- **Generated Service Artifact Set**: Files and folders materialized into the workspace for a specific service under `src/<ServiceName>/`.
- **Service Generator Provenance Record**: Workspace metadata entry that stores generator ID and resolved generator version used for a generated service.
- **Health Endpoint Contract**: Default liveness/readiness routes included in generated APIs.

## Success Criteria

### Measurable Outcomes

- **SC-001**: In acceptance tests, 100% of valid `nfw add service <name> --generator <id>` runs generate all four required layers.
- **SC-002**: In acceptance tests, 100% of generated services match the selected generator output structure without additional CLI-side language validation.
- **SC-003**: In acceptance tests, 100% of generated services compile successfully without manual edits.
- **SC-004**: In acceptance tests, generated API source includes baseline health endpoint mappings from the selected generator.
- **SC-005**: In failure-path tests, command errors are actionable and deterministic for invalid workspace context, invalid name, missing generator, and invalid generator ID.
- **SC-006**: In acceptance tests, 100% of generated services include persisted generator provenance (generator ID and resolved version) in workspace metadata.
- **SC-007**: In acceptance tests, 100% of generated services persist generator provenance in `nfw.yaml` under the created service entry.

## Assumptions

- The generator discovery and source-management model from `001-nfw-generator-system` is already implemented and reused.
- Service generators are maintained in `nfw-generators` and include the canonical .NET service baseline generator.
- The .NET service baseline continues to target the same runtime and naming conventions used by current workspace specs.

## Dependencies

- `docs/PRD.md` US-002 and related FRs for .NET service scaffolding expectations.
- `docs/ROADMAP.md` Phase 1 Milestone M2 (`nfw add service`) and service baseline deliverables.
- `docs/SPECIFICATION_GUIDELINES.md` for spec quality and structure rules.
- `src/nfw/specs/001-nfw-generator-system/spec.md` for shared generator source, cache, metadata, and version rules.
- `src/nfw/specs/002-workspace-structure-new-command/spec.md` for workspace detection and CLI behavior conventions.

## Clarifications

- Q: Should `nfw add service` use a separate scaffolding mechanism or the existing generator system? → A: Use the existing generator system.
- Q: Where should service generators be maintained? → A: In `nfw-generators`, then rendered into the target NFramework workspace by the CLI.
- Q: Should `nfw add service` use `--lang` or generator selection? → A: Remove `--lang`; select service generator via `--generator <id>`.

### Session 2026-04-04

- Q: Which service destination path should be standardized? → A: `src/<ServiceName>/`.
- Q: Should generator selection be optional in automation? → A: Require `--generator` in non-interactive mode; prompt in interactive mode.
- Q: Which generator eligibility rule should apply to `nfw add service`? → A: Only generators with `type=service` metadata.
- Q: Should the generated service generator version be persisted? → A: Yes, persist generator ID + resolved version in workspace metadata.
- Q: Where should per-service generator provenance be stored? → A: Under `nfw.yaml`.

## Non-Goals

- Defining command/query/entity/CRUD generation behavior.
- Defining architecture validation command behavior (`nfw check`) beyond generated default compliance.
- Defining non-.NET service generation behavior in this spec.
- Introducing a second generator registry or a service-only generator lifecycle separate from existing generator management.
