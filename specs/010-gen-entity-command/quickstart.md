# Quickstart: nfw gen entity Command

**Purpose**: Generate domain entity classes with schema-first approach
**Prerequisites**: .NET service with persistence module added

## Prerequisites

Before generating entities, ensure you have:

1. **A workspace with a .NET service**

   ```bash
   # Check your services
   nfw list services
   ```

2. **Persistence module added to the service**

   ```bash
   # Add persistence if not already present
   nfw add persistence --service MyService
   ```

## Quick Start

### Generate Your First Entity

The simplest way to generate an entity:

```bash
nfw gen entity Product --props Name:string,Price:decimal --no-input
```

This creates:

- Schema file: `specs/entities/Product.yaml`
- Entity code: `src/MyService.Domain/Entities/Product.g.cs`

### Generate with Nullable Properties

Use `?` suffix for optional properties:

```bash
nfw gen entity Customer --props Email:string,PhoneNumber:string? --no-input
```

### Specify ID Type

Use `--id-type` for custom ID types:

```bash
nfw gen entity Order --props OrderDate:datetime --id-type uuid --no-input
```

### Specify Entity Type

Use `--entity-type` for different entity types:

```bash
# Standard entity
nfw gen entity Product --props Name:string --entity-type entity --no-input

# Auditable entity (tracks created/updated timestamps)
nfw gen entity Category --props Name:string --entity-type auditable-entity --no-input

# Soft-deletable entity (supports soft delete)
nfw gen entity Tag --props Name:string --entity-type soft-deletable-entity --no-input
```

### Target Specific Service

When you have multiple services:

```bash
nfw gen entity Product --props Name:string --service MyService --no-input
```

## Common Workflows

### Workflow 1: Schema-First Development

Create schema first, review/edit, then generate code:

```bash
# Step 1: Create schema only
nfw gen entity Product --props Name:string,Price:decimal --schema-only --no-input

# Step 2: Review and edit schema
cat specs/entities/Product.yaml
# Edit manually if needed

# Step 3: Generate code from schema
nfw gen entity Product --from-schema --no-input
```

### Workflow 2: Quick Development

Generate everything in one command:

```bash
nfw gen entity Category --props Name:string,Description:string? --no-input
```

### Workflow 3: Interactive Mode

Let the CLI prompt for options:

```bash
nfw gen entity Product --props Name:string,Price:decimal
```

## Supported Property Types

| Type | Description | Example |
|------|-------------|---------|
| string | Text | `Name:string` |
| int | 32-bit integer | `Count:int` |
| long | 64-bit integer | `Population:long` |
| decimal | Decimal number | `Price:decimal` |
| double | Double precision | `Score:double` |
| float | Single precision | `Rating:float` |
| bool | Boolean | `IsActive:bool` |
| DateTime | Date and time | `CreatedAt:DateTime` |
| DateTimeOffset | Date/time with offset | `Timestamp:DateTimeOffset` |
| Guid | Unique identifier | `CategoryId:Guid` |
| byte[] | Binary data | `Thumbnail:byte[]` |

### Nullable Properties

Add `?` to make a property optional:

```bash
nfw gen entity User --props Username:string,AvatarUrl:string? --no-input
```

## Schema Files

### Location

Schema files are stored in:

- **Default**: `<service_path>/specs/entities/`
- **Configurable**: Set `entity-specs-path` in `nfw.yaml`

### Schema Structure

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

### Manual Editing

You can edit schema files manually:

```bash
# Edit schema
nano specs/entities/Product.yaml

# Regenerate entity from edited schema
nfw gen entity Product --from-schema --no-input
```

## Error Handling

### Missing Persistence Module

```bash
$ nfw gen entity Product --props Name:string

error: Entity generation requires the persistence module

hint: Add the persistence module:
    nfw add persistence --service MyService
```

### Invalid Property Type

```bash
$ nfw gen entity Item --props Count:InvalidType

error: Invalid property type: InvalidType

Valid types: string, int, long, decimal, double, float, bool,
            DateTime, DateTimeOffset, Guid, byte[]
```

### Entity Already Exists

```bash
$ nfw gen entity Product --props Name:string

error: Entity already exists: src/MyService.Domain/Entities/Product.g.cs

hint: Use a different name or remove the existing file
```

## Next Steps

After generating an entity:

1. **Review generated code**: Check `src/MyService.Domain/Entities/Product.g.cs`
2. **Compile**: Ensure code compiles without errors
3. **Add repository** (if needed): Generate repository for the entity
4. **Add commands/queries**: Create CQRS operations for the entity
5. **Add API endpoints**: Expose HTTP endpoints for the entity

## Examples

### E-Commerce Product

```bash
nfw gen entity Product \
  --props Name:string,Description:string?,Price:decimal,Stock:int,IsActive:bool \
  --no-input
```

### User Account

```bash
nfw gen entity User \
  --props Username:string,Email:string,PasswordHash:string,LastLogin:DateTime? \
  --id-type Guid \
  --no-input
```

### Blog Post

```bash
nfw gen entity BlogPost \
  --props Title:string,Content:string,PublishedAt:DateTime?,AuthorId:Guid \
  --no-input
```

## Tips

- **Use --no-input**: Always include `--no-input` in scripts or CI/CD
- **Schema-first for complex entities**: Use `--schema-only` to review before generating
- **Check existing entities**: List files in `Domain/Entities/` before generating
- **Validate schema**: Use `--from-schema` to validate schema file syntax
- **Version control**: Commit schema files alongside generated code

## Reference

- Full specification: [spec.md](./spec.md)
- Data model: [data-model.md](./data-model.md)
- CLI contract: [contracts/cli-schema.md](./contracts/cli-schema.md)
