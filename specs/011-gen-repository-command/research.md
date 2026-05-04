# Research: Generate Repository Command (CLI Focus)

## Overview

Research completed during spec phase. No `NEEDS CLARIFICATION` items remained in the specification.

## Key Decisions (CLI-Related Only)

### Decision 1: Template-Driven Generation

- **What was chosen**: CLI reads repository template configuration from `src/nfw-templates/src/dotnet-service/repository/template.yaml` and applies defined steps (render, inject, run_command).
- **Rationale**: Follows existing patterns (entity, persistence generators). Increases flexibility - template changes don't require CLI code changes.
- **Alternatives considered**:
  - Hardcoding file paths in CLI code → Rejected: Reduces flexibility, harder to maintain.
  - Using a separate config file per project → Rejected: Inconsistent with existing template approach.

### Decision 2: Entity-Specific File Generation

- **What was chosen**: CLI generates entity-specific repository files (e.g., `IUserRepository.cs`, `UserRepository.cs`) by applying templates.
- **Rationale**: Template configuration defines entity-specific output files.
- **Alternatives considered**:
  - Generic repository → Rejected: User requested entity-specific files.
  - No interface generation → Rejected: Template supports full generation.

### Decision 3: DI Registration via Template

- **What was chosen**: CLI injects DI registration into Infrastructure layer's service registration file by applying template configuration.
- **Rationale**: Template defines DI registration pattern; CLI applies it.
- **Alternatives considered**:
  - Hardcoding DI patterns in CLI → Rejected: Violates template-driven approach.
  - No DI registration → Rejected: Breaks dependency injection pattern.

### Decision 4: TContext from nfw.yaml

- **What was chosen**: CLI reads `dbContextType` from `nfw.yaml` persistence section for template substitution.
- **Rationale**: Consistent with `nfw add persistence` output and project configuration.
- **Alternatives considered**:
  - Hardcode `AppDbContext` → Rejected: Not flexible for different project setups.
  - Infer from project files → Rejected: Unreliable, configuration is explicit.

## Template Configuration That CLI Reads

- **Template schema**: `.specify/template.schema.json` (actions: render, render_folder, inject, run_command)
- **Repository config**: `src/nfw-templates/src/dotnet-service/repository/template.yaml` (CLI parses this YAML)
- **Template files**: `.tera` files in `content/` subdirectory (CLI renders these)

## What This Document Does NOT Cover

- ❌ Generated .NET code structure (that's in templates)
- ❌ Base repository interfaces (`IReadRepository`, etc.) - those are in `core-persistence-dotnet`
- ❌ .NET project layer conventions (Application, Infrastructure) - those are template-defined
- ❌ DI registration patterns in C# - that's template-defined

**The CLI just reads templates and applies them. It does NOT care about .NET specifics.**
