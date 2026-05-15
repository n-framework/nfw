# Research: Generate Repository Command (CLI Focus)

## Overview

Research completed during spec phase. No `NEEDS CLARIFICATION` items remained in the specification.

## Key Decisions (CLI-Related Only)

### Decision 1: Generator-Driven Generation

- **What was chosen**: CLI reads repository generator configuration from `src/nfw-generators/src/dotnet-service/repository/nfw.generator.yaml` and applies defined steps (render, inject, run_command).
- **Rationale**: Follows existing patterns (entity, persistence generators). Increases flexibility - generator changes don't require CLI code changes.
- **Alternatives considered**:
  - Hardcoding file paths in CLI code → Rejected: Reduces flexibility, harder to maintain.
  - Using a separate config file per project → Rejected: Inconsistent with existing generator approach.

### Decision 2: Entity-Specific File Generation

- **What was chosen**: CLI generates entity-specific repository files (e.g., `IUserRepository.cs`, `UserRepository.cs`) by applying generators.
- **Rationale**: Generator configuration defines entity-specific output files.
- **Alternatives considered**:
  - Generic repository → Rejected: User requested entity-specific files.
  - No interface generation → Rejected: Generator supports full generation.

### Decision 3: DI Registration via Generator

- **What was chosen**: CLI injects DI registration into Infrastructure layer's service registration file by applying generator configuration.
- **Rationale**: Generator defines DI registration pattern; CLI applies it.
- **Alternatives considered**:
  - Hardcoding DI patterns in CLI → Rejected: Violates generator-driven approach.
  - No DI registration → Rejected: Breaks dependency injection pattern.

### Decision 4: TContext from nfw.yaml

- **What was chosen**: CLI reads `dbContextType` from `nfw.yaml` persistence section for generator substitution.
- **Rationale**: Consistent with `nfw add persistence` output and project configuration.
- **Alternatives considered**:
  - Hardcode `AppDbContext` → Rejected: Not flexible for different project setups.
  - Infer from project files → Rejected: Unreliable, configuration is explicit.

## Generator Configuration That CLI Reads

- **Generator schema**: `.specify/generator.schema.json` (actions: render, render_folder, inject, run_command)
- **Repository config**: `src/nfw-generators/src/dotnet-service/repository/nfw.generator.yaml` (CLI parses this YAML)
- **Generator files**: `.tera` files in `content/` subdirectory (CLI renders these)

## What This Document Does NOT Cover

- ❌ Generated .NET code structure (that's in generators)
- ❌ Base repository interfaces (`IReadRepository`, etc.) - those are in `core-persistence-dotnet`
- ❌ .NET project layer conventions (Application, Infrastructure) - those are generator-defined
- ❌ DI registration patterns in C# - that's generator-defined

**The CLI just reads generators and applies them. It does NOT care about .NET specifics.**
