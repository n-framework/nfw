# Research: nfw add persistence Command

**Date**: 2026-04-29
**Feature**: 009-add-persistence-command

## Overview

The `nfw add persistence` command follows the established architectural pattern from `008-add-mediator-command`. This research confirms the implementation approach, identifies all required components, and validates technical decisions.

## Research Findings

### 1. Implementation Pattern Analysis

**Decision**: Follow the exact same architectural pattern as `nfw add mediator` command.

**Rationale**:

- The mediator command (spec 008) is already implemented and tested
- ArtifactGenerationService provides all necessary core functionality
- AddArtifactError type covers all error scenarios
- Integration test pattern is proven and reliable

**Implementation Components**:

```Application Layer:
├── commands/add_persistence/
│   ├── add_persistence_command.rs         # Command DTO (ServiceInfo + WorkspaceContext)
│   └── add_persistence_command_handler.rs # Handler using ArtifactGenerationService

Presentation Layer:
├── commands/add/persistence/
│   ├── mod.rs                               # Module export
│   ├── registration.rs                      # CliCommandSpec registration
│   └── handler.rs                           # CLI adapter with user interaction

Integration Tests:
└── features/module/persistence_add_test.rs
```

### 2. ArtifactGenerationService Interface

**Key Methods Used**:

- `get_workspace_context()` → Loads nfw.yaml with preserved comments
- `extract_services()` → Parses service definitions from nfw.yaml
- `load_generator_context()` → Resolves generator root and loads nfw.generator.yaml
- `execute_generation()` → Executes generator steps
- `add_service_module()` → Atomically adds module to service's modules array

**Rollback Behavior**:

- nfw.yaml is only modified AFTER successful generator execution
- If generator execution fails, no YAML changes are made
- This ensures atomic operation as required by FR-010

### 3. AddArtifactError Type

**Error Types Available**:

- `InvalidIdentifier(String)` → Invalid service/module names
- `WorkspaceError(String)` → General workspace issues
- `ConfigError(String)` → Configuration problems
- `GeneratorNotFound(String)` → Generator resolution failures
- `MissingRequiredModule(String)` → Required module not installed
- `ExecutionFailed(Box<GeneratorError>)` → Generator execution errors
- `NfwYamlReadError/ParseError/WriteError(String)` → YAML handling errors

**Exit Code Mapping**: The CLI handler maps `AddArtifactError` to `ExitCodes` via `ExitCodes::from_add_artifact_error()`.

### 4. CLI Command Structure

**Registration** (`registration.rs`):

```rust
pub fn register() -> CliCommandSpec {
    CliCommandSpec::new("persistence")
        .with_about("Add persistence module to a service")
        .with_option(
            CliOptionSpec::new("service", "service")
                .with_help("Service name to add persistence to"),
        )
        .with_option(
            CliOptionSpec::new("no-input", "no-input")
                .with_help("Disable all interactive prompts")
                .flag(),
        )
}
```

**Handler** (`handler.rs`):

- Extends `AddMediatorCliCommand` pattern
- Uses `InteractivePrompt` trait for user interaction
- Implements `AddPersistenceRequest<'a>` struct for parameters
- Handles service selection logic (interactive vs automated)

### 5. Integration Test Pattern

**Test Setup** (from `mediator_add_test.rs`):

- Uses sandbox directories with `support::create_sandbox_directory()`
- Creates fake nfw.yaml, generator files, and service directories
- Tests run sequentially due to `std::env::current_dir()` mutation
- Uses `Mutex<()>` lock for serialization

**Test Cases Required**:

1. ✅ Successful addition updates nfw.yaml and renders generators
2. ✅ Rollback nfw.yaml if generator execution fails
3. ✅ Fails if service not found
4. ✅ Preserves YAML comments
5. ✅ Detects existing persistence module

### 6. Persistence Generator Context

**Generator Location**: `src/nfw-generators/src/dotnet-service/persistence/`

**Expected Generator Structure** (to be created separately):

```persistence/
├── generator.yaml     # Generator configuration with steps
├── DbContext.cs.tera  # DbContext generation generator
├── RepositoryBase.cs.tera  # Repository base class generator
└── appsettings.json.tera   # Configuration generator
```

**Generator Parameters**:

- `Name`: Service name
- `Namespace`: Workspace namespace
- `Service`: Service name
- Connection string placeholders

**Generator Requirements** (from generator.yaml spec):

- `id: dotnet-service/persistence`
- `required_modules: []` (no dependencies for now)
- Steps render DbContext, repository base classes, and configuration

### 7. Duplicate Detection Logic

**Current Implementation in ArtifactGenerationService**:

- `add_service_module()` checks if module already exists before adding
- `validate_required_modules()` checks if required modules are present

**Required Enhancement**:
The command handler should check for existing "persistence" module BEFORE calling `execute_generation()` to avoid:

- Redundant generator execution
- Duplicate module entries
- Confusing user experience

**Implementation**: Add check in CLI handler before calling command handler:

```rust
let modules = handler.get_service_modules(&workspace_context, &selected_service.path)?;
if modules.contains(&"persistence".to_string()) {
    // Report already present and return early
}
```

## Technical Decisions

### Decision 1: Command Name

**Choice**: `persistence` (not `add-persistence` or `persistence-module`)

**Rationale**: Consistent with existing `mediator` command naming convention.

### Decision 2: Generator Generator Type

**Choice**: `"persistence"` (passed to `load_generator_context()`)

**Rationale**: Matches generator directory name and provides clear identification.

### Decision 3: Module Name in nfw.yaml

**Choice**: `"persistence"` (the string added to modules array)

**Rationale**: Simple, clear, consistent with command name.

### Decision 4: Error Handling for Existing Persistence

**Choice**: Detect and report early, before generator execution

**Rationale**:

- Avoids unnecessary work
- Prevents duplicate entries
- Better user experience (fast feedback)

### Decision 5: Generator Parameters

**Choice**: Standard set (Name, Namespace, Service) without custom parameters

**Rationale**:

- Matches mediator command pattern
- Generators can derive all needed values
- Simpler implementation
- Extensible via GeneratorParameters if needed later

## Open Questions Resolved

### Q: Should generators be created as part of this implementation?

**A**: No. The spec is for the CLI command only. Generator creation is a separate concern tracked by the nfw-generators repository. The command assumes generators exist and provides clear errors if they don't.

### Q: Should the command add EF Core package references?

**A**: No. Package management is explicitly out of scope (non-goals). Generators should handle this if needed, or developers add packages manually.

### Q: Should there be a `--provider` flag for database selection?

**A**: No. The command is generator-agnostic. Database provider selection is handled by the generator system based on service generator configuration.

### Q: Should the command validate EF Core version compatibility?

**A**: Yes, but only at a basic level. The command checks if the persistence module exists (indicating EF Core is likely present). Deep version validation is out of scope.

### Q: How are concurrent modifications to nfw.yaml prevented?

**A**: File locking is NOT implemented. The YAML operations are atomic at write time. Concurrent modifications would result in last-write-wins behavior. This is acceptable for a CLI tool (users typically don't run multiple add commands simultaneously).

## Alternatives Considered

### Alternative 1: Create generators as part of this spec

**Rejected Because**:

- Generators are separate concerns
- nfw-generators is its own repository
- CLI command should work independently of generator implementation details
- Generators may evolve independently

### Alternative 2: Add `--force` flag to overwrite existing persistence

**Rejected Because**:

- Adds complexity
- Risk of data loss
- Users can manually remove and re-add if needed
- Explicit detection is safer

### Alternative 3: Support batch operations (multiple services)

**Rejected Because**:

- Explicitly out of scope (non-goals)
- Adds significant complexity
- Single service is sufficient for current needs
- Can be added later if needed

## Implementation Readiness

✅ **All technical context clarified**
✅ **No NEEDS CLARIFICATION items remaining**
✅ **Implementation pattern confirmed**
✅ **All dependencies identified**
✅ **Testing approach validated**

**Ready for Phase 1**: Design & Contracts
