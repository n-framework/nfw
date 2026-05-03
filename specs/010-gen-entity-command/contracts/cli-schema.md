# CLI Contract: nfw gen entity

**Command**: `nfw gen entity <NAME> --props <DEFINITIONS> [OPTIONS]`

## Command Signature

### Positional Arguments

| Argument | Type | Required | Description |
|----------|------|----------|-------------|
| `<NAME>` | string | Yes | Entity name in PascalCase (e.g., Product, Customer, Order) |

### Options

| Option | Type | Required | Default | Description |
|--------|------|----------|---------|-------------|
| `--props <DEFINITIONS>` | string | Yes* | - | Comma-separated property definitions: `Name:Type,Price:decimal` |
| `--service <NAME>` | string | No | Auto-select | Target service name |
| `--id-type <TYPE>` | string | No | integer | ID type: integer, uuid, string |
| `--entity-type <TYPE>` | string | No | entity | Entity type: entity, auditable-entity, soft-deletable-entity |
| `--schema-only` | flag | No | false | Create schema file without generating code |
| `--from-schema` | flag | No | false | Generate from existing schema file |
| `--no-input` | flag | No | false | Skip interactive prompts |
| `--help` | flag | No | false | Display help information |

*Required unless `--from-schema` is specified

## Property Definition Syntax

### Format

```text
PropertyName[:Type][?]
```

### Components

| Component | Description | Examples |
|-----------|-------------|----------|
| PropertyName | PascalCase identifier | Name, Email, Price |
| Type | C# primitive type | string, int, decimal, DateTime |
| ? | Nullable modifier | Makes property optional |

### Supported Types

| Type | Nullable Syntax | Description |
|------|----------------|-------------|
| string | string? | Text data |
| int | int? | 32-bit integer |
| long | long? | 64-bit integer |
| decimal | decimal? | Decimal number |
| double | double? | Double precision |
| float | float? | Single precision |
| bool | bool? | Boolean |
| DateTime | DateTime? | Date and time |
| DateTimeOffset | DateTimeOffset? | Date/time with offset |
| Guid | Guid? | Unique identifier |
| byte[] | byte[]? | Binary data |

### Examples

```bash
# Single property
--props Name:string

# Multiple properties
--props Name:string,Email:string?,Age:int,Price:decimal

# All required
--props Title:string,Description:string,Price:decimal,Stock:int

# Mixed nullable
--props Name:string,Email:string?,PhoneNumber:string?
```

## Command Modes

### Mode 1: Quick-Start (Default)

Generate entity code directly from CLI arguments.

```bash
nfw gen entity Product --props Name:string,Price:decimal --no-input
```

**Behavior**:

1. Validates persistence module is present
2. Parses property definitions
3. Creates schema file in `specs/entities/Product.yaml`
4. Invokes template engine
5. Generates entity code in Domain layer

### Mode 2: Schema-Only

Create schema file without generating code.

```bash
nfw gen entity Product --props Name:string,Price:decimal --schema-only --no-input
```

**Behavior**:

1. Validates persistence module is present
2. Parses property definitions
3. Creates schema file in `specs/entities/Product.yaml`
4. Skips template invocation and code generation

**Use Case**: Review or edit schema before code generation

### Mode 3: From-Schema

Generate entity code from existing schema file.

```bash
nfw gen entity Product --from-schema --no-input
```

**Behavior**:

1. Reads schema file from `specs/entities/Product.yaml`
2. Validates schema structure
3. Invokes template engine
4. Generates entity code in Domain layer

**Use Case**: Regenerate code after manual schema edits

## Exit Codes

| Code | Meaning | Example Trigger |
|------|---------|-----------------|
| 0 | Success | Entity generated successfully |
| 1 | General Error | Invalid input, missing dependencies |
| 130 | Interrupted | User pressed Ctrl+C |

## Output Format

### Success Output (stdout)

```text
[INFO] Generating entity: Product
[INFO] Target service: MyService
[INFO] Properties: Name (string), Price (decimal)
[INFO] ID type: int
[INFO] Creating schema file: specs/entities/Product.yaml
[INFO] Rendering entity template...
[INFO] Generated: src/MyService.Domain/Entities/Product.g.cs
✓ Entity generated successfully
```

### Error Output (stderr)

```text
[ERROR] Entity generation requires the persistence module

hint: Add the persistence module to the service:
    nfw add persistence --service MyService
```

## Interactive Mode

When `--no-input` is NOT specified and multiple services exist:

```bash
nfw gen entity Product --props Name:string,Price:decimal

? Select target service:
  > MyService
    AnotherService

? Generate schema only or generate code:
  > Generate code (schema + entity)
    Schema only (create YAML file)

? ID type:
  > int
    long
    Guid
```

## Validation Rules

### Entity Name

- Must be valid C# identifier
- Must be PascalCase
- Cannot be a C# reserved keyword
- Cannot conflict with existing entity in Domain layer

### Property Names

- Must be unique within entity
- Must be valid C# identifiers
- Must be PascalCase
- Cannot be reserved (Id, CreatedAt, UpdatedAt) unless explicitly allowed

### Property Types

- Must be from supported types list
- Cannot be complex types (classes, structs)
- Cannot be collection types (`List<T>`, arrays)

### Service Validation

- Service must exist in workspace
- Service must be a .NET service
- Service must have persistence module added

## Schema File Format

### Location

- Configured in `nfw.yaml`: `services.<serviceName>.entity-specs-path`
- Default: `<service_path>/specs/entities/`
- Auto-created if doesn't exist

### YAML Structure

```yaml
entity: Product
idType: integer
entityType: entity
properties:
  - name: Name
    type: string
    nullable: false
  - name: Price
    type: decimal
    nullable: false
```

## Edge Cases

### No Services

```bash
$ nfw gen entity Product --props Name:string
[ERROR] No .NET services found in workspace
hint: Add a service first or create a new .NET service
```

### Missing Persistence Module

```bash
$ nfw gen entity Product --props Name:string --service MyService
[ERROR] Entity generation requires the persistence module

hint: Add the persistence module to the service:
    nfw add persistence --service MyService
```

### Invalid Property Type

```bash
$ nfw gen entity Product --props Name:InvalidType
[ERROR] Invalid property type: InvalidType

Valid types: string, int, long, decimal, double, float, bool,
            DateTime, DateTimeOffset, Guid, byte[]
```

### Entity Already Exists

```bash
$ nfw gen entity Product --props Name:string
[ERROR] Entity already exists: src/MyService.Domain/Entities/Product.g.cs

hint: Use a different entity name or remove the existing file
```

## Performance

- Target completion time: <3 seconds (FR-013)
- Includes validation, schema creation, template rendering, file I/O
- Measured on standard developer hardware

## Dependencies

- Workspace must have valid `nfw.yaml`
- Target service must have persistence module
- Entity templates must be configured
- Template engine must be accessible
