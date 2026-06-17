# Generator Repository Structure Contract

**Version**: 1.0
**Date**: 2026-03-29

This document defines the required and optional structure of a generator repository.

## Repository Types

### Type A: Single Generator Repository

Contains exactly one generator.

```bash
generator-name/
├── nfw.generator.yaml          # REQUIRED: Generator metadata
├── content/               # REQUIRED: Generator file tree
│   └── (files to generate)
└── .nfwignore             # OPTIONAL: Exclusion patterns
```text

### Type B: Generator Catalog Repository

Contains multiple generators in subdirectories.

```bash
catalog-name/
├── generator-1/
│   ├── nfw.generator.yaml      # REQUIRED: Generator metadata
│   ├── content/           # REQUIRED: Generator file tree
│   │   └── (files)
│   └── .nfwignore         # OPTIONAL: Per-generator excludes
├── generator-2/
│   ├── nfw.generator.yaml
│   └── content/
└── README.md              # OPTIONAL: Catalog documentation
```text

## Required Files

### nfw.generator.yaml

Must be located at the root of each generator directory.

**Location**:

- Single generator: Repository root
- Catalog: Subdirectory root

**Schema**: See `generator-metadata-schema.yaml`

**Validation**:

- Must be valid YAML
- All required fields present
- All field values pass validation rules

### content/ Directory

Contains the file tree that will be generated when using this generator.

**Requirements**:

- Must be a directory named `content` exactly
- Must exist at the same level as `nfw.generator.yaml`
- Structure mirrors expected output workspace/service structure

**Supported Patterns**:

- Placeholder substitution: `__PlaceholderName__`
- Conditional includes (future): May be controlled via config file

## Optional Files

### .nfwignore

Defines files to exclude from the generated output.

**Location**: Same level as `nfw.generator.yaml`

**Format**: Similar to `.gitignore`, one pattern per line

**Example**:

```gitignore
# Exclude documentation from generated output
README.md
docs/

# Exclude .git directory
.git/

# Exclude IDE files
.vscode/
.idea/
```text

**Behavior**:

- Patterns are matched against files in `content/`
- Matched files are not copied during workspace generation
- Supports glob patterns (`*`, `**`, `?`)

### generator-config.yaml (Future)

Reserved for future enhancements:

- Conditional includes
- Variable substitution rules
- File permissions
- Post-generation scripts

**Note**: Not implemented in initial release.

## Placeholder Syntax

Placeholders in the `content/` directory are replaced during generation.

**Pattern**: `__PlaceholderName__`

**Rules**:

- Must start and end with double underscores
- Must contain at least one character between
- Case-sensitive
- Only alphanumeric characters (A-Z, a-z, 0-9)

**Standard Placeholders**:

| Placeholder       | Description                         | Example Value      |
| ----------------- | ----------------------------------- | ------------------ |
| `__ServiceName__` | Name of the service being generated | `Orders`           |
| `__Namespace__`   | Project namespace                   | `MyCompany.Orders` |
| `__ProjectGuid__` | Generated GUID for .NET projects    | `{12345678-...}`   |
| `__Year__`        | Current year                        | `2026`             |

**Example**:

```bash
content/
└── src/
    └── __ServiceName__/        # Becomes: src/Orders/
        ├── __ServiceName__.cs  # Becomes: Orders.cs
        └── __ServiceName__Controller.cs
```text

## Repository Validation

The CLI validates a generator repository on discovery:

1. **Structure Check**:
   - `nfw.generator.yaml` exists
   - `content/` directory exists
   - Both are at expected locations

2. **Metadata Check**:
   - YAML is valid
   - All required fields present
   - All fields pass validation

3. **Content Check**:
   - `content/` is not empty
   - Files are readable

4. **Failure Handling**:
   - Invalid generators are skipped
   - Warning logged with specific reason
   - Other generators in catalog still indexed

## Example: Complete Generator Repository

```bash
https://github.com/n-framework/nfw-generators

nfw-generators/
├── microservice/                    # Generator 1
│   ├── nfw.generator.yaml
│   ├── content/
│   │   ├── __ServiceName__.WebApi/
│   │   │   ├── Controllers/
│   │   │   └── Program.cs
│   │   ├── __ServiceName__.Application/
│   │   ├── __ServiceName__.Domain/
│   │   └── __ServiceName__.Infrastructure/
│   └── .nfwignore
├── grpc-service/                    # Generator 2
│   ├── nfw.generator.yaml
│   ├── content/
│   │   └── ...
│   └── .nfwignore
└── README.md                        # Catalog documentation
```text
