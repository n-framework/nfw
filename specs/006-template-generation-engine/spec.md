# Feature Specification: Template Generation Engine (Generic Execution)

## User Scenarios & Testing

### User Story 1 - Multi-step Template Execution (Priority: P1)

As a template author, I want to define a series of generation steps (rendering files, injecting code) in a configuration file so that the CLI can execute complex scaffolding logic without needing hardcoded workflows.

**Why this priority**: This is the core requirement to move away from hardcoded CLI logic. It enables the flexibility requested by the user.

**Independent Test**: Create a template with a `template.yaml` containing multiple `render` steps and verify `nfw` executes all of them accurately.

**Acceptance Scenarios**:

1. **Given** a template with a `template.yaml` defining three `render` steps, **When** I run the corresponding generation command, **Then** all three files are created at their target locations with the correct content.
2. **Given** a step that uses a variable (e.g., `{{Name}}`), **When** the CLI executes the step, **Then** the variable is correctly interpolated from the CLI inputs.

---

### User Story 2 - Automated Code Injection (Priority: P1)

As a developer, I want the CLI to be able to inject code into existing files (e.g., registering a service in a `DependencyInjection.cs` file) so that I don't have to manually update boilerplate after generation.

**Why this priority**: Required for "smooth" developer experience where generation fully "wires up" the new component.

**Independent Test**: Use a template with an `inject` step targeting a specific class and verify the new code appears inside the class body.

**Acceptance Scenarios**:

1. **Given** a target file with a class `public class Startup`, **When** I run a generation step with `action: inject` and `target: class(Startup)`, **Then** the template content is appended to the class body.
2. **Given** a target file with a region `#region Registrations`, **When** I run a step with `target: region(Registrations)`, **Then** the content is injected within that region.

---

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST look for a `template.yaml` file inside the resolved template directory to determine execution steps.
- **FR-002**: The `template.yaml` MUST support a `steps` list.
- **FR-003**: Each step MUST support an `action` field. Supported actions are `render`, `render_folder`, and `inject`.
- **FR-004**: The `render` action MUST take a `source` (single template file) and a `destination` (output path).
- **FR-004b**: The `render_folder` action MUST recursively render all files within a `source` directory to a `destination` directory, preserving the relative path structure.
- **FR-004c**: When using `render_folder`, the `.tera` extension SHOULD be stripped from the generated filenames.
- **FR-005**: The `inject` action MUST take a `source`, a `destination`, and an `injection_target`.
- **FR-006**: The `injection_target` MUST support several modes:
    - `at_end`: Append to the end of the file.
    - `class(Name)`: Inside the body of the specified class.
    - `function(Name)`: Inside the body of the specified function.
    - `region(Name)`: Inside the specified `#region` or equivalent marker.
- **FR-007**: The CLI MUST use the **Tera** template engine for both path interpolation and file content rendering.
- **FR-008**: The CLI MUST pass all standard inputs (e.g., `Name`, `Feature`, `Namespace`) to the Tera engine.
- **FR-009**: If a step fails (e.g., target file missing for injection), the CLI MUST abort and provide a clear error message. Partial file creations SHOULD be cleaned up if possible.

### Key Entities

- **Template Step**: A single unit of work (render or inject) defined in the template configuration.
- **Injection Target**: A specific location within a destination file where code should be merged.
- **Tera Context**: The set of variables provided by the CLI and workspace environment used during rendering.

## Success Criteria

- **SC-001**: Template execution is purely data-driven; no new Rust code is required in `nfw` to support a new template structure.
- **SC-002**: Injection logic handles edge cases like missing classes or regions by failing gracefully with descriptive errors.
- **SC-003**: Path interpolation (e.g. `src/{{Feature}}/{{Name}}.cs`) works correctly for all steps.

## Non-Goals

- Supporting complex conditional logic *inside* `template.yaml` (should be handled within Tera templates where possible).
- Real-time "undo" of complex multi-file injections (standard Git workflow is expected).
