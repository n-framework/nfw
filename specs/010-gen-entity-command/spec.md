# Feature Specification: nfw gen entity Command

**Command**: `nfw gen entity <NAME> --props <DEFINITIONS>`

**Purpose**: Create entity schema files and invoke template engine for code generation. CLI manages schema creation/reading and provides data to templates; templates determine target language, base class, and type mappings.

## User Scenarios & Testing

### User Story 1 - Generate Entity with Properties (Priority: P1)

As a developer, I want to create entity schema files and invoke code generation using a simple CLI command so that I can quickly create domain entities without writing boilerplate code manually.

**Why this priority**: This is a core capability for the NFramework development workflow. Without this feature, developers must manually create entity classes, which is time-consuming and error-prone, blocking rapid application development.

**Independent Test**: Can be tested independently by verifying that the command creates valid schema files and invokes the template engine with correct parameters. The test delivers value by ensuring developers can reliably create entities that conform to NFramework architectural standards.

**Acceptance Scenarios**:

1. **Given** the nfw CLI is installed in a workspace with at least one service that has the persistence module added, **When** I run `nfw gen entity Product --props Name:string,Price:decimal --no-input`, **Then** a schema file is created and the template engine is invoked with entity parameters, generating code using the configured entity template.

2. **Given** the nfw CLI is installed in a workspace with multiple services, **When** I run `nfw gen entity Customer --props Email:string --service MyService --no-input`, **Then** the schema file is created in the specified service's entity specs directory and the template engine generates code with the `Email` property.

3. **Given** the nfw CLI is installed in a workspace with exactly one service, **When** I run `nfw gen entity Order --props OrderDate:datetime --no-input` without specifying a service, **Then** the command automatically selects the single available service and creates the schema file.

4. **Given** the nfw CLI is installed and I have specified an invalid property type, **When** I run `nfw gen entity Item --props Count:InvalidType --no-input`, **Then** the command fails with a clear error message indicating the invalid type and suggesting valid type options.

5. **Given** the nfw CLI is installed in a workspace with a service that uses custom ID types, **When** I run `nfw gen entity Category --props Name:string --id-type uuid --no-input`, **Then** the schema file is created with `uuid` as the ID type and the template engine is invoked.

6. **Given** the nfw CLI is installed in a workspace with a service that has NOT added the persistence module, **When** I run `nfw gen entity Product --props Name:string --no-input`, **Then** the command fails with a clear error message stating that the persistence module is required and providing the command to add it: `nfw add persistence --service <ServiceName>`.

7. **Given** the nfw CLI is installed in a workspace, **When** I run `nfw gen entity Product --props Name:string,Price:decimal --schema-only --no-input`, **Then** a schema file `specs/entities/Product.yaml` is created in the target service directory with the entity definition but the template engine is NOT invoked.

8. **Given** the nfw CLI is installed in a workspace with an existing schema file `specs/entities/Product.yaml`, **When** I run `nfw gen entity Product --from-schema --no-input`, **Then** the schema file is read and the template engine is invoked with the schema parameters.

---

### User Story 2 - Generate Entity Schema File (Priority: P2)

As a developer, I want entity definitions stored in schema files so that I can maintain language-agnostic entity definitions and support polyglot code generation across different programming languages.

**Why this priority**: Schema-first development enables cross-language consistency, supports future CRUD update workflows, and provides a single source of truth for entity structure independent of any specific programming language.

**Independent Test**: Can be tested independently by verifying that the command creates valid YAML schema files with the correct structure (entity name, ID type, properties) that can be used for code generation. The test delivers value by ensuring developers have a reliable schema-driven workflow.

**Acceptance Scenarios**:

1. **Given** the nfw CLI is installed in a workspace, **When** I run `nfw gen entity Product --props Name:string,Price:decimal --no-input`, **Then** a schema file `Product.yaml` is created in the configured entity specs directory containing the entity definition with all properties, ID type, and metadata.

2. **Given** the nfw CLI is installed in a workspace with an existing schema file, **When** I run `nfw gen entity Product --from-schema --no-input`, **Then** the entity code is generated from the schema file definition without requiring CLI property arguments, using the general types defined in the schema.

---

### Edge Cases

- **No services in workspace**: When the command is run in a workspace with no services, the command must fail with a clear error message instructing the user to add a service first.

- **Service not found**: When the specified service name does not exist in the workspace, the command must fail with an actionable error message containing the invalid service name.

- **Missing persistence module**: When the target service does not have the persistence module added (via `nfw add persistence`), the command must fail with a clear error message explaining that entity generation requires the persistence module, and provide the exact command to add it (e.g., `nfw add persistence --service <ServiceName>`).

- **Invalid property syntax**: When property definitions are malformed (missing type, invalid separators, unrecognized format), the command must fail with a specific error about the syntax issue and show the expected format.

- **Invalid property type**: When a property type is not a supported CLI input type, the command must fail with a list of valid type options.

- **Empty properties list**: When `--props` is provided but empty or contains only whitespace, the command must fail with a clear error indicating that at least one property must be defined.

- **Duplicate property names**: When the same property name is specified multiple times, the command must fail with an error identifying the duplicate.

- **Reserved property names**: When a property name conflicts with reserved names (e.g., `Id`, `CreatedAt`, `UpdatedAt` if auto-generated), the command must warn or fail based on configuration.

- **Non-primitive property type**: When a property type is specified that is not a supported primitive type, the command must fail with a list of valid primitive type options.

- **Entity already exists**: When an entity with the same name already exists in the target output directory, the command must fail with a clear error and suggest using a different name or explicitly confirming overwrite (if supported).

- **Permission errors**: When the user lacks write permissions for the entity specs directory, the command must fail with a clear permission error message without making partial changes.

- **Interrupt signal (Ctrl+C)**: When the user interrupts the command during execution, any partial files must be cleaned up to prevent corrupted state.

- **Invalid identifier**: When the entity name contains invalid characters, the command must fail with a clear error about naming requirements.

- **Invalid ID type**: When `--id-type` specifies a type that is not a supported general type (e.g., `integer`, `uuid`, `string`), the command must fail with a list of valid ID type options.

- **Invalid schema file**: When `--from-schema` is specified but the schema file doesn't exist or has invalid YAML syntax, the command must fail with a specific error about the schema file issue.

- **Invalid general type in schema**: When a schema file contains a property with an unsupported general type (not one of: `string`, `integer`, `decimal`, `boolean`, `datetime`, `uuid`, `bytes`), the command must fail with a specific error listing valid general type options.

- **Schema file conflict**: When an entity is generated via CLI args but a schema file already exists with different properties, the command must warn about the discrepancy and ask whether to update the schema or use the existing schema.

- **Schema directory missing**: When schema file creation is requested but the configured entity specs directory doesn't exist, the command must create the directory automatically.

- **Invalid schema path in nfw.yaml**: When the configured `entitySpecsPath` in `nfw.yaml` points to an invalid or inaccessible location, the command must fail with a clear error about the configuration issue.

- **Schema path not a directory**: When the configured `entitySpecsPath` exists but is a file rather than a directory, the command must fail with a clear error indicating the path must be a directory.

- **Template execution failure**: When the template engine fails to execute or returns an error, the command must fail with a clear error message indicating the template execution failure.

---

## Requirements

### Functional Requirements

#### CLI Responsibilities (Schema Management)

- **FR-001**: The command MUST create entity schema files in the configured entity specs directory for the target service.

- **FR-002**: The command MUST support the `--props <DEFINITIONS>` parameter where each definition follows the format `PropertyName:Type`.

- **FR-003**: The command MUST support the `--service <NAME>` parameter to specify the target service by name.

- **FR-004**: The command MUST support the `--id-type <TYPE>` parameter to specify a custom ID type using general types (default: `integer`).

- **FR-005**: The command MUST support the `--no-input` flag to enable automated, non-interactive execution.

- **FR-006**: The command MUST automatically select the single available service when `--no-input` is used and only one service exists in the workspace.

- **FR-008**: The command MUST accept property type arguments using familiar type syntax and map these to language-agnostic general types in the schema file. The specific language type mappings and database type mappings are determined by the entity template in `nfw-templates`.

- **FR-010**: The command MUST support only primitive property types as specified in FR-008 and FR-026.

- **FR-011**: The command MUST validate that all specified property types are supported primitive types and reject any non-primitive types with a clear error message listing valid primitive type options.

- **FR-013**: The command MUST complete execution within 3 seconds for typical entity generation with standard property counts.

- **FR-014**: The command MUST validate property syntax and provide clear error messages for malformed definitions.

- **FR-015**: The command MUST validate that the specified service exists in the workspace before attempting entity generation.

- **FR-015-A**: The command MUST validate that the persistence module has been added to the target service (via `nfw add persistence`) before allowing entity generation, checking the service's module configuration in `nfw.yaml`.

- **FR-015-B**: When the persistence module is not present in the target service, the command MUST fail with a clear error message instructing the user to run `nfw add persistence --service <ServiceName>` first, and provide the exact command to run.

- **FR-016**: The command MUST fail with a clear error message when no services are found in the workspace.

- **FR-017**: The command MUST detect and prevent duplicate property names within a single entity definition.

- **FR-018**: The command MUST invoke the template engine with entity parameters for code generation.

- **FR-020**: The command MUST support interactive prompts for service selection when no service is specified and multiple services exist.

- **FR-021**: The command MUST validate entity names against general identifier naming rules (alphanumeric, no leading numbers, no special characters).

- **FR-022**: The command MUST validate ID types against supported general types (`integer`, `uuid`, `string`, etc.).

- **FR-023**: The command MUST include integration tests covering successful generation, error scenarios, and edge cases.

- **FR-024**: The command MUST preserve existing schema files and never overwrite without explicit confirmation.

- **FR-026**: The command MUST accept property type arguments using familiar type syntax for CLI input (e.g., `string`, `int`, `decimal`, `bool`, `datetime`, `uuid`, `bytes`). These CLI input types are mapped to language-agnostic general types in the schema file.

- **FR-027**: The command MUST support nullable type syntax (e.g., `PropertyName:Type?`) for optional properties.

- **FR-028**: The command MUST create or update a schema file (e.g., `<EntityName>.yaml`) in the configured entity specs directory for the target service, storing the entity definition in a language-agnostic format.

- **FR-028-A**: The command MUST read the entity specs directory path from the service configuration in `nfw.yaml` (e.g., `services.<serviceName>.entity-specs-path`), defaulting to `specs/entities/` relative to the service root if not configured.

- **FR-029**: The command MUST support both quick-start mode (CLI arguments → schema + invoke template) and schema-first mode (create schema file → invoke template from schema).

- **FR-030**: The command MUST emit structured logs to stdout during execution including generation steps, warnings, schema file creation, and final result for debugging and audit purposes.

- **FR-031**: When `--schema-only` flag is provided, the command MUST create the schema file in the configured entity specs directory without invoking the template engine.

- **FR-032**: The schema file format MUST be YAML with standard structure including entity name, ID type, property list (name, general type, nullable), and entity type selection. Property types in the schema MUST be language-agnostic general types (e.g., `string`, `integer`, `decimal`, `boolean`, `datetime`, `uuid`, `bytes`).

#### Template Responsibilities (Code Generation)

- **FR-T001**: Templates MUST determine the target programming language for code generation.

- **FR-T002**: Templates MUST determine the output directory and file structure for generated code.

- **FR-T003**: Templates MUST map general types from the schema to language-specific types for the target programming language.

- **FR-T004**: Templates MUST determine base class selection based on the `entityType` field in the schema (valid values: `entity`, `auditable-entity`, `soft-deletable-entity`).

- **FR-T005**: Templates MAY add property validation attributes based on property types and template-specific rules.

- **FR-T006**: Templates MUST ensure generated code is compatible with Native AOT compilation (no reflection-heavy patterns).

- **FR-T007**: Templates MUST determine file naming conventions and namespace structure for generated code.

### Key Entities

- **AddEntityCommand**: A command model containing entity name, property definitions, ID type, service name, and workspace context.

- **PropertyDefinition**: A value object representing a single property definition with name, type, and nullable flag.

- **EntityGenerationParameters**: A value object containing template rendering parameters including entity name, namespace, ID type, properties, and service information.

- **AddEntityCommandHandler**: A command handler that orchestrates entity generation including validation, template rendering, and file creation.

- **AddEntityCliCommand**: A CLI command adapter that handles user interaction, parameter parsing, and service selection, delegating core logic to the command handler.

- **EntityTypeResolver**: A service responsible for validating that property types are supported primitive types.

- **EntityNameValidator**: A service responsible for validating entity names against C# identifier rules and reserved names.

- **PropertySyntaxParser**: A service responsible for parsing property definitions from string format into structured data.

- **EntitySchemaGenerator**: A service responsible for creating and updating YAML schema files from entity definitions.

- **EntitySchemaReader**: A service responsible for reading and parsing entity schema files for code generation.

- **ServiceModuleValidator**: A service responsible for validating that required service modules (specifically the persistence module) have been added to the target service before allowing entity generation.

- **EntityGenerationError**: An error type representing various failure scenarios including validation errors, file system errors, schema errors, module dependency errors, and template execution failures.

---

## Success Criteria

### Measurable Outcomes

- **SC-001**: The command MUST successfully generate valid entity classes for at least 95% of valid input combinations without errors.

- **SC-002**: The command MUST complete execution within 3 seconds for typical entity generation workloads on standard developer hardware.

- **SC-003**: Generated entity classes MUST compile successfully without manual modifications for at least 98% of test cases.

- **SC-004**: Integration tests MUST cover at least 90% of code paths including successful generation, all error scenarios, service selection, and ID type variations.

- **SC-005**: The command MUST provide clear, actionable error messages for 100% of failure scenarios with error types mapped to appropriate exit codes.

- **SC-005-A**: The command MUST correctly detect when the persistence module is missing from the target service and provide actionable error messages 100% of the time, including the exact command to add the persistence module.

- **SC-006**: Generated entity code MUST be Native AOT compatible with zero AOT warnings for standard entity patterns.

- **SC-007**: The command MUST correctly validate primitive property types with 100% accuracy, rejecting any non-primitive types with clear error messages.

- **SC-008**: Property validation MUST correctly identify and report invalid syntax with at least 98% accuracy for common error patterns.

- **SC-009**: The command MUST support all specified primitive types and generate correctly typed properties with 100% accuracy.

- **SC-010**: Schema files MUST be created or updated successfully for at least 98% of entity generation operations, with valid YAML structure using language-agnostic general types.

- **SC-011**: The hybrid workflow MUST allow developers to use CLI arguments for quick-start OR create/edit schema files manually for advanced scenarios.

---

## Assumptions

- The workspace has a valid `nfw.yaml` configuration file with at least one service defined.

- Template sources are configured and accessible, with entity templates available in the configured template repository (`nfw-templates`).

- The user has write permissions for the entity specs directory of the target service.

- The target service has the persistence module added via `nfw add persistence` before entity generation is attempted, as entity generation depends on persistence infrastructure being in place.

- Entity templates provide various entity type choices (entity, auditable-entity, soft-deletable-entity) through template configuration, with templates handling the actual inheritance and framework integration.

- Entity properties are limited to primitive types only; complex types and value objects are not supported.

- The template engine is properly configured and can resolve entity templates and execute template rendering.

- Schema files (YAML) are stored in a configurable directory path specified in `nfw.yaml` under the service configuration (e.g., `services.<serviceName>.entity-specs-path`), defaulting to `specs/entities/` relative to the service root if not configured.

- The `entity-specs-path` configuration can be an absolute path or a relative path from the service root, providing flexibility for workspace organization.

- Schema property types are language-agnostic general types (e.g., `string`, `integer`, `decimal`, `boolean`, `datetime`, `uuid`, `bytes`). CLI arguments use familiar type syntax which is mapped to general types in the schema.

- Templates are responsible for determining: target programming language, output directory structure, language-specific type mappings, base class selection (based on `entityType` field), validation attributes, and file naming conventions.

- Templates in `nfw-templates` map general types from the schema to both language-specific types for the target programming language (e.g., `string` → C# `string`, Go `string`, Rust `String`) and database-specific types for the persistence layer.

- Generated code uses a "safe zone" approach (e.g., `.g.cs` partial files for C#) to enable schema-driven regeneration without overwriting user code. The specific file extension and pattern is determined by the template.

- The hybrid approach allows quick-start development (CLI args only) while supporting schema-first workflows for polyglot scenarios and future CRUD update operations.

- Identifier naming rules follow general programming conventions (alphanumeric, no leading numbers, no reserved keywords).

- Native AOT compatibility requires avoiding reflection-heavy patterns and using compile-time code generation. Templates ensure generated code meets these requirements.

- ID types are constrained to general types that can serve as entity identifiers (`integer`, `uuid`, `string`, etc.).

- Entity generation is a one-time operation; subsequent modifications to entity properties are done manually or through separate commands.

- Nullable properties use the nullable type syntax (Type?) and are handled appropriately in generated code based on template rules.

---

## Dependencies

- Template engine must be configured with entity templates, providing various base class choices and language-specific code generation capabilities.

- Workspace configuration (`nfw.yaml`) must define services with their module configuration and entity specs path.

- CLI abstraction layer must provide consistent command handling and error reporting.

- File system services must handle atomic file creation and cleanup on failure.

- YAML parsing libraries must be available for schema file generation and reading.

- The `nfw.yaml` configuration file must support an optional `entity-specs-path` property per service for configuring where entity schema files are stored.

- Schema file storage locations must be accessible and writable within the workspace, with automatic directory creation if the configured path doesn't exist.

- Schema files use a standard structure that is language-agnostic and suitable for polyglot code generation. Example schema structure:

  ```yaml
  entity: Product
  idType: uuid  # general type
  entityType: entity  # entity, auditable-entity, or soft-deletable-entity
  properties:
    - name: Name
      type: string  # general type
      nullable: false
    - name: Price
      type: decimal  # general type
      nullable: false
  ```

- The target service MUST have the persistence module added via `nfw add persistence` before entity generation can proceed, as entity generation depends on persistence infrastructure being in place.

- Templates in `nfw-templates` repository handle language-specific decisions including: target language selection, output directory structure, namespace/package organization, type mappings, and validation attributes.

---

## Clarifications

### Session 2026-04-30

- Q: What observability signals should the command provide for debugging and audit trails? → A: Emit structured logs to stdout during execution including generation steps, warnings, and final result.

- Q: What types should entity properties support and is there a maximum property limit? → A: Entity properties MUST be primitive types only (no complex types, nested collections, or value objects). There is NO maximum property limit.

- Q: What base entity classes should generated entities inherit from? → A: Base class selection is a template concern. The CLI provides entity parameters (name, properties, ID type) to templates, and templates determine which base class to use based on template configuration and options. Templates may offer choices like standard entity, auditable entity, or soft-deletable entity.

- Q: Should the command support code-first (CLI args), schema-first (YAML file), or both? → A: Hybrid approach - support both workflows. CLI arguments create both the schema file and generated code; developers can also create/edit schema files manually for polyglot scenarios or schema-first workflows.

- Q: Where should entity schema files be stored? → A: The entity specs directory path should be configurable in `nfw.yaml` under the service configuration (e.g., `services.<serviceName>.entitySpecsPath`), defaulting to `<service_path>/specs/entities/` if not configured. This provides a logical default while allowing per-service customization.

- Q: How should property types be represented in schema files vs CLI arguments? → A: CLI arguments use C#-like primitive type syntax for developer convenience (e.g., `Name:string`, `Price:decimal`). The schema file stores language-agnostic general types (e.g., `name: {type: string}`, `price: {type: decimal}`). Templates then map general types to both language-specific types and database-specific types for the target programming language and persistence layer.

- Q: Why was the command renamed from `nfw add entity` to `nfw gen entity`? → A: The `gen` prefix better aligns with NFramework's code generation focus and distinguishes generation commands from module addition commands. It also groups related generation commands (`gen entity`, `gen command`, `gen query`) under a consistent namespace.

- Q: Why does entity generation require the persistence module? → A: Entity generation depends on persistence infrastructure (DbContext, repositories, base classes) being in place. The persistence module (`nfw add persistence`) sets up this infrastructure, so entities can only be generated after it has been added to the target service.

---

- Q: Should the command support generating entity interfaces alongside concrete classes? → A: No, entity interfaces are out of scope for this command. Entities are concrete domain classes.

- Q: Should the command support generating entity configuration (e.g., EF Core configuration) alongside the entity class? → A: No, entity configuration is handled separately by persistence-related commands or templates.

- Q: Should the command support generating domain events alongside entities? → A: No, domain event generation is a separate concern and should be handled by a dedicated command if needed.

- Q: Should the command support generating collection navigation properties for related entities? → A: Collection types (e.g., `List<T>`) are not supported for entity properties. Only primitive types are allowed.

- Q: Should the command support generating enums for properties that have a limited set of values? → A: No, enum generation is a separate concern. Entity properties are limited to primitive types only.

- Q: Should the command support generating entity constructors with required parameters? → A: Entity constructor generation is determined by the entity template and framework conventions. The command provides property definitions; the template handles constructor patterns.

- Q: Should validation attributes be configurable or follow fixed rules? → A: Validation attributes are determined by entity templates. The CLI provides property type information to templates, and templates decide which validation attributes (if any) to include based on template-specific rules and conventions.

---

## Non-Goals

- Generating entity interfaces or abstract base classes beyond framework-provided ones.

- Generating entity configuration for specific persistence technologies (EF Core, etc.).

- Generating domain events, event handlers, or event sourcing infrastructure.

- Automatic discovery or generation of entity relationships beyond explicitly defined properties.

- Generating enums, DTOs, or mappers alongside entities (handled by other commands).

- Generating commands, queries, or handlers alongside entities (handled by `nfw gen command` and `nfw gen query`, which depend on the mediator module, not on entities).

- Modifying or updating existing entity classes (this command only creates new entities).

- Generating database schema, migrations, or persistence-layer code.

- Cross-language entity generation (e.g., generating Rust or Go entities from .NET entity definitions).

- Generating entities with complex inheritance hierarchies beyond single base class inheritance.

- Generating entities with backing fields, property change notification, or UI-specific features.

- Automatically adding the persistence module to services; users must explicitly run `nfw add persistence` before entity generation.

- Diagnosing or fixing issues with the persistence module installation; the command only validates its presence and provides clear error messages when absent.

- Schema file validation beyond basic YAML syntax checking (business rule validation is out of scope).

- Automatic schema synchronization across multiple services (cross-service schema updates are out of scope).

- Schema file migration or versioning (schema files are recreated on update, not migrated).

- Advanced schema features like relationships, indexes, or constraints (persistence-level concerns handled separately).
