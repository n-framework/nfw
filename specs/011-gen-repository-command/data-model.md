# Data Model: Generate Repository Command (CLI Focus)

## Overview

This document describes the **template configuration structure** that the nfw CLI reads and applies. It does NOT describe the generated .NET code (that's defined in templates).

## Template Configuration Structure

### Location That CLI Reads From

`src/nfw-templates/src/dotnet-service/repository/template.yaml`

### Configuration Format (YAML) That CLI Parses

```yaml
# yaml-language-server: $schema=../../../template.schema.json
id: dotnet-service/repository
name: Repository Generator
description: Generates repository files based on template configuration
version: 1.0.0
language: csharp
tags:
  - repository
  - feature
inputs:
  - id: Feature
    type: text
    prompt: "Feature name"
    default: ""
  - id: Entity
    type: text
    prompt: "Entity name"
    default: ""
steps:
  - action: render
    source: "content/interface/IEntityRepository.cs.tera"
    destination: "src/core/{{ Service }}.Core.Application/Features/{{ Feature }}/Repositories/I{{ Entity }}Repository.cs"
  - action: render
    source: "content/implementation/EntityRepository.cs.tera"
    destination: "src/infrastructure/{{ Service }}.Infrastructure.Persistence/Features/{{ Feature }}/Repositories/{{ Entity }}Repository.cs"
  - action: inject
    source: "content/di-registration/registration.tera"
    destination: "src/infrastructure/{{ Service }}.Infrastructure.Persistence/ServiceRegistration.cs"
    injection_target:
      type: region
      value: repository-registrations
```

## Template Placeholders That CLI Substitutes

| Placeholder | Description | Example Value | How CLI Gets It |
|-------------|-------------|---------------|-------------------|
| `{{ Service }}` | Service name | `MyService` | Reads from `nfw.yaml` |
| `{{ Feature }}` | Feature folder name | `user`, `payments` | From `--feature` flag or auto-detected |
| `{{ Entity }}` | Entity name | `User`, `Order` | From CLI positional argument |

## Configuration Validation That CLI Performs

Before applying the template, the CLI validates:

1. **Template exists**: `src/nfw-templates/src/dotnet-service/repository/template.yaml` is readable
2. **Template schema**: Valid YAML conforming to `template.schema.json`
3. **Required fields**: `id`, `name`, `steps` are present
4. **Step actions**: Each step has valid `action` (render, inject, run_command)
5. **Placeholders**: All required placeholders (`Service`, `Feature`, `Entity`) can be resolved

## Template File Structure That CLI Reads

```text
src/nfw-templates/src/dotnet-service/repository/
├── template.yaml              # Configuration that CLI reads
└── content/
    ├── interface/           # Template files that CLI renders
    │   └── IEntityRepository.cs.tera
    ├── implementation/
    │   └── EntityRepository.cs.tera
    └── di-registration/
        └── registration.tera
```

## nfw.yaml Configuration That CLI Reads

The CLI reads `nfw.yaml` for:

| Field | Purpose | Example |
|-------|----------|---------|
| `persistence` section | Validates persistence is configured | Required for command to proceed |
| `service.name` | Substitutes `{{ Service }}` placeholder | `MyService` |

## CLI Command Structure

```text
nfw gen repository <ENTITY> [--feature <FEATURE>]
```

### CLI Argument Parsing (using `clap`)

- Positional argument: `ENTITY` (required)
- Optional flag: `--feature <FEATURE>` (target feature folder)

### CLI Validation Flow

1. Parse CLI arguments (`clap`)
2. Read `nfw.yaml` → validate persistence section exists
3. Validate entity exists in feature's `Domain/Entities/` folder
4. Read repository template configuration from `src/nfw-templates/`
5. Substitute placeholders (`{{ Service }}`, `{{ Feature }}`, `{{ Entity }}`)
6. Apply template steps (render files, inject DI registration)
7. Complete in <2 seconds

## What This Document Does NOT Cover

- ❌ Generated .NET code structure (that's in templates)
- ❌ Base repository interfaces (`IReadRepository`, etc.) - those are in `core-persistence-dotnet`
- ❌ .NET project layer conventions (Application, Infrastructure) - those are template-defined
- ❌ DI registration patterns in C# - that's template-defined

**The CLI just reads templates and applies them. It does NOT care about .NET specifics.**
