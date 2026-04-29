# CLI Contract: nfw add persistence

**Feature**: 009-add-persistence-command
**Date**: 2026-04-29

## Command Signature

```text
nfw add persistence [--service <NAME>] [--no-input]
```

## Command Specification

### Name

`persistence` (verb: `add`, subcommand: `persistence`)

### Description

Add the Persistence module to an existing service, generating DbContext, repository base classes, and database configuration artifacts.

### Options

| Option | Type | Required | Default | Description |
|--------|------|----------|---------|-------------|
| `--service <NAME>` | string | No* | - | Name of the service to add persistence to |
| `--no-input` | flag | No | false | Disable all interactive prompts |

*Required when not running in interactive mode with a single service.

### Arguments

None (all parameters passed via options)

## Behavior

### Input Modes

**Interactive Mode** (default):

1. If `--service` not specified and multiple services exist:
   - Present interactive prompt: "Select a service to add persistence to:"
   - Display available services as numbered options
   - User selects service by number
   - Proceed with addition

2. If `--service` not specified and exactly one service exists:
   - Auto-select the single service
   - Proceed with addition (no prompt)

3. If `--service` specified:
   - Validate service exists
   - Proceed with addition

**Automated Mode** (`--no-input`):

1. If `--service` specified:
   - Validate service exists
   - Proceed with addition
2. If no `--service` and exactly one service:
   - Auto-select and proceed
3. If no `--service` and multiple services:
   - Error: "Multiple services found. Please specify --service or run without --no-input"

### Execution Flow

```text
1. Validate workspace (nfw.yaml exists)
2. Parse services from nfw.yaml
3. Validate target service exists
4. Check for existing persistence module
    └─ If present: Report and exit (success)
5. Load persistence template context
6. Execute template rendering
    ├── Render DbContext.cs
    ├── Render RepositoryBase.cs
    └── Render configuration files
7. Update nfw.yaml (add "persistence" to service's modules array)
8. Report success
```

### Rollback Behavior

If template execution fails:

- NO changes to nfw.yaml
- Partial template files MAY exist (not cleaned up)
- Error message indicates failure reason
- Exit code reflects error type

## Exit Codes

| Code | Condition |
|------|-----------|
| 0 | Success (persistence added or already present) |
| 1 | Generic error |
| 2 | Workspace error (no nfw.yaml, invalid structure) |
| 3 | Template not found or execution failed |
| 4 | Service not found |
| 5 | Permission error (cannot write files) |
| 130 | SIGINT (Ctrl+C) |

## Output Format

### Standard Output (stdout)

**Success**:

```text
✓ Adding persistence module to 'MyService'...

Successfully added Persistence module to 'MyService'.

Generated artifacts:
   - src/MyService/Infrastructure/Persistence/MyServiceDbContext.cs
   - src/MyService/Application/Persistence/RepositoryBase.cs
   - src/MyService/appsettings.json

Next steps:
   1. Update connection string in appsettings.json
   2. Run 'dotnet ef migrations add InitialCreate' to create initial migration
   3. Run 'dotnet ef database update' to apply migrations
```

**Already Present**:

```text
ℹ Persistence module is already present in 'MyService'
```

### Error Output (stderr)

**Service not found**:

```text
Error: Service 'NonExistent' not found in workspace.

Available services:
   - MyService
   - AnotherService
```

**Template not found**:

```text
Error: Template not found: Could not resolve template 'dotnet-service/persistence'

Ensure the persistence template is available in your configured template sources.
```

**Permission error**:

```text
Error: Permission denied: cannot write to /path/to/nfw.yaml

Check file permissions and try again.
```

## Examples

### Basic usage (interactive prompt)

```bash
$ nfw add persistence
? Select a service to add persistence to:
  ● MyService
  ○ AnotherService

✓ Adding persistence module to 'MyService'...
Successfully added Persistence module to 'MyService'
```

### Specify service explicitly

```bash
$ nfw add persistence --service MyService
✓ Adding persistence module to 'MyService'...
Successfully added Persistence module to 'MyService'
```

### Automated mode

```bash
$ nfw add persistence --service MyService --no-input
# No prompts, executes directly
# Exit code 0 on success, non-zero on failure
```

### Single service (auto-select)

```bash
$ nfw add persistence --no-input
# Auto-selects the only service
# No prompts needed
```

### Error cases

```bash
# Service not found
$ nfw add persistence --service NotFound
Error: Service 'NotFound' not found in workspace

# Multiple services without --service in --no-input mode
$ nfw add persistence --no-input
Error: Multiple services found. Please specify --service or run without --no-input
```

## Preconditions

1. **Workspace exists**: Command must be run within a valid nfw workspace (nfw.yaml present)
2. **Service exists**: Target service must be defined in nfw.yaml
3. **Template available**: Persistence template must be accessible
4. **Write permissions**: User must have write access to nfw.yaml and service directory

## Postconditions

**On Success**:

1. Service's `modules` array in nfw.yaml contains `"persistence"`
2. Template artifacts generated in service directory
3. YAML comments preserved
4. All generated code compiles (if templates are correct)

**On Failure**:

1. nfw.yaml unchanged (atomic operation)
2. Partial template files may exist (not cleaned up)
3. Error message explains failure reason
4. Appropriate exit code returned

## Edge Case Behavior

### Service already has persistence module

- **Detection**: Check service's modules array before execution
- **Action**: Report info message, exit with code 0
- **No template execution**
- **No YAML modification**

### Empty services list

- **Error**: "No services found in workspace. Add a service first."
- **Exit code**: 2

### Invalid service name

- **Error**: "Service 'Invalid Name' not found in workspace."
- **Exit code**: 4

### Template execution fails

- **Action**: Rollback YAML changes (no write)
- **Error**: Specific failure reason from template engine
- **Exit code**: 3

### Concurrent modifications

- **Behavior**: Last-write-wins on nfw.yaml
- **Note**: Acceptable for CLI tool (users typically don't run multiple add commands simultaneously)

### Interrupt signal (Ctrl+C)

- **Action**: Terminate immediately
- **State**: Partial changes may exist (no rollback on SIGINT)
- **Exit code**: 130

## Configuration

The command reads configuration from:

- `nfw.yaml`: Workspace and service definitions
- `nfw.yaml → template_sources`: Template repository locations
- `nfw.yaml → services → <service> → template`: Service template ID

The command writes configuration to:

- `nfw.yaml → services → <service> → modules`: Adds `"persistence"` entry

## Dependencies

**External Dependencies**:

- `nfw.yaml` (workspace configuration)
- Template sources (local or remote)
- Service directory (file system)

**Internal Dependencies**:

- `ArtifactGenerationService`: Core workflow orchestration
- `TemplateEngine`: Template execution
- `WorkingDirectoryProvider`: File system operations
- `TemplateRootResolver`: Template resolution
- `InteractivePrompt`: User interaction (if not --no-input)

## Performance Requirements

- **Total execution time**: <5 seconds for typical workspaces
- **Rollback time**: <1 second on template failure
- **Workspace size**: Support up to 10 services

## Security Considerations

**File Operations**:

- Only writes to workspace directory (no arbitrary file access)
- Respects file system permissions
- Does not execute arbitrary code

**Input Validation**:

- Service names validated against regex: `^[a-zA-Z0-9_-]+$`
- Template IDs validated before use
- No code injection from user input

**YAML Safety**:

- Parses YAML with size limits
- Prevents YAML entity expansion attacks
- Comments are preserved, not interpreted

## Testing Requirements

Integration tests must verify:

1. ✅ Successful addition with valid inputs
2. ✅ Rollback on template execution failure
3. ✅ Service not found error handling
4. ✅ YAML comment preservation
5. ✅ Duplicate module detection
6. ✅ Interactive prompt behavior
7. ✅ Automated mode (--no-input) behavior
8. ✅ Auto-selection for single service
9. ✅ Multiple services error without --service
10. ✅ Exit code mapping for all error types

## Versioning

**Command Version**: 1.0.0
**Minimum nfw Version**: Compatible with current CLI architecture
**Breaking Changes**: None (initial release)
