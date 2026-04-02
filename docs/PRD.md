# Product Requirements Document (PRD): nfw CLI

## 1. Introduction / Overview

The `nfw` CLI is the primary developer-facing tool for creating, managing, and validating NFramework workspaces and services. It serves as the entry point for all framework lifecycle operations, providing template-based project generation, code scaffolding, architecture validation, and local orchestration capabilities.

The CLI exists to eliminate manual project setup, enforce architectural standards through automation, and provide a consistent developer experience across the polyglot NFramework ecosystem. It transforms complex multi-project scaffolding into single-command operations while maintaining the strict layer boundaries and clean architecture principles that define NFramework services.

The CLI is designed for fast startup and minimal runtime overhead.

## 2. Problem Statement

Teams building modern microservices repeatedly face the same issues:

- Frameworks depend heavily on runtime mechanisms that increase startup time and reduce portability.
- Clean Architecture rules are documented but not enforced, so boundaries decay over time.
- New services require large amounts of manual scaffolding and duplicate boilerplate.
- Infrastructure choices leak into application and domain layers, making services harder to test and replace.
- Polyglot teams struggle to keep the same architecture, naming, and contracts across languages.
- Cloud-native building blocks such as telemetry, messaging, state management, and local orchestration are often bolted on late instead of built in.

The `nfw` CLI must solve these problems by providing fast, opinionated commands that generate correct-by-default code, validate architecture automatically, and streamline the entire development lifecycle.

## 3. Goals

- Provide a single CLI-driven workflow for creating and evolving NFramework workspaces and services.
- Enforce Clean Architecture boundaries through project structure, package boundaries, and automated checks.
- Reduce time-to-first-service and time-to-first-CRUD endpoint to seconds, not hours.
- Generate production-ready code that compiles without manual modifications.
- Maintain consistent workspace structure across supported languages.
- Support both interactive and non-interactive (CI-friendly) workflows.
- Provide clear, actionable error messages and validation feedback.
- Keep domain and application code free from infrastructure dependencies.

## 4. Target Users

- **Enterprise Architects**: Need enforceable architectural standards across multiple teams.
- **Tech Leads**: Want repeatable service templates and fewer framework decisions per project.
- **Application Developers**: Quickly scaffold services, entities, commands, and queries.
- **Platform Engineers**: Need predictable scaffolding, local orchestration, and cloud-ready service defaults.
- **DevOps Engineers**: Integrate CLI commands into CI/CD pipelines.
- **New Team Members**: Onboard rapidly through standardized project generation.

## 5. Product Principles

1. **Instant Feedback**: Commands should complete in under 1 second for workspace creation and under 10 seconds for full CRUD generation.
2. **Correct by Default**: Generated code must compile and pass basic validation without manual edits.
3. **Actionable Errors**: Every failure must state what went wrong, why, and exactly how to fix it.
4. **Idempotent Where Possible**: Rerunning commands should be safe and predictable.
5. **CI-First Design**: All commands must work without interactive input for automation.
6. **Opinionated Standards**: The CLI enforces NFramework conventions while allowing template-based customization.
7. **Deterministic Generation**: Framework behavior such as registration and routing should be generated deterministically, not discovered at runtime.

## 6. Scope

### In Scope for Initial Beta

#### Core Commands

- `nfw templates` — List available starter templates.
- `nfw new [workspace-name] [--template <id>] [--no-input]` — Create new workspace.
- `nfw add service <name> --lang <go|rust>` — Add service scaffold.
- `nfw add entity <name> --props <properties>` — Generate entity class only.
- `nfw add crud <entity-name>` — Generate full CRUD scaffolding (DTOs, commands, queries, handlers, repository contracts, endpoints) for an existing entity.
- `nfw add command <name> <feature>` — Generate standalone command handler.
- `nfw add query <name> <feature>` — Generate standalone query handler.
- `nfw check` — Validate architecture boundaries.

#### Language Support

- Full service generation for supported languages.
- Folder structure and conventions for Go and Rust.

#### Validation

- Project reference validation across layers.
- Namespace/package usage validation.
- Dependency rule enforcement.
- CI exit codes for automated checks.

### Planned Post-Beta Scope

- `nfw up` — Local orchestration.
- `nfw sync` — Protobuf contract synchronization across languages.
- Remote template catalogs.
- Workspace update and migration commands.
- Advanced configuration and customization commands.

### Out of Scope

- Graphical user interface for project generation.
- Runtime service management beyond local development orchestration.
- Custom language support outside the supported language set.
- Template editor or builder CLI.

## 7. User Stories

### US-001: Create a new workspace

**Description:** As a platform engineer, I want to create a new NFramework workspace with one command so that I can start a service ecosystem without manual setup.

**Acceptance Criteria:**

- `nfw templates` lists available starter templates with identifiers and descriptions.
- `nfw new [workspace-name]` creates a workspace root with the expected folders, solution files, and baseline configuration.
- In an interactive terminal, `nfw new` prompts for any missing required input before generation starts.
- `nfw new [workspace-name] --template <id> --no-input` selects a starter template without requiring interactive input.
- The generated workspace can be built with one documented command.
- The generated workspace test suite can be run with one documented command.

### US-002: Add a service scaffold

**Description:** As a developer, I want to generate a service scaffold so that I can start from a standard Clean Architecture baseline instead of creating projects by hand.

**Acceptance Criteria:**

- `nfw add service <name> --lang <language>` creates the standard layer structure for the target language.
- Generated projects reference only allowed dependencies for their layer.
- The service compiles immediately after generation without manual file edits.
- The scaffold includes sample health or readiness endpoints.

### US-003: Add an entity and CRUD flow

**Description:** As an application developer, I want to generate an entity and its CRUD flow so that I can deliver standard features without repeating boilerplate.

**Acceptance Criteria:**

- `nfw add entity Product --props Name:string,Price:decimal` generates the entity class only.
- `nfw add crud Product` generates DTOs, commands, queries, handlers, repository contracts, and HTTP endpoints for an existing entity.
- Generated files are placed in the expected layer for each concern.
- Generated code builds without manual edits in a sample service.
- CLI validation rejects invalid property syntax with an actionable error message.

### US-004: Check architecture boundaries

**Description:** As a tech lead, I want an automated architecture audit so that boundary violations are detected before they reach production.

**Acceptance Criteria:**

- `nfw check` scans a workspace for forbidden project references and forbidden namespace or package usage.
- The command exits with a non-zero status when a violation is found.
- The error output identifies the violating project, file, or dependency with a concrete remediation hint.
- The command can be executed in CI without requiring interactive input.

### US-005: Generate standalone commands and queries

**Description:** As an application developer, I want to generate standalone commands and queries so that I can add feature behavior without creating full CRUD scaffolding.

**Acceptance Criteria:**

- `nfw add command ApproveOrder Orders` generates the command, response, handler, and supporting route wiring when API exposure is enabled.
- `nfw add query GetOrderByNumber Orders` generates the query, response, handler, and supporting route wiring when API exposure is enabled.
- The generator can create a missing feature folder when the requested feature does not already exist.
- Command and query generation supports optional caching, logging, transaction, secured-operation, and API exposure settings.

## 8. Functional Requirements

### FR-1: Core CLI Infrastructure

- The system must provide an `nfw` executable available through standard package managers.
- The CLI must support subcommands with consistent help text and examples.
- The CLI must support both interactive prompts and flag-based non-interactive mode.
- The CLI must provide version information.
- The CLI must support configuration files for user preferences and workspace defaults.

### FR-2: Template Listing

- `nfw templates` must display all available starter templates with stable identifiers and descriptions.
- Output must include template identifier, name, description, and supported languages.
- Templates must be tagged by category (blank, minimal, full-featured).

### FR-3: Workspace Creation

- `nfw new [workspace-name]` must create a valid NFramework workspace.
- In an interactive terminal, missing required `nfw new` inputs must be collected through prompts before generation starts.
- `--template <id>` must select a specific template without prompting for template choice.
- `--no-input` must disable all interactive questions and require every remaining required input to be supplied explicitly.
- The workspace must include: solution file, directory structure, configuration files, documentation.
- The workspace must be immediately buildable with a single documented command.
- The command must validate workspace name against naming conventions.
- The command must fail if the target directory already exists (unless `--force` is specified).

### FR-4: Service Addition

- `nfw add service <name> --lang <language>` must generate a complete service scaffold.
- Generated projects must include the standard layer structure for the target language.
- Each project must have correct references and allowed dependencies.
- The service must compile immediately after generation.
- The service must include sample health/readiness endpoints.

### FR-5: Service Addition (Go/Rust)

- `nfw add service <name> --lang go|rust` must generate the standard folder structure.
- Generated structure must match the NFramework layer model.
- Generated code must include build configuration files.
- Documentation must specify build, test, and run commands for the language.

### FR-6: Entity Generation

- `nfw add entity <name> --props <properties>` must generate the entity class with identity and properties only.
- Property syntax must support: `Name:Type`, `Name:Type:modifier` (required, nullable, etc.).
- The entity class must be placed in the correct Domain layer directory.
- The entity must use NFramework domain abstractions.

### FR-7: CRUD Generation

- `nfw add crud <entity-name>` must generate full CRUD scaffolding for an existing entity.
- Generated artifacts must include:
  - DTOs for create/update/response.
  - Create, Update, Delete commands with handlers.
  - GetById and List queries with handlers.
  - Repository contract interface.
  - HTTP endpoint mappings.
- All files must be placed in correct layer directories.
- Generated code must build without errors.

### FR-8: Command Generation

- `nfw add command <name> <feature>` must generate command flow boilerplate.
- Generated artifacts must include:
  - Command record/class with request properties.
  - Response type.
  - Handler implementation.
  - Optional HTTP route registration.
- The command must create the feature folder if it does not exist.
- Optional flags must support: `--cached`, `--logged`, `--transactional`, `--secured`, `--no-api`.

### FR-9: Query Generation

- `nfw add query <name> <feature>` must generate query flow boilerplate.
- Generated artifacts must include:
  - Query record/class with request properties.
  - Response type.
  - Handler implementation.
  - Optional HTTP route registration.
- The command must create the feature folder if it does not exist.
- Optional flags must match command generation (cache, log, secure, API exposure).

### FR-10: Architecture Validation

- `nfw check` must scan all projects in the workspace.
- The command must detect:
  - Forbidden project references across layers.
  - Forbidden namespace/package imports in domain and application layers.
  - Direct infrastructure dependencies in core layers.
- The command must exit with non-zero status on violations.
- Error output must identify: violating file/project, rule violated, remediation steps.
- The command must support CI execution without interactive input.

### FR-11: Input Validation

- All commands must validate arguments before generation.
- Invalid identifiers must produce actionable error messages.
- Invalid property syntax must show correct syntax examples.
- Invalid option combinations must be detected and reported.
- Validation errors must suggest corrective actions.

### FR-12: Help and Documentation

- Every command must have `--help` flag with usage, examples, and options.
- Help text must include realistic examples, not placeholders.
- Error messages must reference the help command for guidance.
- The CLI must provide a `--verbose` flag for diagnostic output.

## 9. Non-Functional Requirements

### Performance

- Workspace creation must complete in under 1 second.
- Full CRUD entity generation must complete in under 10 seconds.
- Architecture validation must complete in under 5 seconds for a standard workspace.
- The CLI executable must start up in under 100ms.

### Reliability

- Generated code must compile without errors 100% of the time.
- The CLI must handle Ctrl+C gracefully and clean up partial state.
- The CLI must never leave a workspace in an invalid state.

### Usability

- All commands must be discoverable through help text.
- Error messages must be specific and actionable.
- Interactive prompts must have clear defaults.
- Progress indicators must be shown for long-running operations.

### Compatibility

- The CLI must run on Linux, macOS, and Windows.
- The CLI must detect and validate required SDK installations for target languages.
- The CLI must be container-friendly for CI/CD environments.

## 10. Command Reference

```text
nfw templates [--lang <language>] [--category <category>]
    List available starter templates

nfw new [workspace-name] [--template <id>] [--no-input] [--force]
    Create a new NFramework workspace

nfw add service <name> --lang <go|rust> [--force]
    Add a new service to the workspace

nfw add entity <name> --props <prop1:type,prop2:type,...>
    Generate entity class only

nfw add crud <entity-name> [options]
    Generate full CRUD scaffolding for an existing entity

nfw add command <name> <feature> [options]
    Generate standalone command handler

nfw add query <name> <feature> [options]
    Generate standalone query handler

nfw check [--verbose]
    Validate architecture boundaries

nfw up [--detached]
    Start local development environment (post-beta)

nfw sync [--watch]
    Synchronize Protobuf contracts (post-beta)

nfw --version
    Display CLI version

nfw --help
    Display help information
```

### Common Options

- `--force` — Overwrite existing files (use with caution).
- `--verbose` — Enable detailed diagnostic output.
- `--no-input` — Disable interactive prompts and require explicit input values.
- `--dry-run` — Show what would be generated without writing files.

### Generation Options (entity, crud, command, query)

- `--cached` — Enable caching behavior in generated handlers.
- `--logged` — Enable logging behavior in generated handlers.
- `--transactional` — Wrap handler execution in a transaction.
- `--secured` — Mark operation as requiring authentication.
- `--no-api` — Skip HTTP endpoint generation.

## 11. Exit Codes

| Code | Meaning              |
| ---- | -------------------- |
| 0    | Success              |
| 1    | Runtime failure      |
| 2    | Usage error          |
| 130  | Interrupted (SIGINT) |

## 12. Design Considerations

### Template System

Templates should be:

- Versioned and independently updateable.
- Discoverable through a local cache or remote catalog.
- Composable (base + layers).
- Validated before distribution.

### Code Generation

Generated code should be:

- Human-readable and suitable for code review.
- Marked with generation headers for identification.
- Regenerable with idempotency where safe.
- Preserving user modifications where indicated.

### Error Handling

- Distinguish between user errors (fixable) and system errors (bugs).
- Provide error codes for programmatic handling.
- Include suggested fixes for common errors.
- Log verbose diagnostics to a configurable location.

### Configuration

- Support workspace-level configuration (`nfw.yaml`).
- Support user-level configuration (`~/.nfw/config.yaml`).
- Allow workspace defaults to override user preferences.
- Document all configuration options.

## 13. Open Questions

1. What is the exact template format and composition model?
2. Should the CLI integrate with IDEs (VS Code, JetBrains, etc.)?
3. How should the CLI handle workspace schema migrations between versions?
4. What is the strategy for custom templates within an organization?

## 14. Success Metrics

- A new user can create and build their first workspace in under 5 minutes.
- Generated services have 0 compiler warnings.
- Architecture validation completes in under 5 seconds for 20-project workspaces.
- Error messages enable issue resolution without external documentation in 90% of cases.
- Average time to add a new entity with CRUD is under 10 seconds end-to-end.

## 15. Dependencies

### Internal

- NFramework domain and application abstractions for generated entities and workflows.
- Source generators for language-specific registration and routing.
- Template definitions — Workspace and service scaffolds.

### External

- Target language SDKs — For service generation and compilation (Go, Rust, or other supported languages).
- Git — For workspace initialization (optional).
