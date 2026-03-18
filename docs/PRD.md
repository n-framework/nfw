# Product Requirements Document (PRD): nfw CLI

## 1. Introduction / Overview

The `nfw` CLI is the primary developer-facing tool for creating, managing, and validating NFramework workspaces and services. It serves as the entry point for all framework lifecycle operations, providing template-based project generation, code scaffolding, architecture validation, and local orchestration capabilities.

The CLI exists to eliminate manual project setup, enforce architectural standards through automation, and provide a consistent developer experience across the polyglot NFramework ecosystem. It transforms complex multi-project scaffolding into single-command operations while maintaining the strict layer boundaries and clean architecture principles that define NFramework services.

## 2. Problem Statement

Developers building microservices with clean architecture face significant friction:

- Creating a new service requires manually creating multiple projects, configuring dependencies, and establishing folder structures
- Clean Architecture boundaries are documented but not enforced, leading to gradual decay
- CRUD operations, commands, and queries require repetitive boilerplate across many files
- Local development environments require complex orchestration of multiple services and dependencies
- Architecture violations are only discovered during code review or in production
- Polyglot teams struggle to maintain consistent structures across different languages
- New team members have long onboarding times due to complex setup procedures

The `nfw` CLI must solve these problems by providing fast, opinionated commands that generate correct-by-default code, validate architecture automatically, and streamline the entire development lifecycle.

## 3. Goals

- Provide instant workspace and service scaffolding through single-command operations
- Generate production-ready code that compiles without manual modifications
- Enforce Clean Architecture boundaries through automated validation
- Reduce time-to-first-service and time-to-first-endpoint to seconds
- Support the full NFramework development lifecycle: create, add, generate, validate, run
- Maintain consistent workspace structure across supported languages (.NET, Go, Rust)
- Enable template-based customization for different service patterns
- Provide clear, actionable error messages and validation feedback
- Support both interactive and non-interactive (CI-friendly) workflows
- Integrate with build tools, source generators, and local orchestration

## 4. Target Users

- **Platform Engineers**: Create and maintain workspace templates for their organizations
- **Tech Leads**: Enforce architectural standards across teams through automated validation
- **Application Developers**: Quickly scaffold services, entities, commands, and queries
- **DevOps Engineers**: Integrate CLI commands into CI/CD pipelines
- **New Team Members**: Onboard rapidly through standardized project generation

## 5. Product Principles

1. **Instant Feedback**: Commands should complete in under 3 seconds for workspace creation and under 10 seconds for full CRUD generation
2. **Correct by Default**: Generated code must compile and pass basic validation without manual edits
3. **Actionable Errors**: Every failure must state what went wrong, why, and exactly how to fix it
4. **Idempotent Where Possible**: Rerunning commands should be safe and predictable
5. **CI-First Design**: All commands must work without interactive input for automation
6. **Opinionated Standards**: The CLI enforces NFramework conventions while allowing template-based customization

## 6. Scope

### In Scope for Initial Beta

#### Core Commands

- `nfw templates` — List available starter templates
- `nfw new [workspace-name] [--template <id>] [--no-input]` — Create new workspace
- `nfw add service <name> --lang <dotnet|go|rust>` — Add service scaffold
- `nfw add entity <name> --props <properties>` — Generate entity with CRUD flow
- `nfw add command <name> <feature>` — Generate command handler
- `nfw add query <name> <feature>` — Generate query handler
- `nfw check` — Validate architecture boundaries

#### Language Support

- Full .NET service generation with source generator integration
- Folder structure and conventions for Go and Rust (code generation deferred)

#### Validation

- Project reference validation across layers
- Namespace/package usage validation
- Dependency rule enforcement
- CI-exit codes for automated checks

### Planned Post-Beta Scope

- `nfw up` — Local orchestration (Aspire/Dapr integration)
- `nfw sync` — Protobuf contract synchronization across languages
- Remote template catalogs
- Workspace update and migration commands
- Advanced configuration and customization commands

### Out of Scope

- Graphical user interface for project generation
- Runtime service management beyond local development orchestration
- Custom language support outside .NET/Go/Rust
- Template editor or builder CLI

## 7. Functional Requirements

### FR-1: Core CLI Infrastructure

- The system must provide an `nfw` executable available through standard package managers
- The CLI must support subcommands with consistent help text and examples
- The CLI must support both interactive prompts and flag-based non-interactive mode
- The CLI must provide version information and update notifications
- The CLI must support configuration files for user preferences and workspace defaults

### FR-2: Template Listing

- `nfw templates` must display all available starter templates
- Output must include template identifier, name, description, and supported languages
- Templates must be tagged by category (blank, minimal, full-featured)
- The command must support filtering by language or category

### FR-3: Workspace Creation

- `nfw new [workspace-name]` must create a valid NFramework workspace
- In an interactive terminal, missing required `nfw new` inputs must be collected through prompts before generation starts
- `--template <id>` must select a specific template without prompting for template choice
- `--no-input` must disable all interactive questions and require every remaining required input to be supplied explicitly
- The workspace must include: solution file, directory structure, configuration files, documentation
- The workspace must be immediately buildable with a single documented command
- The command must validate workspace name against naming conventions
- The command must fail if the target directory already exists (unless `--force` is specified)
- Template resolution and interactive prompt behavior must be deterministic and documented

### FR-4: Service Addition (.NET)

- `nfw add service <name> --lang dotnet` must generate a complete service scaffold
- Generated projects must include: Domain, Application, Infrastructure, Api layers
- Each project must have correct references and allowed dependencies
- The service must compile immediately after generation
- The service must include sample health/readiness endpoints
- Project and namespace names must follow NFramework conventions
- The command must validate service names against identifier rules

### FR-5: Service Addition (Go/Rust)

- `nfw add service <name> --lang go|rust` must generate the standard folder structure
- Generated structure must match the NFramework layer model
- Generated code must include build configuration files
- Documentation must specify build, test, and run commands for the language

### FR-6: Entity Generation

- `nfw add entity <name> --props <properties>` must generate complete CRUD boilerplate
- Property syntax must support: `Name:Type`, `Name:Type:modifier` (required, nullable, etc.)
- Generated artifacts must include:
  - Entity class with identity and properties
  - DTOs for create/update/response
  - Repository interface
  - Create/Update/Delete commands
  - GetById/List/Query queries
  - HTTP endpoint mappings
- All files must be placed in correct layer directories
- Generated code must build without errors

### FR-7: Command Generation

- `nfw add command <name> <feature>` must generate command flow boilerplate
- Generated artifacts must include:
  - Command record/class with request properties
  - Response type
  - Handler implementation
  - Optional HTTP route registration
- The command must create the feature folder if it doesn't exist
- Optional flags must support: `--cached`, `--logged`, `--transactional`, `--secured`, `--no-api`

### FR-8: Query Generation

- `nfw add query <name> <feature>` must generate query flow boilerplate
- Generated artifacts must include:
  - Query record/class with request properties
  - Response type
  - Handler implementation
  - Optional HTTP route registration
- The command must create the feature folder if it doesn't exist
- Optional flags must match command generation (cache, log, secure, API exposure)

### FR-9: Architecture Validation

- `nfw check` must scan all projects in the workspace
- The command must detect:
  - Forbidden project references across layers
  - Forbidden namespace/package imports in domain and application layers
  - Direct infrastructure dependencies in core layers
- The command must exit with non-zero status on violations
- Error output must identify: violating file/project, rule violated, remediation steps
- The command must support `--fix` for auto-fixable violations (future)

### FR-10: Input Validation

- All commands must validate arguments before generation
- Invalid identifiers must produce actionable error messages
- Invalid property syntax must show correct syntax examples
- Invalid option combinations must be detected and reported
- Validation errors must suggest corrective actions

### FR-11: Help and Documentation

- Every command must have `--help` flag with usage, examples, and options
- Help text must include realistic examples, not placeholders
- Error messages must reference the help command for guidance
- The CLI must provide a `--verbose` flag for diagnostic output

## 8. Non-Functional Requirements

### Performance

- Workspace creation must complete in under 3 seconds
- Full CRUD entity generation must complete in under 10 seconds
- Architecture validation must complete in under 5 seconds for a standard workspace
- The CLI executable must start up in under 100ms

### Reliability

- Generated code must compile without errors 100% of the time
- The CLI must handle Ctrl+C gracefully and clean up partial state
- The CLI must never leave a workspace in an invalid state

### Usability

- All commands must be discoverable through help text
- Error messages must be specific and actionable
- Interactive prompts must have clear defaults
- Progress indicators must be shown for long-running operations

### Compatibility

- The CLI must run on Linux, macOS, and Windows
- The CLI must support .NET 11 SDK detection and validation
- The CLI must be container-friendly for CI/CD environments

## 9. Command Reference

```
nfw templates [--lang <language>] [--category <category>]
    List available starter templates

nfw new [workspace-name] [--template <id>] [--no-input] [--force]
    Create a new NFramework workspace

nfw add service <name> --lang <dotnet|go|rust> [--force]
    Add a new service to the workspace

nfw add entity <name> --props <prop1:type,prop2:type,...> [options]
    Generate entity with CRUD flow

nfw add command <name> <feature> [options]
    Generate command handler

nfw add query <name> <feature> [options]
    Generate query handler

nfw check [--fix] [--verbose]
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

- `--force` — Overwrite existing files (use with caution)
- `--verbose` — Enable detailed diagnostic output
- `--no-input` — Disable interactive prompts and require explicit input values
- `--dry-run` — Show what would be generated without writing files

## 10. Exit Codes

- `0` — Success
- `1` — General error
- `2` — Invalid arguments or usage
- `3` — Validation failure (architecture violations)
- `4` — Template not found
- `5` — Workspace already exists
- `6` — Generation failure (compiler error, invalid state)

## 11. Design Considerations

### Template System

Templates should be:

- Versioned and independently updateable
- Discoverable through a local cache or remote catalog
- Composable (base + layers)
- Validated before distribution

### Code Generation

Generated code should be:

- Human-readable and suitable for code review
- Marked with generation headers for identification
- Regenerable with idempotency where safe
- Preserving user modifications where indicated

### Error Handling

- Distinguish between user errors (fixable) and system errors (bugs)
- Provide error codes for programmatic handling
- Include suggested fixes for common errors
- Log verbose diagnostics to a configurable location

### Configuration

- Support workspace-level configuration (nfw.yaml)
- Support user-level configuration (~/.nfw/config.yaml)
- Allow workspace defaults to override user preferences
- Document all configuration options

## 12. Open Questions

1. Should the CLI be implemented in Go or Native AOT C#?
2. Should templates support variable interpolation for custom naming patterns?
3. What is the exact template format and composition model?
4. Should `nfw check` support auto-fix for certain violation types?
5. How should the CLI handle workspace schema migrations between versions?
6. Should there be an `nfw init` command for adding nfw to existing projects?
7. What is the strategy for custom templates within an organization?
8. How should the CLI integrate with IDEs (VS Code, Rider, Visual Studio)?

## 13. Success Metrics

- A new user can create and build their first workspace in under 5 minutes
- Generated .NET services have 0 compiler warnings
- Architecture validation completes in under 5 seconds for 20-project workspaces
- Error messages enable issue resolution without external documentation in 90% of cases
- CLI adoption reaches 80% of NFramework users within 3 months of release
- Average time to add a new entity with CRUD is under 30 seconds end-to-end

## 14. Dependencies

### Internal

- NFramework.Domain — Base abstractions for generated entities
- NFramework.Application — Result types and CQRS contracts
- Source generators — DI registration and route generation
- Template definitions — Workspace and service scaffolds

### External

- .NET 11 SDK — For .NET service generation and compilation
- Git — For workspace initialization (optional)
- Build tools — dotnet, go, rustc depending on language

## 15. Related Documentation

- [NFramework PRD](https://github.com/n-framework/n-framework/blob/main/docs/PRD.md) — Overall framework requirements
