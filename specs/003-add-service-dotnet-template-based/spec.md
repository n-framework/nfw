# Feature Specification: Template-Based `nfw add service`

## User Scenarios & Testing

### User Story 1 - Generate a Service from Template (Priority: P1)

As a developer, I want to run `nfw add service <name> --template <id>` so that I can create a new service from an NFramework service template without manual project setup.

**Why this priority**: Service scaffolding is a beta release gate workflow. If this path is not deterministic and fast, downstream command/query generation and architecture checks cannot be trusted.

**Independent Test**: In a valid workspace, run `nfw add service Orders --template official/dotnet-service --no-input` and verify the service folder and projects are generated from the selected template and compile immediately.

**Acceptance Scenarios**:

1. **Given** an NFramework workspace and available service templates, **When** I run `nfw add service Orders --template official/dotnet-service`, **Then** a new service named `Orders` is generated using the selected service template.
2. **Given** successful generation, **When** I inspect created projects, **Then** the service is created at `src/Orders/` and contains `Domain`, `Application`, `Infrastructure`, and `Api` layers.
3. **Given** successful generation, **When** I build the service solution, **Then** it builds without manual edits.

---

### User Story 2 - Enforce Layer Dependency Rules (Priority: P1)

As a tech lead, I want generated templates to already contain the intended layer dependencies so that clean architecture boundaries are preserved from the first commit.

**Why this priority**: The value of scaffold generation depends on enforcing architectural direction; invalid defaults create long-term coupling debt.

**Independent Test**: Generate from the official service template and inspect project references in generated output.

**Acceptance Scenarios**:

1. **Given** a generated service from the official service template, **When** I inspect project references, **Then** the generated baseline matches the template's intended clean architecture dependency layout.
2. **Given** a generated service, **When** I inspect CLI behavior, **Then** the command does not perform language-specific project-reference validation.

---

### User Story 3 - Include Baseline Health Endpoints (Priority: P2)

As an operator, I want generated services to expose health endpoints so that I can quickly validate runtime readiness.

**Why this priority**: Health endpoints are required for operational baseline but are secondary to basic scaffold correctness.

**Independent Test**: Generate from the official service template and inspect API source files for baseline health endpoint mappings.

**Acceptance Scenarios**:

1. **Given** a generated service from the official service template, **When** I inspect generated API source files, **Then** health endpoints are present by default.
2. **Given** the CLI scope, **When** generation completes, **Then** endpoint runtime behavior verification is owned by the generated service tests, not by `nfw` command logic.

---

### User Story 4 - Reuse Existing Template System (Priority: P1)

As a platform engineer, I want `nfw add service` to use the same template source and versioning model as `nfw new` so that template governance remains consistent.

**Why this priority**: Maintaining separate template paths increases drift risk and duplicated implementation complexity.

**Independent Test**: Register or refresh template sources once, run `nfw add service`, and verify service generation resolves templates through the same template catalog and cache used by `nfw new`.

**Acceptance Scenarios**:

1. **Given** service templates exist in `nfw-templates`, **When** `nfw add service <name> --template <id>` is executed, **Then** the command resolves the template via the existing template catalog and cache workflow.
2. **Given** template version metadata is available, **When** service generation selects a template, **Then** resolution follows the same version rules as other template-backed commands.
3. **Given** the service template is unavailable or invalid, **When** generation starts, **Then** the command exits non-zero with an actionable template-resolution error.
4. **Given** an interactive terminal and no `--template` flag, **When** I run `nfw add service <name>`, **Then** the CLI prompts me to select a service template before generation.
5. **Given** non-interactive mode and no `--template` flag, **When** I run `nfw add service <name> --no-input`, **Then** the command fails before generation with an actionable missing-template error.

## Edge Cases

- **Missing workspace marker**: If `nfw add service` runs outside a workspace (no `nfw.yaml` in current or parent path), the command fails with remediation guidance.
- **Duplicate service name**: If a service folder already exists for `<name>`, the command fails before writing files.
- **Missing template selection**: If no template is provided in non-interactive mode, the command fails and explains how to pass `--template <id>`.
- **Unsupported template type**: If the provided template ID exists but is not a service template (`type=service`), the command fails with a template-type error.
- **Template render failure**: If placeholder rendering fails, generation aborts and partial output is cleaned up.
- **Interrupted generation**: If interrupted with Ctrl+C, partially created files for the target service are removed and the process exits with code 130 on Unix-like systems.

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST implement `nfw add service <name>` for workspace-local service generation.
- **FR-002**: The command MUST execute only inside an NFramework workspace identified by `nfw.yaml`.
- **FR-003**: The command MUST generate exactly four layer projects for the initial service baseline template: `Domain`, `Application`, `Infrastructure`, and `Api`.
- **FR-003b**: The generated service root directory MUST be `src/<ServiceName>/`.
- **FR-004**: The generated layer structure MUST be created from a selected service template resolved through the existing template system (catalog, source registration, cache, metadata validation, and version resolution).
- **FR-004b**: The command MUST support `--template <id>` to select a service template explicitly.
- **FR-004c**: In interactive terminals, if `--template` is not provided, the CLI MUST prompt for service template selection.
- **FR-004d**: In non-interactive execution (including `--no-input`), `--template <id>` MUST be provided or the command MUST fail before file generation.
- **FR-004e**: The command MUST allow only templates explicitly marked as service templates (`type=service`) and MUST reject other template types before rendering.
- **FR-005**: Service templates MUST be sourceable from the `nfw-templates` repository through the configured template sources and rendered into the target workspace.
- **FR-006**: The command MUST remain language-agnostic and MUST NOT implement language-specific project-reference validation logic.
- **FR-007**: The generated API project baseline (including health endpoints) MUST come from the selected service template content.
- **FR-008**: The generated service MUST compile on first build without manual edits.
- **FR-009**: The command MUST validate service names using documented naming rules before file generation starts.
- **FR-010**: The command MUST fail with actionable errors for template resolution failure, invalid inputs, invalid template selection, and invalid workspace context.
- **FR-011**: The command MUST clean up partially generated service files when generation is interrupted or fails after filesystem writes begin.
- **FR-012**: Service template rendering MUST support placeholder substitution for service name and namespace values consistent with the existing template placeholder conventions.
- **FR-013**: After successful generation, the CLI MUST create a per-service template provenance record that includes selected template ID and resolved template version.
- **FR-014**: Service template provenance records MUST be stored per service under the workspace `nfw.yaml` file.

### Key Entities

- **Service Generation Request**: Parsed command input containing service name, selected template, and execution mode.
- **Service Template Selection**: Resolved template identity and version used for service scaffolding.
- **Layer Dependency Matrix**: Allowed reference graph between generated `Domain`, `Application`, `Infrastructure`, and `Api` projects.
- **Generated Service Artifact Set**: Files and folders materialized into the workspace for a specific service under `src/<ServiceName>/`.
- **Service Template Provenance Record**: Workspace metadata entry that stores template ID and resolved template version used for a generated service.
- **Health Endpoint Contract**: Default liveness/readiness routes included in generated APIs.

## Success Criteria

### Measurable Outcomes

- **SC-001**: In acceptance tests, 100% of valid `nfw add service <name> --template <id>` runs generate all four required layers.
- **SC-002**: In acceptance tests, 100% of generated services match the selected template output structure without additional CLI-side language validation.
- **SC-003**: In acceptance tests, 100% of generated services compile successfully without manual edits.
- **SC-004**: In acceptance tests, generated API source includes baseline health endpoint mappings from the selected template.
- **SC-005**: In failure-path tests, command errors are actionable and deterministic for invalid workspace context, invalid name, missing template, and invalid template ID.
- **SC-006**: In acceptance tests, 100% of generated services include persisted template provenance (template ID and resolved version) in workspace metadata.
- **SC-007**: In acceptance tests, 100% of generated services persist template provenance in `nfw.yaml` under the created service entry.

## Assumptions

- The template discovery and source-management model from `001-nfw-template-system` is already implemented and reused.
- Service templates are maintained in `nfw-templates` and include the canonical .NET service baseline template.
- The .NET service baseline continues to target the same runtime and naming conventions used by current workspace specs.

## Dependencies

- `docs/PRD.md` US-002 and related FRs for .NET service scaffolding expectations.
- `docs/ROADMAP.md` Phase 1 Milestone M2 (`nfw add service`) and service baseline deliverables.
- `docs/SPECIFICATION_GUIDELINES.md` for spec quality and structure rules.
- `src/nfw/specs/001-nfw-template-system/spec.md` for shared template source, cache, metadata, and version rules.
- `src/nfw/specs/002-workspace-structure-new-command/spec.md` for workspace detection and CLI behavior conventions.

## Clarifications

- Q: Should `nfw add service` use a separate scaffolding mechanism or the existing template system? → A: Use the existing template system.
- Q: Where should service templates be maintained? → A: In `nfw-templates`, then rendered into the target NFramework workspace by the CLI.
- Q: Should `nfw add service` use `--lang` or template selection? → A: Remove `--lang`; select service template via `--template <id>`.

### Session 2026-04-04

- Q: Which service destination path should be standardized? → A: `src/<ServiceName>/`.
- Q: Should template selection be optional in automation? → A: Require `--template` in non-interactive mode; prompt in interactive mode.
- Q: Which template eligibility rule should apply to `nfw add service`? → A: Only templates with `type=service` metadata.
- Q: Should the generated service template version be persisted? → A: Yes, persist template ID + resolved version in workspace metadata.
- Q: Where should per-service template provenance be stored? → A: Under `nfw.yaml`.

## Non-Goals

- Defining command/query/entity/CRUD generation behavior.
- Defining architecture validation command behavior (`nfw check`) beyond generated default compliance.
- Defining non-.NET service generation behavior in this spec.
- Introducing a second template registry or a service-only template lifecycle separate from existing template management.
