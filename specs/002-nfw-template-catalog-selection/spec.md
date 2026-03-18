# Feature Specification: Template Catalog Listing and Selection

**Feature Branch**: `002-nfw-template-catalog-selection`
**Spec Type**: Project-Based
**Project**: `nfw`
**Created**: 2026-03-17
**Status**: Draft
**Input**: User description: "Define template catalog listing and deterministic template selection for nfw, including stable template identifiers, interactive selection when a terminal is interactive, and explicit template selection for non-interactive usage."

> **Note**: This spec is organized as project-based. See `.specify/SPEC_ORGANIZATION.md` for details on spec organization types.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Review Available Templates (Priority: P1)

A developer wants to inspect the available starter templates before creating a workspace so they can choose the right starting point without reading source files or external documentation.

**Why this priority**: Users cannot make a correct template choice unless the catalog is visible, stable, and understandable.

**Independent Test**: Can be fully tested by running the template listing command and confirming the output shows every available template with a stable identifier and clear descriptive information in a repeatable order.

**Acceptance Scenarios**:

1. **Given** the official template catalog contains multiple templates, **When** the user runs the template listing command, **Then** the system shows every available template with its stable identifier, display name, and short description.
2. **Given** the template catalog has not changed, **When** the user runs the template listing command multiple times, **Then** the templates appear in the same order each time.
3. **Given** the catalog includes templates intended for different use cases, **When** the user reviews the list, **Then** the information shown is sufficient to distinguish one template from another without opening the template contents.

---

### User Story 2 - Choose a Template Interactively (Priority: P2)

A developer working in an interactive terminal wants the workspace creation flow to guide template selection when they did not provide a template identifier up front.

**Why this priority**: Interactive users should not need to memorize identifiers, and they should be able to make an informed choice at the point of creation.

**Independent Test**: Can be fully tested by starting workspace creation in an interactive terminal without a template identifier or workspace name and confirming the system prompts for the missing input before generation starts.

**Acceptance Scenarios**:

1. **Given** the user starts workspace creation in an interactive terminal without providing a workspace name, **When** the command begins, **Then** it prompts for the missing workspace name before any files are created.
2. **Given** the user starts workspace creation in an interactive terminal without providing a template identifier, **When** the system detects that multiple templates are available, **Then** it prompts the user to choose one from the catalog before creating any files.
3. **Given** the system is prompting for required interactive input, **When** the user supplies the missing values and chooses a template, **Then** the workspace is created from the chosen template and the chosen identifier is reflected in user-visible confirmation output.
4. **Given** the system is prompting for required interactive input, **When** the user cancels the prompt, **Then** the command exits cleanly without creating a partial workspace.

---

### User Story 3 - Use Templates Reliably in Automation (Priority: P3)

A platform engineer wants non-interactive usage to be explicit and deterministic so CI jobs and scripted workflows always create the intended workspace template.

**Why this priority**: Automation depends on predictable behavior and must not rely on prompts or hidden defaults.

**Independent Test**: Can be fully tested by running workspace creation in a non-interactive context both with and without an explicit template identifier and verifying the success and failure behavior.

**Acceptance Scenarios**:

1. **Given** workspace creation runs in a non-interactive context and the user provides a valid template identifier, **When** the command starts, **Then** it uses that exact template without prompting.
2. **Given** workspace creation runs in a non-interactive context and no template identifier is provided, **When** the command starts, **Then** it fails before creating any files and tells the user how to specify a template explicitly.
3. **Given** the user disables interactive prompts explicitly, **When** any required `new` input is omitted, **Then** the command fails before creating any files and tells the user which value must be provided explicitly.
4. **Given** the user provides an unknown template identifier, **When** the command validates the request, **Then** it fails with an actionable error that identifies the invalid identifier and points the user to the list of valid choices.

### Edge Cases

- The catalog contains no available templates when the user requests a listing or attempts to create a workspace.
- Two catalog entries attempt to use the same identifier.
- A template identifier was valid when listed but is no longer available when creation begins.
- The user enters or passes a template identifier using different letter casing than the catalog display.
- Interactive selection is cancelled after the catalog has been loaded but before workspace generation begins.
- Interactive input is disabled explicitly even though the command is running in a real terminal.

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The system MUST assign every available template a stable identifier that is unique within the catalog and intended for user-facing selection.
- **FR-002**: The system MUST expose the stable identifier consistently anywhere a template can be listed, selected, documented, or referenced in error output.
- **FR-003**: The template listing command MUST show each available template with, at minimum, its stable identifier, display name, and short description.
- **FR-004**: The template listing command MUST present templates in a deterministic order so repeated runs against the same catalog produce the same visible sequence.
- **FR-005**: Workspace creation MUST accept an explicit template identifier so users can select a template directly without interactive input.
- **FR-006**: When workspace creation runs in an interactive terminal without a required workspace name, the system MUST prompt the user for that missing value before generation begins.
- **FR-007**: When workspace creation runs in an interactive terminal without an explicit template identifier, the system MUST prompt the user to choose from the available templates before generation begins.
- **FR-008**: The interactive selection experience MUST display the same stable identifiers and descriptive information exposed by the template listing command.
- **FR-009**: The interactive selection flow MUST require a concrete user choice or an explicit cancellation; it MUST NOT silently choose a template on the user's behalf.
- **FR-010**: When workspace creation runs in a non-interactive context without an explicit template identifier, the system MUST stop before creating files and return an actionable error that explains how to provide the identifier.
- **FR-011**: When a user provides an unknown, invalid, or unavailable template identifier, the system MUST reject the request before generation starts and identify how the user can discover valid identifiers.
- **FR-012**: The system MUST guarantee deterministic template selection so the same explicit identifier always resolves to the same catalog entry within the same catalog version.
- **FR-013**: The system MUST prevent ambiguous selection by rejecting catalogs that contain duplicate identifiers.
- **FR-014**: If the user cancels interactive selection, the system MUST exit without leaving partially generated workspace files behind.
- **FR-015**: User-facing help and error guidance for template-driven workspace creation MUST explain the difference between interactive selection and explicit non-interactive usage.
- **FR-016**: Delivery documentation for the `templates` and `new` CLI surfaces MUST include executable build, test, and verification commands that can be run from the repository root.
- **FR-017**: The system MUST resolve template identifiers in a documented, deterministic way when user input differs only by letter casing, and it MUST surface the canonical catalog identifier in confirmation and error output.
- **FR-018**: When workspace creation cannot find any selectable templates, the system MUST fail before file generation begins with an actionable error rather than prompting indefinitely or choosing a hidden default.
- **FR-019**: Workspace creation MUST provide an explicit option to disable interactive prompts, and when that option is used the system MUST require all remaining required inputs to be provided explicitly.

### Key Entities _(include if feature involves data)_

- **Template Catalog**: The user-visible collection of available workspace templates, including the set of selectable entries and their presentation order.
- **Template Entry**: A single catalog item with a stable identifier, display name, short description, and any additional user-facing classification needed to distinguish it from other templates.
- **Template Selection Request**: The user's intent to create a workspace from a specific template, either by explicit identifier or by making a choice during an interactive prompt.

## Assumptions

- This feature applies to template-driven workspace creation flows and does not expand scope to service, entity, command, or query generation.
- The initial catalog source remains the official catalog already used by `nfw`; custom organizational catalogs are outside this feature's scope.
- Stable identifiers are treated as part of the public CLI contract and should remain valid across catalog updates unless intentionally removed or deprecated.
- Non-interactive usage includes CI pipelines, scripts, redirected input/output, and any other environment where prompting is not appropriate.
- Repository-root developer entrypoints may require small supporting updates outside `src/nfw` so the delivered CLI surface remains compliant with the project constitution.

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: In acceptance testing, 100% of available templates can be identified from the listing output using a unique stable identifier and a human-readable description.
- **SC-002**: In repeated acceptance tests against an unchanged catalog, the visible order of listed templates is identical across all runs.
- **SC-003**: In interactive usability testing, users can select the intended template and begin workspace creation without external documentation in under 30 seconds.
- **SC-004**: In non-interactive acceptance tests, 100% of runs without an explicit template identifier fail before file generation begins and provide a corrective next step.
- **SC-005**: In non-interactive acceptance tests, 100% of runs with a valid explicit template identifier create a workspace from the requested template without prompting.
