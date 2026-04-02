# Template Repository Structure Contract

**Version**: 1.0
**Date**: 2026-03-29

This document defines the required and optional structure of a template repository.

## Repository Types

### Type A: Single Template Repository

Contains exactly one template.

```bash
template-name/
├── template.yaml          # REQUIRED: Template metadata
├── content/               # REQUIRED: Template file tree
│   └── (files to generate)
└── .nfwignore             # OPTIONAL: Exclusion patterns
```

### Type B: Template Catalog Repository

Contains multiple templates in subdirectories.

```bash
catalog-name/
├── template-1/
│   ├── template.yaml      # REQUIRED: Template metadata
│   ├── content/           # REQUIRED: Template file tree
│   │   └── (files)
│   └── .nfwignore         # OPTIONAL: Per-template excludes
├── template-2/
│   ├── template.yaml
│   └── content/
└── README.md              # OPTIONAL: Catalog documentation
```

## Required Files

### template.yaml

Must be located at the root of each template directory.

**Location**:

- Single template: Repository root
- Catalog: Subdirectory root

**Schema**: See `template-metadata-schema.yaml`

**Validation**:

- Must be valid YAML
- All required fields present
- All field values pass validation rules

### content/ Directory

Contains the file tree that will be generated when using this template.

**Requirements**:

- Must be a directory named `content` exactly
- Must exist at the same level as `template.yaml`
- Structure mirrors expected output workspace/service structure

**Supported Patterns**:

- Placeholder substitution: `__PlaceholderName__`
- Conditional includes (future): May be controlled via config file

## Optional Files

### .nfwignore

Defines files to exclude from the generated output.

**Location**: Same level as `template.yaml`

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
```

**Behavior**:

- Patterns are matched against files in `content/`
- Matched files are not copied during workspace generation
- Supports glob patterns (`*`, `**`, `?`)

### template-config.yaml (Future)

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
```

## Repository Validation

The CLI validates a template repository on discovery:

1. **Structure Check**:
   - `template.yaml` exists
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
   - Invalid templates are skipped
   - Warning logged with specific reason
   - Other templates in catalog still indexed

## Example: Complete Template Repository

```bash
https://github.com/n-framework/nfw-templates

nfw-templates/
├── microservice/                    # Template 1
│   ├── template.yaml
│   ├── content/
│   │   ├── __ServiceName__.Api/
│   │   │   ├── Controllers/
│   │   │   └── Program.cs
│   │   ├── __ServiceName__.Application/
│   │   ├── __ServiceName__.Domain/
│   │   └── __ServiceName__.Infrastructure/
│   └── .nfwignore
├── grpc-service/                    # Template 2
│   ├── template.yaml
│   ├── content/
│   │   └── ...
│   └── .nfwignore
└── README.md                        # Catalog documentation
```
