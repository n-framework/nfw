<!-- markdownlint-disable MD024 -->
# Research: nfw gen entity Command

**Feature**: Generate domain entity classes with schema-first approach
**Date**: 2026-04-30
**Status**: Complete

## Type Mapping: CLI Syntax to General Types

### Decision

CLI arguments accept C#-like primitive type syntax for developer familiarity. These types are mapped to language-agnostic general types in the schema file.

### Type Mapping Table

| CLI Input Syntax | General Type (Schema) | Description |
|-----------------|----------------------|-------------|
| `string` | `string` | Text data |
| `int` | `integer` | 32-bit integer |
| `long` | `integer` | 64-bit integer |
| `decimal` | `decimal` | High-precision decimal |
| `double` | `decimal` | Floating-point number |
| `float` | `decimal` | Single-precision float |
| `bool` | `boolean` | True/false |
| `DateTime` | `datetime` | Date and time |
| `DateTimeOffset` | `datetime` | Date and time with offset |
| `Guid` | `uuid` | Globally unique identifier |
| `byte[]` | `bytes` | Binary data |

### Rationale

- **Developer Experience**: C#-like syntax is familiar to .NET developers who are the primary users
- **Polyglot Support**: General types in schema enable future multi-language code generation
- **Separation of Concerns**: CLI handles input parsing; templates handle language-specific type mapping

### Alternatives Considered

1. **Use general types in CLI**: Rejected - less intuitive for target users
2. **Store language-specific types in schema**: Rejected - breaks polyglot support
3. **Support multiple CLI syntaxes**: Rejected - adds complexity without clear benefit

## Schema File Format

### Decision

Schema files use YAML format with a flat structure containing entity metadata and property definitions.

### Schema Structure

```yaml
entity: Product              # Entity name (PascalCase)
idType: uuid                 # General type for ID (uuid, integer, etc.)
entityType: entity            # Entity type: entity, auditable-entity, or soft-deletable-entity
properties:                  # List of property definitions
  - name: Name               # Property name (PascalCase)
    type: string             # General type (string, integer, decimal, etc.)
    nullable: false          # Whether property is optional
  - name: Price
    type: decimal
    nullable: false
```

### Schema Storage Location

- **Configuration**: `services.<serviceName>.entity-specs-path` in `nfw.yaml`
- **Default**: `<service_path>/specs/entities/` if not configured
- **Directory Creation**: Automatically created if doesn't exist

### Rationale

- **YAML over JSON**: More readable for manual editing, supports comments
- **Flat structure**: Simple to parse, no nested complexity
- **General types only**: Keeps schema language-agnostic for polyglot support
- **Nullable flag**: Explicit control over optional properties

### Alternatives Considered

1. **JSON format**: Rejected - less readable, no comments
2. **Nested property structure**: Rejected - adds complexity without benefit
3. **Include database types**: Rejected - templates handle DB type mapping

## Template Integration

### Decision

The CLI command provides entity parameters to the template engine. Templates determine:

- Base class selection (Entity, AuditableEntity, SoftDeletableEntity)
- Language-specific type mappings (general type → C# type)
- Validation attributes based on property types
- Generated code structure and formatting

### Template Parameters

```rust
TemplateParameters {
    entity_name: String,
    namespace: String,
    id_type: GeneralType,
    properties: Vec<Property>,
    base_class: String,
    service_name: String,
}
```

### Template Responsibilities

1. **Type Mapping**: Map general types to language-specific types
   - `string` → `string`
   - `integer` → `int` or `long` based on context
   - `decimal` → `decimal`
   - `boolean` → `bool`
   - `datetime` → `DateTime` or `DateTimeOffset`
   - `uuid` → `Guid`
   - `bytes` → `byte[]`

2. **Base Class Selection**: Use `baseClass` field to determine inheritance
   - `Entity` → `public class Product : Entity<Guid>`
   - `AuditableEntity` → `public class Product : AuditableEntity<Guid>`
   - `SoftDeletableEntity` → `public class Product : SoftDeletableEntity<Guid>`

3. **Validation Attributes**: Add attributes based on type and template rules
   - Required attributes for non-nullable properties
   - String length attributes based on naming conventions
   - Range attributes for numeric types

### Rationale

- **Separation of Concerns**: CLI handles validation; templates handle code generation
- **Flexibility**: Different templates can provide different behaviors
- **Language Independence**: Same schema can generate code for multiple languages

### Alternatives Considered

1. **CLI generates code directly**: Rejected - breaks template abstraction
2. **Templates read YAML directly**: Rejected - couples templates to schema format
3. **Hardcoded type mappings in CLI**: Rejected - limits template flexibility

## Service Module Validation

### Decision

Entity generation requires the persistence module to be added first. The command validates module presence in `nfw.yaml` before proceeding.

### Validation Logic

```rust
fn validate_persistence_module(service: &ServiceInfo) -> Result<(), EntityGenerationError> {
    let has_persistence = service.modules.contains(&"persistence".to_string());
    if !has_persistence {
        return Err(EntityGenerationError::MissingPersistenceModule {
            service: service.name.clone(),
            hint: format!("nfw add persistence --service {}", service.name),
        });
    }
    Ok(())
}
```

### Error Message

```text
error: Entity generation requires the persistence module

hint: Add the persistence module to the service:
    nfw add persistence --service MyService
```

### Rationale

- **Clear Dependency**: Entities depend on persistence infrastructure (DbContext, repositories)
- **Fail Fast**: Prevents generation that would fail at compile time
- **Actionable Feedback**: Provides exact command to fix the issue

### Alternatives Considered

1. **Auto-add persistence module**: Rejected - violates explicit module addition workflow
2. **Allow generation without persistence**: Rejected - generated code wouldn't compile
3. **Warning instead of error**: Rejected - would create broken state

## Property Validation Rules

### Decision

Properties must be primitive types only. Complex types, collections, and nested objects are not supported.

### Supported Property Types

- Primitive types only (see Type Mapping table above)
- No collection types (`List<T>`, Array, etc.)
- No nested objects or value objects
- Nullable syntax supported (Type?)

### Validation Flow

1. Parse property syntax: `PropertyName:Type?`
2. Extract property name and type
3. Validate type is in supported list
4. Check for duplicate property names
5. Validate property name against C# identifier rules

### Error Handling

```rust
match parse_property_definition(input) {
    Ok(prop) => validate_property_type(&prop.type)?,
    Err(ParseError::InvalidSyntax) => {
        Err(EntityGenerationError::InvalidPropertySyntax {
            hint: "Expected format: PropertyName:Type or PropertyName:Type?",
        })
    }
    Err(ParseError::InvalidType { type_name }) => {
        Err(EntityGenerationError::InvalidPropertyType {
            type_name,
            valid_types: vec!["string", "int", "long", "decimal", "bool", "DateTime", "Guid", "byte[]"],
        })
    }
}
```

### Rationale

- **Simplicity**: Primitive types are sufficient for basic entity properties
- **Polyglot Compatibility**: Primitive types map consistently across languages
- **Complex Types Elsewhere**: Navigation properties and value objects handled separately

### Alternatives Considered

1. **Support collection types**: Rejected - adds complexity, templates should handle navigation properties
2. **Support value objects**: Rejected - separate concern, should be own command
3. **Infer type from name**: Rejected - explicit types are clearer

## Implementation Order

1. **Property Syntax Parser**: Parse CLI arguments into structured data
2. **Type Mapper**: Map C# types to general types
3. **Service Module Validator**: Check persistence module presence
4. **Schema Generator**: Create YAML files from entity definitions
5. **Schema Reader**: Parse YAML files for code generation
6. **Template Integration**: Invoke template engine with parameters
7. **CLI Command**: Wire everything together with clap

## Open Questions Resolved

All questions from the specification have been resolved through this research:

- **Type mapping**: CLI uses C# syntax, schema uses general types, templates handle language-specific types
- **Base class selection**: Template concern based on `baseClass` field in schema
- **Schema storage**: Configurable in `nfw.yaml` with sensible default
- **Persistence module dependency**: Validated before generation with actionable error message
- **Property type restrictions**: Primitive types only, validated at CLI input
