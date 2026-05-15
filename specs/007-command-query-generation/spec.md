# Feature Specification: Command and Query Generation

## User Scenarios & Testing

### User Story 1 - Generate a Command (Priority: P1)

As an application developer, I want to run `nfw add command ApproveOrder Orders` so that all necessary Command, Handler, and Registration code is boilerplate-generated and injected correctly according to my workspace's generator choice.

**Why this priority**: Core productivity feature for the framework.

**Independent Test**: Run `nfw add command` in a .NET workspace and verify it produces the expected files and injections defined in the mapped generator.

**Acceptance Scenarios**:

1. **Given** a workspace with `nfw.yaml` mapping `command` to `dotnet-mediator-v1`, **When** I run `nfw add command CreateProduct Products`, **Then** NFW resolves the `dotnet-mediator-v1` generator and executes its steps.
2. **Given** the command is run, **When** generation completes, **Then** variables like `{{Name}}` (CreateProduct), `{{Feature}}` (Products), and `{{Namespace}}` are correctly passed to the generator engine.

---

### User Story 2 - Map Commands to Generators (Priority: P1)

As a tech lead, I want to specify in `nfw.yaml` which generators should be used for commands and queries so that I can switch between different architectural variations (e.g., Simple vs. Standard vs. Advanced) globally for the workspace.

**Why this priority**: Enables the "Clean Architecture" enforcement and flexibility requested by the user.

**Independent Test**: Change the generator mapping in `nfw.yaml` and verify `nfw add command` uses the new generator immediately.

**Acceptance Scenarios**:

1. **Given** `nfw.yaml` has `generators: { command: "custom-generator" }`, **When** I run `nfw add command MyCmd MyFeature`, **Then** the CLI uses the `custom-generator` folder for execution.
2. **Given** `nfw.yaml` is missing a mapping, **When** I run the command, **Then** the CLI fails with an actionable message explaining how to configure the mapping in `nfw.yaml`.

---

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST implement `nfw add command <NAME> <FEATURE>` and `nfw add query <NAME> <FEATURE>`.
- **FR-002**: The CLI MUST read the `nfw.yaml` file from the workspace root to find generator mappings.
- **FR-003**: The `nfw.yaml` schema MUST support a `generators` section:

    ```yaml
    generators:
      command: <generator-id>
      query: <generator-id>
    ```

- **FR-004**: The CLI MUST provide the following standard inputs to the generator generation engine:
  - `Name`: The first argument (e.g., `ApproveOrder`).
  - `Feature`: The second argument (e.g., `Orders`).
  - `Namespace`: The base namespace of the workspace (determined from `nfw.yaml` or project folder).
  - `WorkspaceRoot`: The absolute path to the workspace root.
- **FR-005**: The CLI MUST resolve the `<generator-id>` using the existing generator discovery mechanism (searching `src/nfw-generators/` and cached remotes).
- **FR-006**: The CLI MUST support passing custom generator parameters via an optional `--param` flag (format: `Key=Value` or `Key1=Value1,Key2=Value2`).
- **FR-007**: The CLI MUST validate that `<NAME>` and `<FEATURE>` are valid identifiers (alphanumeric, hyphens, and underscores only).

### Key Entities

- **Generator Request**: The intent to generate a component, consisting of the type (command/query), name, feature, and additional options.
- **Generator Mapping**: The connection between a CLI command type and a specific generator defined in `nfw.yaml`.

## Success Criteria

- **SC-001**: Running `nfw add command` successfully triggers the execution engine with the correct generator and variables.
- **SC-002**: Missing configuration in `nfw.yaml` is handled with clear instructions for the user.
- **SC-003**: Generation is fast (sub-second for typical scenarios).

## Assumptions

- The project already has a mechanism to identify the "Workspace Root" (usually presence of `nfw.yaml`).
- Generator IDs are unique within the discovery path.
