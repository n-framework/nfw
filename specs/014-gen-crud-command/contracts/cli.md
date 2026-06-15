# Contract: `nfw gen crud` Command

This contract defines the CLI interface for the `nfw gen crud` command, parsing rules, and the expected integration with the NFramework generator engine.

## CLI Usage

```bash
nfw gen crud <ENTITY_NAME> [OPTIONS]
```

## Arguments

- `ENTITY_NAME` (Required): The PascalCase name of the entity for which to generate the CRUD operations (e.g., `Product`, `CustomerOrder`). Must be a valid C# identifier.

## Options

- `--no-api`: Skips generation of the Presentation layer Minimal API endpoint files.
- `--secured`: Adds authorization/security markers to the generated Command and Query classes.
- `--cached`: Adds caching markers to the generated Query classes.
- `--force`: Overwrites existing files in the target directories without prompting.
- `--no-input`: Disables interactive prompts. If required prerequisites (like the entity itself) are missing, or files exist and `--force` is not passed, the command fails fast.

## Interactive Prompts

If run in a TTY environment without `--no-input`:

1. **Missing Entity**: If the specified entity is not found in the domain layer:
   - "Entity [ENTITY_NAME] not found. Create it now? (Y/n)"
2. **Missing Options**: If no feature flags are passed:
   - "Generate API Endpoints? (Y/n)"
   - "Include caching markers? (y/N)"
   - "Include security markers? (y/N)"
3. **Existing Files**: If target files already exist:
   - "Files for [ENTITY_NAME] already exist. Overwrite? (y/N)"

## Output Format

**Success (stdout)**:

```text
✓ Created Application/Features/Product/CreateProductCommand.cs
✓ Created Presentation.WebApi/Endpoints/Product/CreateProductEndpoint.cs
...
Generated CRUD for Product successfully in 1.42s.
```

**Error (stderr)**:

```text
Error: Entity 'Product' not found.
Run `nfw add entity Product` first, or run without --no-input to be prompted.
```
