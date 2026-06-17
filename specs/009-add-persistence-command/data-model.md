# Data Model: nfw add persistence Command

**Feature**: 009-add-persistence-command
**Date**: 2026-04-29

## Core Entities

### AddPersistenceCommand

**Purpose**: Command model representing a request to add the persistence module to a service.

**Fields**:

| Field               | Type               | Description                       |
| ------------------- | ------------------ | --------------------------------- |
| `service_info`      | `ServiceInfo`      | Target service information        |
| `workspace_context` | `WorkspaceContext` | Workspace configuration and state |

**Lifecycle**:

- Created by CLI handler based on user input
- Passed to `AddPersistenceCommandHandler::handle()`
- Consumed during execution (no persistence)

**Validation Rules**:

- Service must exist in workspace
- Service must have a valid generator configuration
- Workspace must have valid nfw.yaml

---

### AddPersistenceCommandHandler

**Purpose**: Application-layer handler that orchestrates persistence module addition.

**Dependencies**:

- `ArtifactGenerationService<W, R, E>` - Core workflow service
- `WorkingDirectoryProvider` - File system operations
- `GeneratorRootResolver` - Generator resolution
- `GeneratorEngine` - Generator execution

**Key Operations**:

| Method                    | Input                   | Output                                       | Description                           |
| ------------------------- | ----------------------- | -------------------------------------------- | ------------------------------------- |
| `handle()`                | `AddPersistenceCommand` | `Result<(), AddArtifactError>`               | Executes the add persistence workflow |
| `get_workspace_context()` | -                       | `Result<WorkspaceContext, AddArtifactError>` | Loads workspace configuration         |
| `extract_services()`      | `WorkspaceContext`      | `Result<Vec<ServiceInfo>, AddArtifactError>` | Lists available services              |

**Error Handling**:

- Returns `AddArtifactError` for all failure scenarios
- Errors map to exit codes via `ExitCodes::from_add_artifact_error()`

---

### AddPersistenceCliCommand

**Purpose**: Presentation-layer CLI adapter that handles user interaction.

**Generic Parameters**: `<W, R, E, P>` where:

- `W: WorkingDirectoryProvider`
- `R: GeneratorRootResolver`
- `E: GeneratorEngine`
- `P: InteractivePrompt + Logger`

**Input Structure**:

```rust
pub struct AddPersistenceRequest<'a> {
    pub no_input: bool,
    pub is_interactive_terminal: bool,
    pub service_name: Option<&'a str>,
}
```

**User Interaction Flows**:

1. **Automated mode** (`--no-input` + explicit service):
   - Validates service exists
   - Executes command directly
   - Reports success/failure

2. **Automated mode** (`--no-input`, single service):
   - Auto-selects the only service
   - Executes command
   - Reports success/failure

3. **Interactive mode** (no service specified, multiple services):
   - Presents service selection prompt
   - Executes command on selection
   - Reports success/failure

4. **Error cases**:
   - No services: Clear error message
   - Service not found: Error with invalid name
   - Module already present: Info message

---

### ServiceInfo

**Purpose**: Value object representing a service in the workspace.

**Fields**:

| Field          | Type     | Description                                   |
| -------------- | -------- | --------------------------------------------- |
| `name`         | `String` | Service identifier                            |
| `path`         | `String` | Relative path from workspace root             |
| `generator_id` | `String` | Generator identifier (e.g., "dotnet-service") |

**Source**: Parsed from `nfw.yaml` services mapping.

**Validation**: Name must match identifier regex: `^[a-zA-Z0-9_-]+$`

---

### WorkspaceContext

**Purpose**: Value object containing workspace state.

**Fields**:

| Field                | Type                | Description                     |
| -------------------- | ------------------- | ------------------------------- |
| `workspace_root`     | `PathBuf`           | Absolute path to workspace root |
| `nfw_yaml`           | `YamlValue`         | Parsed workspace configuration  |
| `preserved_comments` | `PreservedComments` | YAML comment metadata           |

**Lifecycle**:

- Loaded from current directory (searches up for nfw.yaml)
- Used throughout command execution
- Preserved for YAML comment restoration

**Invariants**:

- `workspace_root` must contain `nfw.yaml`
- `nfw.yaml` must be valid YAML with required fields
- `preserved_comments` must correspond to `nfw.yaml` content

---

### AddArtifactContext

**Purpose**: Execution context for generator generation.

**Fields**:

| Field                | Type                | Description                             |
| -------------------- | ------------------- | --------------------------------------- |
| `workspace_root`     | `PathBuf`           | Workspace root path                     |
| `nfw_yaml`           | `YamlValue`         | Workspace configuration                 |
| `preserved_comments` | `PreservedComments` | YAML comment metadata                   |
| `generator_root`     | `PathBuf`           | Path to persistence generator directory |
| `config`             | `GeneratorConfig`   | Parsed generator.yaml                   |
| `service_name`       | `String`            | Target service name                     |
| `service_path`       | `PathBuf`           | Service directory path                  |

**Creation**: Built by `ArtifactGenerationService::load_generator_context()`.

**Usage**: Passed to `GeneratorEngine::execute()` for rendering.

---

### GeneratorParameters

**Purpose**: Value object passed to generators for variable substitution.

**Standard Parameters** (always included):

| Parameter   | Type   | Source              |
| ----------- | ------ | ------------------- |
| `Name`      | String | Service name        |
| `Namespace` | String | Workspace namespace |
| `Service`   | String | Service name        |

**Optional Parameters** (via `--params` or generator config):

- Custom key-value pairs for generator-specific needs

**Builder Pattern**:

```rust
GeneratorParameters::new()
    .with_name("MyService")?
    .with_namespace("MyApp")?
    .with_service("MyService")?
```

---

## State Transitions

### Service Lifecycle

```┌─────────────────┐
│  Service exists │
│  without        │
│  persistence    │
└────────┬────────┘
          │
          │ nfw add persistence
          │ (success)
          ▼
┌─────────────────┐      ┌─────────────────┐
│  Service has    │◄─────┤  nfw add        │
│  persistence    │      │  persistence    │
│  module         │      │  (already       │
└─────────────────┘      │  present)       │
                          └─────────────────┘
                               ↓
                          Info message
                          (no-op)
```

### YAML Update Lifecycle

```┌──────────────────┐
│ Load nfw.yaml     │
│ + parse           │
│ + extract comments│
└────────┬──────────┘
          │
          │
          ▼
┌──────────────────┐
│ Execute generators │
│ (write files)     │
└────────┬──────────┘
          │
          │ success?
          ├───────┬────────┘
          │       │
         yes      no
          │       │
          ▼       ▼
┌──────────────┐  ┌──────────────┐
│ Add module to │  │ Rollback    │
│ nfw.yaml     │  │ (no changes) │
│ + restore    │  └──────────────┘
│   comments   │
└──────────────┘
```

---

## Error Hierarchy

```AddArtifactError
├── InvalidIdentifier          // Name validation failures
├── WorkspaceError             // Workspace-level issues
├── ConfigError                // Configuration problems
├── GeneratorNotFound           // Generator resolution failures
├── InvalidParameter           // Parameter validation errors
├── ExecutionFailed            // Generator execution errors
│   └── GeneratorError          // Underlying generator errors
├── MissingRequiredModule      // Dependency check failures
├── NfwYamlReadError           // File read errors
├── NfwYamlParseError          // YAML parsing errors
└── NfwYamlWriteError          // File write errors
```

**Error Propagation**:

- Generator errors wrapped in `ExecutionFailed`
- File system errors mapped to specific YAML errors
- All errors surface to user with actionable messages

---

## Relationships

```AddPersistenceCliCommand (presentation)
         │
         │ creates
         ▼
AddPersistenceRequest
         │
         │ passes to
         ▼
AddPersistenceCommandHandler (application)
         │
         │ uses
         ▼
ArtifactGenerationService (infrastructure)
         │
         │ creates
         ├──► AddPersistenceCommand
         └──► loads → WorkspaceContext
         └──► loads → ServiceInfo
         └──► loads → AddArtifactContext
         └──► builds → GeneratorParameters
         └──► modifies → nfw.yaml
```

---

## Validation Rules

### Service Name Validation

**Regex**: `^[a-zA-Z0-9_-]+$`

**Valid Examples**:

- `MyService`
- `my-service`
- `my_service`
- `Service123`

**Invalid Examples**:

- `my service` (space)
- `my.service` (dot)
- `my/service` (slash)

### Module Name Validation

**Allowed Values**: `["mediator", "persistence", ...]`

**Validation**: Module must exist as a generator in the configured generator sources.

### Namespace Validation

**Required**: `workspace.namespace` in nfw.yaml

**Format**: PascalCase .NET namespace (e.g., `MyApp.Services`)

---

## Data Flow

```User Input (CLI args)
         │
         ▼
AddPersistenceRequest
         │
         ├──► Service Selection Logic
         │     ├──► Explicit service
         │     ├──► Interactive prompt
         │     └──► Auto-select (single service)
         │
         ▼
AddPersistenceCommand
         │
         ├──► WorkspaceContext (load nfw.yaml)
         ├──► ServiceInfo (from services list)
         │
         ▼
AddPersistenceCommandHandler::handle()
         │
         ├──► load_generator_context("persistence")
         │     ├──► Resolve generator root
         │     ├──► Load generator.yaml
         │     └──► Validate generator config
         │
         ├──► build_parameters()
         │     ├──► Extract namespace
         │     ├──► Build GeneratorParameters
         │     └──► Validate identifiers
         │
         ├──► GeneratorEngine::execute()
         │     ├──► Render DbContext.cs.tera
         │     ├──► Render RepositoryBase.cs.tera
         │     └───► Render configuration generators
         │
         └──► add_service_module("persistence")
               ├──► Update nfw.yaml
               ├──► Restore comments
               └──► Write atomically
```

---

## Persistence Rules

### YAML Comment Preservation

**Rule**: All comments must be preserved in their original positions.

**Implementation**:

- Extract comments on read: `extract_preserved_comments()`
- Store in `WorkspaceContext.preserved_comments`
- Restore on write: `format_nfw_yaml_document()`

**Supported Comment Types**:

- Top-level comments (before documents)
- Section comments (before mappings/sequences)
- Inline comments (after values)
- Block comments (multi-line)

### Atomic Update

**Rule**: nfw.yaml is only modified after successful generator execution.

**Implementation**:

- Generator execution happens BEFORE YAML update
- If generator execution fails, YAML is never written
- Ensures consistent state

---

## Test Data Model

### Integration Test Fixtures

**Sandbox Workspace**:

```{sandbox}/
├── nfw.yaml           # Fake workspace config
├── generators/         # Fake generator sources
│   └── dotnet-service/
│       └── persistence/
│           ├── nfw.generator.yaml
│           ├── DbContext.cs.tera
│           └── ...
└── src/
    └── TestService/   # Fake service directory
```

**Test Scenarios**:

1. Valid workspace, valid service → Success
2. Generator source missing → GeneratorNotFound error
3. Invalid service name → WorkspaceError
4. Module already present → Early return with info
5. YAML with comments → Comments preserved
