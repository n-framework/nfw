# Feature Specification: Generate Repository Command

## User Scenarios & Testing

### User Story1 - Generate repository for default feature (Priority: P1)

As a developer using the nfw CLI, I want to run `nfw gen repository <ENTITY_NAME>` without additional flags to generate repository files for an existing entity in the default feature folder, so that I can quickly scaffold repository patterns via the CLI.

**Why this priority**: Core functionality of the command; delivers the primary user value of automating repository generation through the CLI.

**Independent Test**: Can be tested by running the command with a valid existing entity, verifying that the CLI reads template configuration and generates all specified files in the correct locations.

**Acceptance Scenarios**:

1. **Given** a valid nfw project with an existing `User` entity in the default feature folder and persistence configured in `nfw.yaml`, **When** I run `nfw gen repository User`, **Then** the CLI:
   - Reads the repository template configuration from `src/nfw-templates/`
   - Generates all files specified in the template configuration
   - Completes in <2 seconds
   - Exits with code 0
2. **Given** a valid nfw project with no existing `NonExistent` entity, **When** I run `nfw gen repository NonExistent`, **Then** the CLI:
   - Returns an error message to stderr: "Entity NonExistent not found"
   - Exits with non-zero status code
   - Does not generate any files
3. **Given** a valid nfw project without persistence configured in `nfw.yaml`, **When** I run `nfw gen repository User`, **Then** the CLI:
   - Returns an error message to stderr: "Persistence not configured. Run `nfw add persistence` to add persistence support"
   - Exits with non-zero status code
   - Does not generate any files

---

### User Story2 - Generate repository for specific feature (Priority: P2)

As a developer, I want to run `nfw gen repository <ENTITY_NAME> --feature <FEATURE_NAME>` to target a specific feature folder, so that the repository files are generated in the correct feature's directory structure.

**Why this priority**: Extends core functionality to multi-feature projects; high value for larger codebases.

**Independent Test**: Can be tested by running the command with `--feature` flag, verifying the CLI applies the template configuration for the specified feature.

**Acceptance Scenarios**:

1. **Given** a valid nfw project with a `payments` feature containing an `Order` entity and persistence configured, **When** I run `nfw gen repository Order --feature payments`, **Then** the CLI:
   - Reads the repository template configuration
   - Generates all repository files in the `payments` feature's directory structure as specified by the template
   - Completes in <2 seconds
2. **Given** a valid nfw project with no `invalid` feature folder, **When** I run `nfw gen repository User --feature invalid`, **Then** the CLI:
   - Returns an error to stderr: "Feature folder invalid not found"
   - Exits with non-zero status code

---

### User Story3 - Command performance and validation (Priority: P3)

As a developer, I want the `nfw gen repository` command to complete in under 2 seconds and validate all preconditions before generation, so that my development workflow remains fast and error-free.

**Why this priority**: Critical for user experience but depends on core functionality being implemented first.

**Independent Test**: Can be tested by timing command execution for valid inputs and verifying error cases return quickly.

**Acceptance Scenarios**:

1. **Given** a valid entity exists in the target feature and all preconditions are met, **When** I run `nfw gen repository User`, **Then** the CLI completes execution in <2 seconds.
2. **Given** the target entity exists but the `--feature` folder is invalid, **When** I run the command, **Then** the CLI returns an error in <1 second without attempting generation.

---

## Edge Cases

- **Invalid entity name**: When the entity name contains invalid characters, the CLI returns an error to stderr and exits with non-zero code.
- **Missing write permissions**: When the CLI lacks permissions to write to target directories, it returns a permission error to stderr.
- **Repository already exists**: When repository files already exist, the CLI returns an error and preserves existing files.
- **Concurrent execution**: When multiple `nfw gen repository` commands run simultaneously, the CLI completes within 2 seconds using atomic file operations.
- **No arguments**: When `nfw gen repository` is run without an entity name, the CLI displays usage information to stdout and exits with non-zero status code.
- **Interrupt signal (Ctrl+C)**: When the user sends Ctrl+C during execution, the CLI performs a clean shutdown and removes any partially generated files.
- **Persistence not configured**: When `nfw.yaml` does not contain a persistence configuration section, the CLI returns an error to stderr and exits.
- **Invalid nfw.yaml**: When `nfw.yaml` exists but is malformed, the CLI returns an error to stderr and exits.
- **Template configuration not found**: When the repository template configuration cannot be found in the template store, the CLI returns an error to stderr and exits.
- **core-persistence-dotnet not available**: When the module is not accessible, the CLI returns an error to stderr and exits.

## Requirements

### Functional Requirements

- **FR-001**: The CLI MUST accept the command `nfw gen repository <NAME>` where `<NAME>` is the name of an existing entity.
- **FR-002**: The CLI MUST accept an optional `--feature <FEATURE_NAME>` parameter to specify the target feature folder.
- **FR-003**: The CLI MUST read the repository template configuration from `src/nfw-templates/` to determine which files to generate and their target locations.
- **FR-004**: The CLI MUST generate all files specified in the repository template configuration with proper entity name substitution.
- **FR-005**: The CLI MUST validate that the specified entity exists in the target feature folder before reading template configuration; if not found, it MUST return an error to stderr and exit with non-zero code.
- **FR-006**: The CLI MUST complete execution in less than 2 seconds for all valid inputs.
- **FR-007**: The implementation MUST include integration tests covering all user scenarios, edge cases, and validation logic.
- **FR-008**: The CLI MUST verify that `nfw.yaml` contains a persistence configuration section; if not configured, it MUST return an error to stderr and exit with non-zero code.
- **FR-009**: The CLI MUST use the template configuration to determine file paths, class names, and other generation parameters (the template configuration defines these, not the CLI code).
- **FR-010**: All error messages MUST be written to stderr; normal output (success messages) MUST go to stdout, per Constitution Article II (CLI I/O).

### Key Entities

- **Entity**: Existing domain entity (e.g., `User`, `Order`) for which the repository is generated. The CLI validates its existence in the feature's `Domain/Entities/` folder.
- **Repository Template Configuration**: Configuration file in `src/nfw-templates` that defines which files to generate, their target paths, and template parameters. The CLI reads this configuration.
- **Feature Folder**: Target directory for the feature, either default or specified via `--feature` parameter.
- **nfw.yaml Configuration**: Project configuration file that MUST contain a persistence section for the command to proceed.
- **Template Store**: The `src/nfw-templates` directory containing template configurations and file templates that the CLI reads from.

## Success Criteria

### Measurable Outcomes

- **SC-001**: 100% of valid `nfw gen repository` commands with existing entities and persistence configured complete in <2 seconds.
- **SC-002**: 100% of commands targeting non-existent entities, invalid features, or missing persistence configuration return clear, actionable error messages to stderr within 1 second.
- **SC-003**: 100% of generated files match the specifications defined in the repository template configuration.
- **SC-004**: 100% of generated files are placed in the correct locations as defined by the template configuration.
- **SC-005**: 100% of integration tests pass, covering all user scenarios, edge cases, and validation rules (including `nfw.yaml` persistence check and template configuration reading).

## Assumptions

- nfw CLI handles file placement, DI registration, and template processing based on template configuration.
- Repository template configuration in `src/nfw-templates` defines all file generation details (paths, substitutions, etc.).
- Integration tests use nfw's built-in CLI test utilities to simulate command execution and verify file generation.
- `nfw.yaml` configuration file contains a `persistence` section (added by `nfw add persistence`).
- The template store (`src/nfw-templates`) is accessible and contains valid repository template configuration.

## Dependencies

- Existing nfw CLI infrastructure for command parsing (`clap`), template reading (`serde`/`serde_yaml`), and file generation.
- `src/nfw-templates` repository containing repository template configuration and file templates that the CLI reads from.
- `src/core-persistence-dotnet` module: Must be accessible for validation (template rendering will reference it).
- nfw project structure with proper feature folder organization.
- `nfw.yaml` configuration file with persistence section present (the CLI validates this).
- Template configuration defines all file generation steps, target paths, and substitutions that the CLI applies.

## Clarifications

- Q: What template configuration parameters are available for repository generation? → A: The template configuration in `src/nfw-templates` defines file templates, target paths (using placeholders like `{Feature}`, `{Entity}`), and substitution patterns. The CLI reads and applies this configuration.
- Q: How does the command validate persistence is configured? → A: The CLI reads `nfw.yaml` and checks for a `persistence` configuration section. If absent, it returns the error to stderr.
- Q: What does `nfw add persistence` do? → A: It adds the `core-persistence-dotnet` module to the project and updates `nfw.yaml` with the persistence configuration section.
- Q: Where is DI registration handled? → A: The template configuration specifies DI registration details. The CLI applies this configuration to inject the registration into the appropriate file.

## Non-Goals

- The CLI will not hardcode file paths, class names, or layer conventions; these come from template configuration.
- The CLI will not modify existing base repository interfaces or implementations in `core-persistence-dotnet`.
- The CLI will not modify existing entity files in Domain layer.
- The CLI will not support generating repositories for multiple entities in a single invocation.
- The CLI will not create the DbContext or persistence infrastructure (that's done by `nfw add persistence`).
- The CLI will not proceed if `nfw.yaml` does not contain a persistence configuration section.
- The CLI will not proceed if the repository template configuration cannot be found in the template store.
