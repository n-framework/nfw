# Feature Specification: nfw add mediator Command

## User Scenarios & Testing

### User Story 1 - Add Mediator to Service (Priority: P1)

As a developer, I want to add the Mediator module to an existing service so that I can enable CQRS patterns and mediator-based request handling in my application.

**Why this priority**: This is a critical capability for enabling the mediator pattern in NFramework services. Without this feature, developers cannot easily add mediator capabilities to existing services, blocking adoption of CQRS patterns.

**Independent Test**: Can be tested independently by verifying that the command adds the mediator module to `nfw.yaml`, renders template files, and preserves workspace state on failure. The test delivers value by ensuring developers can reliably add mediator capabilities to their services.

**Acceptance Scenarios**:

1. **Given** the nfw CLI is installed in a workspace with at least one service, **When** I run `nfw add mediator --service TestService --no-input`, **Then** the mediator module is added to the TestService configuration in `nfw.yaml`, template files are rendered in the service directory, and the command completes successfully.

2. **Given** the nfw CLI is installed in a workspace with multiple services, **When** I run `nfw add mediator` without specifying a service, **Then** I am presented with an interactive prompt to select from available services, and upon selection, the mediator module is added to the chosen service.

3. **Given** the nfw CLI is installed in a workspace with exactly one service, **When** I run `nfw add mediator --no-input` without specifying a service, **Then** the command automatically selects the single available service and adds the mediator module.

4. **Given** the nfw CLI is installed in a workspace, **When** I run `nfw add mediator --service NonExistentService --no-input`, **Then** the command fails with an error message indicating that the service was not found in the workspace, and no changes are made to `nfw.yaml`.

5. **Given** the nfw CLI is installed and I have a service with mediator templates configured, **When** the template execution fails after updating `nfw.yaml`, **Then** the `nfw.yaml` changes are rolled back, preserving the original state with all comments intact.

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

- **YAML comment preservation**: When adding the mediator module to `nfw.yaml`, all existing comments (top-level, inline, section-level, and block comments) must be preserved in their original positions.

- **Missing template directory**: When the configured template directory does not exist or is inaccessible, the command must fail with a specific error about template resolution.

---

## Requirements

### Functional Requirements

- **FR-001**: The command MUST support adding the mediator module to existing services defined in the workspace configuration.

- **FR-002**: The command MUST support the `--service <NAME>` parameter to specify the target service by name.

- **FR-003**: The command MUST support the `--no-input` flag to enable automated, non-interactive execution.

- **FR-004**: The command MUST provide interactive service selection when no service is specified and multiple services exist, using the interactive prompt system.

- **FR-005**: The command MUST automatically select the single available service when `--no-input` is used and only one service exists in the workspace.

- **FR-006**: The command MUST update the `nfw.yaml` configuration file to register the mediator module in the target service's `modules` array.

- **FR-007**: The command MUST execute template rendering for the mediator module using the configured template engine and templates.

- **FR-008**: The command MUST complete execution within 5 seconds for typical workspaces with standard template complexity.

- **FR-009**: The command MUST preserve all YAML comments (top-level, inline, section-level, and block comments) when updating `nfw.yaml`.

- **FR-010**: The command MUST roll back any changes made to `nfw.yaml` if template execution fails, ensuring atomic operation.

- **FR-011**: The command MUST validate that the specified service exists in the workspace before attempting to add the mediator module.

- **FR-012**: The command MUST fail with a clear, actionable error message when no services are found in the workspace.

- **FR-013**: The command MUST use the template context to resolve template roots and configurations for the mediator module.

- **FR-014**: The command MUST construct template parameters including service name, namespace, and workspace context for template rendering.

- **FR-015**: The command MUST report success with appropriate user feedback including spinner progress and success messages.

- **FR-016**: The command MUST integrate with the existing CLI abstraction layer for consistent command handling and error reporting.

- **FR-017**: The command MUST support both interactive terminal detection and automated execution modes.

- **FR-018**: The command MUST use the ArtifactGenerationService for workspace operations, template execution, and service module registration.

### Key Entities

- **AddMediatorCommand**: A command model containing service information and workspace context for adding the mediator module to a service.

- **AddMediatorCommandHandler**: A command handler that orchestrates the mediator addition workflow including workspace context loading, template execution, and service module registration.

- **AddMediatorCliCommand**: A CLI command adapter that handles user interaction, parameter parsing, and service selection, delegating core logic to the command handler.

- **ArtifactGenerationService**: A service responsible for template context loading, workspace operations, template engine execution, and service module registration.

- **TemplateParameters**: A value object containing template rendering parameters including name, namespace, and service information.

- **WorkspaceContext**: A value object containing workspace configuration, root path, and parsed `nfw.yaml` content.

- **ServiceInfo**: A value object representing a service with its name, path, and template configuration.

- **AddArtifactError**: An error type representing various failure scenarios including workspace errors, parameter errors, and execution failures.

---

## Success Criteria

### Measurable Outcomes

- **SC-001**: The command MUST successfully add the mediator module to at least 95% of valid service configurations without errors.

- **SC-002**: The command MUST preserve 100% of existing YAML comments when updating `nfw.yaml`, as verified by automated tests.

- **SC-003**: The command MUST complete execution within 5 seconds for workspaces with up to 10 services and standard template complexity.

- **SC-004**: The command MUST roll back `nfw.yaml` changes within 1 second when template execution fails, ensuring no partial state is left.

- **SC-005**: Integration tests MUST cover at least 90% of code paths including successful addition, rollback scenarios, service validation, and YAML comment preservation.

- **SC-006**: The command MUST provide clear error messages for all failure scenarios, with error types mapped to appropriate exit codes.

- **SC-007**: The command MUST handle concurrent execution safely without corrupting `nfw.yaml` when multiple instances attempt modifications.

- **SC-008**: Template execution MUST use the configured template engine and produce valid output files in the target service directory.

---

## Assumptions

- The workspace has a valid `nfw.yaml` configuration file with at least one service defined.

- Template sources are configured and accessible, with the mediator template available in the configured template repository.

- The user has write permissions for the workspace directory, `nfw.yaml`, and target service directories.

- The interactive prompt system is available when running in interactive mode (terminal detected).

- The template engine is properly configured and can resolve template roots and execute template steps.

- The ArtifactGenerationService handles workspace operations, template context loading, and service module registration.

- YAML parsing and serialization preserve comments and structure using the appropriate YAML library.

- File system operations are atomic or can be rolled back to ensure workspace integrity.

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

---

## Clarifications

- **Q**: Should the command create the service directory if it doesn't exist? → **A**: No, the command should only add the mediator module to existing services. Service creation is handled by the `nfw add service` command.

- **Q**: What happens if the mediator module is already added to the service? → **A**: The command should detect this and report that the mediator module is already present, avoiding duplicate entries and redundant template execution.

- **Q**: Should the command support removing the mediator module? → **A**: No, removal functionality is out of scope for this command. A separate `nfw remove mediator` command would handle module removal.

- **Q**: How should the command handle custom template sources? → **A**: The command uses the configured template sources from `nfw.yaml` and the template root resolver, supporting both local and remote template repositories as configured by the workspace.

- **Q**: What template parameters are required for mediator module rendering? → **A**: The command provides at minimum the service name, namespace, and workspace context. Additional parameters may be added as required by the template configuration.

- **Q**: Should the command validate the generated code after template execution? → **A**: Template validation is the responsibility of the template engine. The command ensures template execution completes successfully but does not perform additional code validation.

- **Q**: How does the command handle different service types (e.g., .NET, Go, Rust)? → **A**: The command is template-agnostic and delegates service-specific logic to the template system. Different service types use different templates configured in the workspace.

---

## Non-goals

- **Module removal**: This command does not support removing the mediator module from services. Removal functionality would be provided by a separate command.

- **Template creation**: This command does not create or modify mediator templates. Template management is handled separately through the template system.

- **Service creation**: This command does not create new services. Service creation is handled by the `nfw add service` command.

- **Code validation**: This command does not perform post-generation code validation beyond ensuring template execution succeeds.

- **Dependency management**: This command does not manage package dependencies or project files beyond what the templates render.

- **Multi-service batch operations**: This command adds the mediator module to one service at a time. Batch operations are not supported.

- **Template debugging**: This command does not provide template debugging capabilities. Template issues should be debugged using dedicated template development tools.

- **Workspace migration**: This command does not handle workspace structure migrations or updates to existing mediator configurations.

- **Interactive template customization**: This command does not support interactive customization of template parameters. All customization is done through template configuration and command parameters.
