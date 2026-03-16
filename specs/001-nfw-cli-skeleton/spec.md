# Feature Specification: nfw CLI Skeleton

**Feature Branch**: `001-nfw-cli-skeleton`
**Spec Type**: Project-Based
**Project**: nfw
**Created**: 2026-03-14
**Status**: Draft
**Input**: User description: "Phase 1, First Deliverable: Initial nfw CLI Skeleton - This is the foundational entry point. Everything else depends on having a working CLI."

> **Note**: This spec is organized as project-based. See `.specify/SPEC_ORGANIZATION.md` for details on spec organization types.

## User Scenarios & Testing _(mandatory)_

### User Story 1 - Discover CLI Capabilities (Priority: P1)

As a developer using the nfw CLI for the first time, I want to see comprehensive help information so I can understand what commands are available and how to use them.

**Why this priority**: This is the most critical user journey - without help information, users cannot discover or learn to use any other functionality. A CLI without help is essentially unusable for new users.

**Independent Test**: Can be fully tested by running `nfw --help` and verifying the help text displays with all available commands, usage examples, and command descriptions. Delivers immediate value by enabling user onboarding.

**Acceptance Scenarios**:

1. **Given** the nfw CLI is installed, **When** I run `nfw --help` or `nfw -h`, **Then** I see a help screen displaying available commands and usage information
2. **Given** the nfw CLI is installed, **When** I run `nfw --help`, **Then** the help text includes command names, brief descriptions, and usage syntax
3. **Given** the nfw CLI is installed, **When** I run `nfw --help`, **Then** the help output is formatted and readable on standard terminal widths (80 columns)
4. **Given** the nfw CLI is installed, **When** I run `nfw --help`, **Then** the help text indicates how to get more detailed help for specific commands

---

### User Story 2 - Verify CLI Version (Priority: P2)

As a developer troubleshooting issues or reporting bugs, I want to check which version of the nfw CLI I have installed so I can confirm I'm using the expected version and include version information in bug reports.

**Why this priority**: Version information is essential for debugging and support workflows. While less critical than help, it's a standard CLI expectation and enables proper issue reporting.

**Independent Test**: Can be fully tested by running `nfw --version` and verifying version information is displayed. Delivers value by enabling version verification.

**Acceptance Scenarios**:

1. **Given** the nfw CLI is installed, **When** I run `nfw --version`, **Then** I see the current version number displayed
2. **Given** the nfw CLI is installed, **When** I run `nfw -v`, **Then** I see the current version number displayed
3. **Given** the nfw CLI is installed, **When** I run `nfw --version`, **Then** the version follows semantic versioning format (e.g., "0.1.0")
4. **Given** the nfw CLI is installed, **When** I run `nfw --version`, **Then** additional build information (such as commit hash or build date) may be included if available

---

### User Story 3 - List Available Templates (Priority: P3)

As a developer starting a new project, I want to see what templates are available so I can choose an appropriate starting point for my workspace or service.

**Why this priority**: While templates are a key feature, the skeleton phase only requires the command to exist and function. It can initially return an empty list, making this lower priority than help and version.

**Independent Test**: Can be fully tested by running `nfw templates` and verifying the command executes and displays available templates (which may be empty initially). Delivers value by establishing the template listing pattern.

**Acceptance Scenarios**:

1. **Given** the nfw CLI is installed, **When** I run `nfw templates`, **Then** I see a list of available template names
2. **Given** the nfw CLI is installed, **When** I run `nfw templates` and no templates are installed, **Then** I see a message indicating no templates are available (not an error)
3. **Given** the nfw CLI is installed, **When** I run `nfw templates`, **Then** each template is displayed with its name and a brief description
4. **Given** the nfw CLI is installed (Release build), **When** I run `nfw templates` and the remote template catalog is unreachable, **Then** I see a clear error message and the command exits with status code 1

---

### Edge Cases

- **No arguments**: When `nfw` is run with no arguments, display help information
- **Invalid command**: When an unknown command is entered (e.g., `nfw hekp`), display an error message suggesting similar valid commands or showing help, and exit with status code 2
- **Conflicting flags**: When both `--help` and `--version` are provided together, `--help` takes precedence
- **No write permissions**: When run from a directory without write permissions, operations that require writing fail with a clear error message indicating the permission issue
- **Corrupted configuration**: When the configuration file is corrupted or malformed, display an error message indicating the configuration file cannot be parsed and continue with default configuration
- **Unknown flags**: When a command is invoked with unknown flags, display an error message listing valid flags for that command
- **Interrupt signal**: When the user sends an interrupt signal (Ctrl+C), the CLI exits cleanly with status code 130 (standard Unix convention)
- **Missing configuration**: When required configuration values are missing, display an error message indicating which values are missing and how to provide them

## Requirements _(mandatory)_

### Functional Requirements

- **FR-001**: The CLI MUST parse command-line arguments for commands, flags, and options
- **FR-002**: The CLI MUST route recognized commands to their corresponding command handlers and report an error for unrecognized commands
- **FR-003**: The CLI MUST display help information when `--help` or `-h` flag is provided
- **FR-004**: The CLI MUST display version information when `--version` or `-v` flag is provided
- **FR-005**: The CLI MUST support a `templates` command that lists available templates (Debug build: local `src/nfw/packages/n-framework-nfw-templates` submodule if present; Release build: remote `n-framework/nfw-templates` release tag `v{cliVersion}`)
- **FR-006**: The CLI MUST provide clear error messages for invalid commands or arguments, including guidance on what went wrong and how to fix it
- **FR-007**: The CLI MUST support loading configuration from a configuration file located in the current working directory as the base configuration
- **FR-008**: The CLI MUST support configuration via environment variables, which override file-based settings for the same configuration key
- **FR-009**: The CLI MUST exit with status code 0 for successful operations and non-zero status codes for errors (1 for general errors; 2 for usage errors such as unknown commands, invalid arguments, or unknown flags)
- **FR-010**: The CLI MUST handle interrupt signals (SIGINT) by exiting cleanly with status code 130
- **FR-011**: The CLI MUST validate that required configuration values are present at startup and display an error message listing any missing required values (note: skeleton phase has no required configuration values; this requirement is for future extensibility)
- **FR-012**: The CLI MUST provide a default help display when invoked without arguments
- **FR-013**: The CLI MUST support subcommand help (e.g., `nfw templates --help`)
- **FR-014**: When configuration file parsing fails, the CLI MUST display an error message and continue with default configuration
- **FR-015**: When both `--help` and `--version` flags are provided, `--help` MUST take precedence
- **FR-016**: The CLI MUST support a `--verbose` flag that enables diagnostic logging output to stderr (no short flag)

### Key Entities

- **CLI Command**: A discrete action the user can invoke (e.g., `templates`, `new`, `add`)
- **Command Handler**: The logic that executes when a command is invoked
- **Configuration**: Key-value pairs that control CLI behavior (source: config file or environment)
- **Template**: A project scaffold that users can generate (initially empty list)
- **Version**: Semantic version identifier for the CLI build

## Success Criteria _(mandatory)_

### Measurable Outcomes

- **SC-001**: Developers can view help information within 100ms of invoking the command
- **SC-002**: 100% of CLI commands provide help text accessible via `--help` flag
- **SC-003**: New users can understand available CLI capabilities within 30 seconds of first invocation
- **SC-004**: The CLI executable starts up and displays output in under 200ms on typical development hardware
- **SC-005**: All error messages provide actionable guidance (what went wrong and how to fix it)
- **SC-006**: The CLI returns appropriate exit codes that can be relied upon in shell scripts (0 for success, non-zero for errors)

## Assumptions

- The CLI will be distributed as a single binary executable
- Users run the CLI from a terminal or command prompt with standard output
- Configuration file format is YAML, loaded from `nfw.yaml` in the current working directory
- The CLI runs on major operating systems (Linux, macOS, Windows)
- Users have basic familiarity with command-line interfaces
- Terminal width of at least 80 columns can be assumed for help text formatting
- Environment variables follow the naming convention `NFW_<SETTING_NAME>`
- Status code 130 is the appropriate exit code for SIGINT on Unix-like systems
- Templates are sourced from an official catalog (not user-configurable)
- In debugging environments, templates are read from the `src/nfw/packages/n-framework-nfw-templates` git submodule
- In production (Release builds), templates are read from the `n-framework/nfw-templates` GitHub release tag `v{cliVersion}`

## Dependencies

- `src/nfw/packages/n-framework-nfw-templates` git submodule (debugging environments)
- Remote template catalog releases (production)

## Clarifications

### Session 2026-03-14

- Q: When both config file and environment variables provide values for the same key, which takes precedence? → A: Environment variables override file settings
- Q: Where is the configuration file located and how is it discovered? → A: Current working directory only (project-scoped)
- Q: What config file format and name does the CLI load from the current working directory? → A: YAML (`nfw.yaml`)
- Q: What exit code should be used for unknown commands or invalid arguments? → A: Exit 2 (usage error)
- Q: Which short flags should be reserved for version and verbose? → A: Version uses `-v`; verbose has no short flag (use `--verbose`)
- Q: Which configuration values are required in the skeleton phase? → A: None (no required configuration for help, version, templates listing)
- Q: Should the CLI have any logging/observability capabilities? → A: Verbose flag for stderr logging (opt-in debugging)

### Session 2026-03-15

- Q: Where should templates be sourced from in debugging vs production environments? → A: Debug uses `src/nfw/packages/n-framework-nfw-templates` submodule; production uses `n-framework/nfw-templates` tag matching CLI version

## Non-Goals

- GUI or web-based interfaces (CLI only)
- Interactive shells or REPL modes
- Remote command execution or API endpoints
- Plugin system or extensibility beyond initial command set
- Advanced templating features (template creation comes in later phases)
