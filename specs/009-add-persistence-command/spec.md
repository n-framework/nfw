# Feature Specification: nfw add persistence Command

## User Scenarios & Testing

### User Story 1 - Add Persistence to Service (Priority: P1)

As a developer, I want to add the Persistence module to an existing service so that I can enable data access patterns, repository-based CRUD operations, and database context management in my application.

**Why this priority**: This is a critical capability for enabling data persistence in NFramework services. Without this feature, developers cannot easily add persistence capabilities to existing services, blocking adoption of repository patterns and database operations.

**Independent Test**: Can be tested independently by verifying that the command adds the persistence module to `nfw.yaml`, renders template files for DbContext and repository base classes, and preserves workspace state on failure. The test delivers value by ensuring developers can reliably add persistence capabilities to their services.

**Acceptance Scenarios**:

1. **Given** the nfw CLI is installed in a workspace with at least one service, **When** I run `nfw add persistence --service TestService --no-input`, **Then** the persistence module is added to the TestService configuration in `nfw.yaml`, template files are rendered in the service directory including DbContext and repository base classes, and the command completes successfully.

2. **Given** the nfw CLI is installed in a workspace with multiple services, **When** I run `nfw add persistence` without specifying a service, **Then** I am presented with an interactive prompt to select from available services, and upon selection, the persistence module is added to the chosen service.

3. **Given** the nfw CLI is installed in a workspace with exactly one service, **When** I run `nfw add persistence --no-input` without specifying a service, **Then** the command automatically selects the single available service and adds the persistence module.

4. **Given** the nfw CLI is installed in a workspace, **When** I run `nfw add persistence --service NonExistentService --no-input`, **Then** the command fails with an error message indicating that the service was not found in the workspace, and no changes are made to `nfw.yaml`.

5. **Given** the nfw CLI is installed and I have a service with persistence templates configured, **When** the template execution fails after updating `nfw.yaml`, **Then** the `nfw.yaml` changes are rolled back, preserving the original state with all comments intact.

6. **Given** a service with the persistence module already added, **When** I run `nfw add persistence` for the same service, **Then** the command detects the existing persistence module and reports that it is already present, avoiding duplicate entries and redundant template execution.

---

### User Story 2 - Generate Persistence Artifacts (Priority: P1)

**Note**: This is an acceptance scenario of US1, not a separate user story. Artifact generation is a core part of adding persistence to a service.

As a developer, I want the persistence module addition to generate all necessary artifacts including DbContext, repository base classes, and configuration so that I have a complete foundation for data access without manual setup.

**Why this priority**: Complete artifact generation is essential for developer productivity. Without it, developers would need to manually create DbContext, configure repositories, and set up database configuration, which is error-prone and time-consuming.

**Independent Test**: Can be tested by verifying that after adding persistence, the service directory contains all expected artifacts: a configured DbContext class, repository base classes with generic CRUD methods, and configuration files for database connection settings.

**Acceptance Scenarios** (for US1 - Artifact Generation):

1. **Given** a service without persistence, **When** I add the persistence module, **Then** a DbContext class is generated with proper namespace inheritance and configuration support.

2. **Given** a service receiving the persistence module, **When** template execution completes, **Then** repository base classes are generated with generic CRUD methods (add, update, delete, getById, getAll, query with pagination).

3. **Given** a service with existing entities, **When** persistence is added, **Then** the DbContext is configured to discover and register entity configurations from the service's domain layer.

4. **Given** the generated artifacts, **When** I build the service, **Then** all generated code compiles without errors and follows the framework's naming conventions.

---

### User Story 3 - Configure Database Connection (Priority: P2)

**Note**: This is an acceptance scenario of US1, not a separate user story. Database configuration generation is part of the template rendering process when adding persistence.

As a developer, I want the persistence module addition to generate database configuration so that my service can connect to a database without manual configuration steps.

**Why this priority**: Database configuration is a necessary part of persistence setup. While developers can configure it manually, having it generated saves time and ensures consistency with framework conventions.

**Independent Test**: Can be tested by verifying that configuration files or settings are generated with placeholder connection strings that can be easily updated for specific environments.

**Acceptance Scenarios** (for US1 - Database Configuration):

1. **Given** a service receiving the persistence module, **When** template execution completes, **Then** configuration artifacts are generated with database connection settings following the framework's configuration pattern.

2. **Given** the generated configuration, **When** I update the connection string, **Then** the service can connect to the specified database without additional code changes.

3. **Given** different database providers, **When** persistence templates are executed, **Then** the generated configuration supports the provider specified in the service template (e.g., SQL Server, PostgreSQL, SQLite).

---

## Edge Cases

- **No services in workspace**: When the command is run in a workspace with no services, the command must fail with a clear error message instructing the user to add a service first.

- **Service not found**: When the specified service name does not exist in the workspace, the command must fail with an actionable error message containing the invalid service name.

- **Template execution failure**: When template rendering fails (missing template file, invalid template syntax, permission errors), the command must roll back any changes made to `nfw.yaml` and report the specific failure reason.

- **Concurrent execution**: When multiple instances of the command attempt to modify `nfw.yaml` simultaneously, the command must handle file locking or detect conflicts to prevent corruption.

- **Missing nfw.yaml**: When the command is run outside of a valid workspace (no `nfw.yaml` present), the command must fail with a clear error message indicating that no workspace was found.

- **Invalid YAML structure**: When `nfw.yaml` exists but has an invalid structure preventing module addition, the command must fail with a specific error about the structural issue.

- **Permission errors**: When the user lacks write permissions for `nfw.yaml` or the service directory, the command must fail with a clear permission error message without making partial changes.

- **Interrupt signal (Ctrl+C)**: When the user interrupts the command during execution, any partial changes to `nfw.yaml` must be rolled back to preserve workspace integrity.

- **YAML comment preservation**: When adding the persistence module to `nfw.yaml`, all existing comments (top-level, inline, section-level, and block comments) must be preserved in their original positions.

- **Missing template directory**: When the configured template directory does not exist or is inaccessible, the command must fail with a specific error about template resolution.

- **Persistence module already present**: When the persistence module is already registered in the target service, the command must detect this and report that the module is already present without attempting to add it again or re-execute templates.

- **Conflicting modules**: When the service has modules that may conflict with persistence (e.g., manual data access patterns), the command should proceed but may warn about potential conflicts.

- **Entity framework version mismatch**: When the project references an incompatible Entity Framework Core version, the command must fail with a clear error about version requirements.

- **Missing database provider**: When the service template does not specify a database provider, the command must use a sensible default or prompt the user to select one.

---

## Requirements

### Functional Requirements

- **FR-001**: The command MUST support adding the persistence module to existing services defined in the workspace configuration.

- **FR-002**: The command MUST support the `--service <NAME>` parameter to specify the target service by name.

- **FR-003**: The command MUST support the `--no-input` flag to enable automated, non-interactive execution.

- **FR-004**: The command MUST provide interactive service selection when no service is specified and multiple services exist, using the interactive prompt system.

- **FR-005**: The command MUST automatically select the single available service when `--no-input` is used and only one service exists in the workspace.

- **FR-006**: The command MUST update the `nfw.yaml` configuration file to register the persistence module in the target service's `modules` array.

- **FR-007**: The command MUST complete execution within 5 seconds for typical workspaces with standard template complexity.

- **FR-008**: The command MUST preserve all YAML comments (top-level, inline, section-level, and block comments) when updating `nfw.yaml`.

- **FR-009**: The command MUST roll back any changes made to `nfw.yaml` if template execution fails, ensuring atomic operation.

- **FR-010**: The command MUST validate that the specified service exists in the workspace before attempting to add the persistence module.

- **FR-011**: The command MUST fail with a clear, actionable error message when no services are found in the workspace.

- **FR-012**: The command MUST use the template context to resolve template roots and configurations for the persistence module.

- **FR-013**: The command MUST construct template parameters including service name, namespace, and workspace context for template rendering.

- **FR-014**: The command MUST report success with appropriate user feedback including spinner progress and success messages.

- **FR-015**: The command MUST integrate with the existing CLI abstraction layer for consistent command handling and error reporting.

- **FR-016**: The command MUST support both interactive terminal detection and automated execution modes.

- **FR-017**: The command MUST use the ArtifactGenerationService for workspace operations, template execution, and service module registration.

- **FR-018**: The command MUST generate a DbContext class with proper namespace inheritance and configuration support for entity registration.

- **FR-019**: The command MUST generate repository base classes with generic CRUD methods following the framework's repository pattern.

- **FR-020**: The command MUST generate database configuration artifacts with connection string placeholders.

- **FR-021**: The command MUST detect when the persistence module is already present in a service and report accordingly without redundant operations.

### Key Entities

- **AddPersistenceCommand**: A command model containing service information and workspace context for adding the persistence module to a service.

- **AddPersistenceCommandHandler**: A command handler that orchestrates the persistence addition workflow including workspace context loading, template execution, and service module registration.

- **AddPersistenceCliCommand**: A CLI command adapter that handles user interaction, parameter parsing, and service selection, delegating core logic to the command handler.

- **ArtifactGenerationService**: A service responsible for template context loading, workspace operations, template engine execution, and service module registration.

- **TemplateParameters**: A value object containing template rendering parameters including name, namespace, and service information.

- **WorkspaceContext**: A value object containing workspace configuration, root path, and parsed `nfw.yaml` content.

- **ServiceInfo**: A value object representing a service with its name, path, and template configuration.

- **AddArtifactError**: An error type representing various failure scenarios including workspace errors, parameter errors, and execution failures.

- **DbContext**: A generated Entity Framework Core database context class that manages entity mappings and database operations.

- **RepositoryBase**: Generated base classes providing generic CRUD operations for repository implementations.

---

## Success Criteria

### Measurable Outcomes

- **SC-001**: The command MUST successfully add the persistence module to at least 95% of valid service configurations without errors.

- **SC-002**: The command MUST preserve 100% of existing YAML comments when updating `nfw.yaml`, as verified by automated tests.

- **SC-003**: The command MUST complete execution within 5 seconds for workspaces with up to 10 services and standard template complexity.

- **SC-004**: The command MUST roll back `nfw.yaml` changes within 1 second when template execution fails, ensuring no partial state is left.

- **SC-005**: Integration tests MUST cover at least 90% of code paths including successful addition, rollback scenarios, service validation, YAML comment preservation, and duplicate module detection.

- **SC-006**: The command MUST provide clear error messages for all failure scenarios, with error types mapped to appropriate exit codes.

- **SC-007**: The command MUST handle concurrent execution safely without corrupting `nfw.yaml` when multiple instances attempt modifications.

- **SC-008**: Template execution MUST use the configured template engine and produce valid output files including DbContext, repository base classes, and configuration artifacts in the target service directory.

- **SC-009**: Generated artifacts MUST compile without errors and follow the framework's naming and architectural conventions.

- **SC-010**: The command MUST detect existing persistence modules and skip redundant operations without error.

---

## Assumptions

- The workspace has a valid `nfw.yaml` configuration file with at least one service defined.

- Template sources are configured and accessible, with the persistence template available in the configured template repository.

- The user has write permissions for the workspace directory, `nfw.yaml`, and target service directories.

- The interactive prompt system is available when running in interactive mode (terminal detected).

- The template engine is properly configured and can resolve template roots and execute template steps.

- The ArtifactGenerationService handles workspace operations, template context loading, and service module registration.

- YAML parsing and serialization preserve comments and structure using the appropriate YAML library.

- File system operations are atomic or can be rolled back to ensure workspace integrity.

- The service template specifies or defaults to a database provider (e.g., SQL Server, PostgreSQL, SQLite).

- Entity Framework Core is available as a package dependency in the target service project.

- The generated files follow the framework's layer placement conventions: Infrastructure/Persistence layer for DbContext, repository base classes with dependency injection setup, and Application layer for repository interfaces (same pattern as mediator command).

- The nfw-templates repository includes the persistence template at `templates/dotnet-service/persistence/` with template.yaml configuration and .tera template files for DbContext, RepositoryBase, and configuration generation.

---

## Dependencies

- **nfw CLI workspace**: The command requires a valid nfw workspace with at least one service configured.

- **Template system**: The command depends on the template engine (`TemplateEngine` trait) and template root resolver (`TemplateRootResolver` trait) for template execution.

- **Workspace management**: The command depends on the working directory provider (`WorkingDirectoryProvider` trait) for workspace operations.

- **CLI abstractions**: The command depends on the `n_framework_core_cli_abstractions` crate for `Command`, `InteractivePrompt`, `Logger`, and `SelectOption` traits.

- **Interactive prompts**: The command depends on the interactive prompt service (`n_framework_core_cli_cliclack`) for user interaction in interactive mode.

- **Artifact generation service**: The command depends on the `ArtifactGenerationService` for core workflow orchestration.

- **YAML handling**: The command requires proper YAML parsing and serialization with comment preservation.

- **Exit codes**: The command depends on the `ExitCodes` enum for consistent error reporting.

- **core-persistence-dotnet**: The persistence templates reference abstractions and patterns defined in the NFramework.Persistence package.

- **nfw-templates**: The command requires the persistence template to be available at `templates/dotnet-service/persistence/` in the configured template sources with template.yaml and .tera files for code generation.

- **Entity Framework Core**: The generated DbContext and repository patterns assume Entity Framework Core is the underlying ORM.

---

## Clarifications

- **Q**: Should the command create the service directory if it doesn't exist? → **A**: No, the command should only add the persistence module to existing services. Service creation is handled by the `nfw add service` command.

- **Q**: What happens if the persistence module is already added to the service? → **A**: The command should detect this and report that the persistence module is already present, avoiding duplicate entries and redundant template execution.

- **Q**: Should the command support removing the persistence module? → **A**: No, removal functionality is out of scope for this command. A separate `nfw remove persistence` command would handle module removal.

- **Q**: How should the command handle custom template sources? → **A**: The command uses the configured template sources from `nfw.yaml` and the template root resolver, supporting both local and remote template repositories as configured by the workspace.

- **Q**: What template parameters are required for persistence module rendering? → **A**: The command provides at minimum the service name, namespace, and workspace context. Additional parameters may include database provider type, connection string placeholders, and entity configuration paths as required by the template configuration.

- **Q**: How does the command handle different database providers? → **A**: The command is template-agnostic and delegates provider-specific logic to the template system. Different database providers use different templates configured in the workspace.

- **Q**: Should the command validate the generated code after template execution? → **A**: Template validation is the responsibility of the template engine. The command ensures template execution completes successfully but does not perform additional code validation beyond checking for compilation errors if specified by the template.

- **Q**: What happens if the service project doesn't reference Entity Framework Core? → **A**: The command must fail with a clear error indicating that Entity Framework Core is required for the persistence module. The template system may optionally add the necessary package references as part of template execution.

- **Q**: Should the command create database migrations? → **A**: Database migration creation is out of scope for this command. Migrations can be created separately using EF Core tools or framework-specific commands after the persistence module is added.

---

## Non-goals

- **Module removal**: This command does not support removing the persistence module from services. Removal functionality would be provided by a separate command.

- **Template creation**: This command does not create or modify persistence templates. Template management is handled separately through the template system.

- **Service creation**: This command does not create new services. Service creation is handled by the `nfw add service` command.

- **Code validation**: This command does not perform post-generation code validation beyond ensuring template execution succeeds.

- **Dependency management**: This command does not manage package dependencies or project files beyond what the templates render.

- **Multi-service batch operations**: This command adds the persistence module to one service at a time. Batch operations are not supported.

- **Template debugging**: This command does not provide template debugging capabilities. Template issues should be debugged using dedicated template development tools.

- **Workspace migration**: This command does not handle workspace structure migrations or updates to existing persistence configurations.

- **Interactive template customization**: This command does not support interactive customization of template parameters. All customization is done through template configuration and command parameters.

- **Database schema management**: This command does not create or modify database schemas. Schema management is handled through Entity Framework Core migrations or other schema management tools.

- **Data seeding**: This command does not generate seed data or data initialization logic. Data seeding is handled separately through EF Core seed data mechanisms or custom initialization code.

- **Connection string management**: This command generates connection string placeholders but does not validate, encrypt, or manage connection strings for different environments. Connection string management is the responsibility of the developer or operations team.
