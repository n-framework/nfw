# Feature Specification: Workspace Structure and `nfw new` Command

## User Scenarios & Testing

### User Story 1 - Create a New Workspace Baseline (Priority: P1)

As a platform engineer, I want one command to create a new workspace with a standard structure so that teams start from a consistent baseline without manual setup.

**Why this priority**: Workspace creation is the first user journey. If this fails or is inconsistent, every downstream command and convention becomes unreliable.

**Independent Test**: Run `nfw new <workspace-name> --no-input` and verify the generated workspace contains the documented folders and baseline configuration artifacts in the correct locations.

**Acceptance Scenarios**:

1. **Given** a user runs `nfw new BillingPlatform --no-input`, **When** generation completes, **Then** a workspace is created with `src/`, `tests/`, and `docs/` directories plus documented root-level configuration files.
2. **Given** the generated workspace, **When** the user inspects file layout and naming, **Then** project folders and namespace conventions match the specification.
3. **Given** generation finishes successfully, **When** the user runs the documented build and test commands, **Then** the workspace builds and tests without additional setup.

---

### User Story 2 - Select Generators in Interactive and Non-Interactive Modes (Priority: P1)

As a developer, I want generator selection to work with both prompts and flags so that I can use the command locally and in automation.

**Why this priority**: Generator selection is part of the core `nfw new` flow and directly affects usability in terminals and CI.

**Independent Test**: Verify `nfw new` prompts for missing input in an interactive terminal, and verify `nfw new <name> --generator <id> --no-input` runs without prompts.

**Acceptance Scenarios**:

1. **Given** an interactive terminal and missing required input, **When** the user runs `nfw new`, **Then** the command prompts for required values before generation.
2. **Given** a non-interactive environment, **When** the user runs `nfw new MyWorkspace --generator official/blank-workspace --no-input`, **Then** generation runs without any prompt.
3. **Given** `--no-input` and missing required values, **When** the command starts, **Then** it fails fast with an actionable error explaining which inputs are missing.

---

### User Story 3 - Route CLI Commands Predictably (Priority: P2)

As a CLI user, I want consistent command parsing and routing so that command behavior is predictable and errors are easy to correct.

**Why this priority**: Parsing and routing consistency reduces user confusion and supports future command growth.

**Independent Test**: Execute valid and invalid `nfw` command shapes and verify commands are routed correctly and errors provide remediation guidance.

**Acceptance Scenarios**:

1. **Given** a valid `nfw new` command shape, **When** the command is invoked, **Then** it routes to workspace creation logic with parsed options.
2. **Given** unknown flags or invalid option combinations, **When** the command is invoked, **Then** it exits non-zero with clear usage guidance.
3. **Given** an unknown subcommand, **When** the user runs it, **Then** the CLI returns a clear error and shows available commands.

## Edge Cases

- **Existing target directory**: If the destination path already exists and is not empty, the command must fail with remediation guidance.
- **Invalid workspace name**: If the workspace name violates naming rules, the command must reject it with the expected naming format.
- **Generator not found**: If `--generator <id>` does not match any available generator, the command must fail with available generator hints.
- **Conflicting flags**: If mutually incompatible options are passed, the command must fail with explicit conflict details.
- **Non-interactive missing inputs**: In non-interactive mode, any missing required value must cause immediate failure without prompting.
- **Interrupted execution**: If the user interrupts generation (Ctrl+C), partially created artifacts must not be left in an ambiguous state.
- **Configuration file errors**: If required configuration files are malformed or unreadable, the command must report file path and corrective action.

## Requirements

### Functional Requirements

- **FR-001**: The system MUST define a canonical workspace folder structure for newly created workspaces using a layered root layout with `src/`, `tests/`, and `docs/` directories plus root-level baseline configuration files.
- **FR-002**: The system MUST define namespace conventions using a workspace-root base namespace with explicit service and layer suffixes.
- **FR-003**: The system MUST define generator-driven artifact organization, where generated files come from the selected generator content and placeholder rendering rules.
- **FR-004**: The system MUST define baseline configuration file locations and names, and MUST use YAML as the only canonical configuration format for generated workspace baseline configuration files.
- **FR-005**: The CLI MUST parse commands and options using a deterministic routing model where each valid command shape maps to exactly one command handler.
- **FR-006**: The CLI MUST reject unknown commands, unknown flags, and invalid argument combinations with actionable error messages.
- **FR-007**: The CLI MUST provide help text that documents command syntax, required arguments, optional flags, and examples for `nfw new`.
- **FR-008**: `nfw new [workspace-name]` MUST create a workspace root with the documented folder and file structure.
- **FR-009**: `nfw new` MUST support interactive prompting for missing required inputs when running in an interactive terminal.
- **FR-010**: `nfw new` MUST support non-interactive execution using explicit arguments and flags.
- **FR-011**: `nfw new --generator <id>` MUST select the specified generator when the generator exists and is available.
- **FR-012**: `nfw new --no-input` MUST disable all interactive prompts.
- **FR-013**: When `--no-input` is set and required values are missing, the command MUST fail before generation starts and identify the missing values.
- **FR-014**: The system MUST apply documented defaults for optional values when the user does not provide them.
- **FR-015**: The command MUST validate workspace names and generator identifiers before file generation.
- **FR-016**: The command MUST return deterministic exit outcomes for success, validation failure, and runtime failure.
- **FR-017**: Generated workspaces MUST include baseline documentation indicating how to build and test the workspace with one command each.
- **FR-018**: The system MUST ensure generated output is reproducible for the same input values and generator version.
- **FR-019**: If the target directory already exists and is non-empty, `nfw new` MUST fail immediately before generation starts.

### Key Entities

- **Workspace Blueprint**: The canonical definition of required folders and baseline configuration artifacts for a generated workspace.
- **Workspace Naming Rule Set**: The constraints and derivation rules for workspace names and namespaces.
  The namespace model uses a workspace-root base namespace and appends service and layer suffixes consistently.
- **Command Route Definition**: The normalized mapping from parsed CLI input to a specific command behavior and validation path.
- **Workspace Initialization Request**: The resolved set of user-provided and default values used to generate a workspace.
- **Generator Selection Input**: The generator identifier and selection mode used during workspace creation.

## Success Criteria

### Measurable Outcomes

- **SC-001**: In acceptance testing, 100% of generated workspaces match the documented required folder and file structure.
- **SC-002**: In acceptance testing, 100% of non-interactive `nfw new` runs with complete valid inputs finish without prompts.
- **SC-003**: In acceptance testing, 100% of invalid command shapes fail with a non-zero exit and actionable remediation text.
- **SC-004**: A new user following the generated quickstart can create, build, and test a workspace on first attempt in at least 90% of test runs.
- **SC-005**: Two workspace generations using the same generator version and identical inputs produce identical file paths and file contents.

## Assumptions

- Generator discovery and metadata validation are provided by the existing generator system specification in `001-nfw-generator-system`.
- The initial focus is the .NET-first workspace baseline already defined by product-level requirements.
- A default official generator is available so `nfw new` can run without requiring manual source registration in standard setups.
- The CLI environment can detect whether input is interactive or non-interactive.

## Dependencies

- `docs/PRD.md` US-001 and FR-1 through FR-7 for CLI and workspace creation scope.
- `docs/SPECIFICATION_GUIDELINES.md` for required spec structure and quality rules.
- `src/nfw/specs/001-nfw-generator-system/spec.md` for generator IDs, source handling, and version selection behavior.

## Clarifications

- Q: Should `nfw new` support interactive prompting? → A: Yes, in interactive terminals it prompts for missing required values.
- Q: Should automation be supported without prompts? → A: Yes, `--no-input` enforces non-interactive behavior.
- Q: Can users choose a generator explicitly during workspace creation? → A: Yes, `--generator <id>` selects the generator for generation.

### Session 2026-04-02

- Q: Which workspace root layout should be standardized? → A: Layered root with `src/`, `tests/`, and `docs/`, plus root-level configuration files.
- Q: What namespace convention base should be used? → A: Workspace-root base namespace with service/layer suffixes.
- Q: What baseline configuration format should be canonical? → A: YAML only.
- Q: How should generated artifacts be organized? → A: Generator-driven; artifact files are copied from selected generator content after placeholder rendering.
- Q: What is the policy for existing non-empty target directories? → A: Fail immediately.

## Non-Goals

- Defining entity, command, query, or CRUD generation workflows.
- Defining distributed runtime orchestration commands beyond workspace initialization.
- Defining cross-language scaffold parity details for Go and Rust in this spec.
- Replacing generator discovery, source registration, or generator versioning behavior already covered by the generator system spec.
