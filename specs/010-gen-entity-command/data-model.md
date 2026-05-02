# Data Model: nfw gen entity Command

**Feature**: Generate domain entity classes with schema-first approach
**Date**: 2026-04-30

## Core Entities

### AddEntityCommand

**Purpose**: Encapsulates parameters for entity generation

**Fields**:

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| entity_name | `String` | Name of the entity to generate | Required; valid C# identifier; not a reserved keyword |
| properties | `Vec<PropertyDefinition>` | List of property definitions | Required; non-empty; unique names |
| id_type | `String` | ID type (default: "int") | Optional; must be valid ID value type |
| service_name | `Option<String>` | Target service name | Optional; auto-select if single service |
| no_input | `bool` | Skip interactive prompts | Default: false |
| schema_only | `bool` | Create schema without generating code | Default: false |
| from_schema | `bool` | Generate from existing schema file | Default: false |

**Lifecycle**: Created from CLI arguments, validated, passed to handler

---

### PropertyDefinition

**Purpose**: Represents a single property definition from CLI input

**Fields**:

| Field | Type | Description | Validation |
|-------|------|-------------|------------|
| name | String | Property name in PascalCase | Required; valid C# identifier |
| cli_type | String | Original CLI type syntax | Required; must be supported primitive type |
| general_type | GeneralType | Mapped general type | Derived from cli_type |
| nullable | bool | Whether property is optional | Derived from Type? syntax |

**Validation Rules**:

- Name must be unique within entity
- Name cannot be reserved (Id, CreatedAt, UpdatedAt if auto-generated)
- cli_type must be in supported types list
- GeneralType is derived via type mapping table

**Examples**:

- `Name:string` → name="Name", cli_type="string", general_type=String, nullable=false
- `Email:string?` → name="Email", cli_type="string", general_type=String, nullable=true
- `Price:decimal` → name="Price", cli_type="decimal", general_type=Decimal, nullable=false

---

### GeneralType

**Purpose**: Language-agnostic type representation for schema files

**Variants**:

| Variant | CLI Input Types | Description |
|---------|-----------------|-------------|
| String | string | Text data |
| Integer | int, long | Numeric integer |
| Decimal | decimal, double, float | Decimal/float number |
| Boolean | bool | True/false |
| DateTime | DateTime, DateTimeOffset | Date and time |
| Uuid | Guid | Unique identifier |
| Bytes | byte[] | Binary data |

**Purpose**: Enables polyglot code generation by abstracting language-specific types

---

### EntityGenerationParameters

**Purpose**: Template rendering parameters passed to template engine

**Fields**:

| Field | Type | Description |
|-------|------|-------------|
| entity_name | `String` | Name of entity (PascalCase) |
| namespace | `String` | Target namespace for generated code |
| id_type | `GeneralType` | ID type as general type |
| id_type_cli | `String` | ID type as CLI syntax (for template) |
| properties | `Vec<PropertyTemplate>` | Properties for template rendering |
| base_class | `String` | Selected base class name |
| service_name | `String` | Target service name |
| service_path | `PathBuf` | File system path to service |

**Lifecycle**: Created by handler from validated command, passed to template engine

---

### PropertyTemplate

**Purpose**: Property representation for template rendering

**Fields**:

| Field | Type | Description |
|-------|------|-------------|
| name | `String` | Property name (PascalCase) |
| type | `GeneralType` | General type for schema |
| nullable | `bool` | Optional flag |
| validations | `Vec<ValidationRule>` | Validation rules (optional) |

---

### EntitySchema

**Purpose**: In-memory representation of YAML schema file

**Fields**:

| Field | Type | Description |
|-------|------|-------------|
| entity | `String` | Entity name |
| id_type | `String` | ID type (general type) |
| entity_type | `String` | Entity type (entity, auditable-entity, soft-deletable-entity) |
| properties | `Vec<SchemaProperty>` | Property definitions |

**Serialization**: Maps to/from YAML structure

---

### SchemaProperty

**Purpose**: Property definition in schema file

**Fields**:

| Field | Type | Description |
|-------|------|-------------|
| name | `String` | Property name |
| type | `String` | General type |
| nullable | `bool` | Optional flag |
| validations | `Option<Vec<ValidationRule>>` | Optional validation rules |

**YAML Example**:

```yaml
name: Name
type: string
nullable: false
validations:
  - type: required
  - type: maxLength
    value: 100
```

---

### ValidationRule

**Purpose**: Validation rule definition for properties

**Fields**:

| Field | Type | Description |
|-------|------|-------------|
| rule_type | String | Rule type (required, maxLength, range, etc.) |
| parameters | HashMap<String, serde_yaml::Value> | Rule-specific parameters |

**Examples**:

- Required: `{rule_type: "required"}`
- Max length: `{rule_type: "maxLength", parameters: {value: 100}}`
- Range: `{rule_type: "range", parameters: {min: 0, max: 1000}}`

---

## Error Types

### EntityGenerationError

**Purpose**: Comprehensive error handling for entity generation

**Variants**:

| Variant | Trigger | Exit Code |
|---------|---------|-----------|
| NoServicesFound | No .NET services in workspace | 1 |
| ServiceNotFound | Specified service doesn't exist | 1 |
| MissingPersistenceModule | Persistence module not added | 1 |
| InvalidEntityName | Entity name violates C# rules | 1 |
| InvalidPropertySyntax | Property syntax malformed | 1 |
| InvalidPropertyType | Property type not supported | 1 |
| DuplicatePropertyName | Same property name twice | 1 |
| EmptyPropertiesList | No properties provided | 1 |
| InvalidIdType | ID type not supported | 1 |
| SchemaFileNotFound | --from-schema but file missing | 1 |
| InvalidSchemaYaml | Schema file has invalid YAML | 1 |
| SchemaWriteError | Cannot write schema file | 1 |
| TemplateExecutionFailed | Template engine failure | 1 |
| DomainLayerNotFound | Domain layer missing | 1 |
| EntityAlreadyExists | Entity file already exists | 1 |
| PermissionDenied | Cannot write to directory | 1 |

---

## Value Objects

### ServiceInfo

**Purpose**: Service metadata from workspace configuration

**Fields**:

| Field | Type | Description |
|-------|------|-------------|
| name | `String` | Service name |
| path | `PathBuf` | Path to service directory |
| modules | `Vec<String>` | Added modules (persistence, mediator, etc.) |
| entity_specs_path | `Option<PathBuf>` | Configured schema directory |

---

### WorkspaceContext

**Purpose**: Workspace configuration and services

**Fields**:

| Field | Type | Description |
|-------|------|-------------|
| root | `PathBuf` | Workspace root path |
| services | `Vec<ServiceInfo>` | Available services |
| default_service | `Option<ServiceInfo>` | Single service if only one |

---

## Relationships

```text
AddEntityCommand
    ├── validates → PropertyDefinition
    ├── maps → GeneralType
    ├── requires → ServiceInfo (from WorkspaceContext)
    └── produces → EntityGenerationParameters

PropertyDefinition
    └── maps to → GeneralType

EntityGenerationParameters
    └── renders → EntitySchema (YAML)

EntitySchema
    └── serializes to → YAML file
```

## State Transitions

### Entity Generation Flow

```text
[CLI Arguments]
    ↓
[Parse & Validate]
    ↓
[Check Persistence Module]
    ↓
[Create EntityGenerationParameters]
    ↓
[Generate Schema File] ←──┐
    ↓                      │
[Invoke Template Engine]    │
    ↓                      │
[Write Entity Code]         │
    ↓                      │
[Success]            [Schema-Only Mode]
```

### From-Schema Flow

```text
[--from-schema flag]
    ↓
[Read Schema File]
    ↓
[Parse YAML]
    ↓
[Validate Schema]
    ↓
[Create EntityGenerationParameters]
    ↓
[Invoke Template Engine]
    ↓
[Write Entity Code]
```
