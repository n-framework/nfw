# Research: Workspace Structure and `nfw new` Command

## Decision 1: Routing Model for `nfw new`

- **Decision**: Use a single deterministic route for `nfw new` where parsing produces one normalized request model before any filesystem writes.
- **Rationale**: Prevents divergent behavior between interactive and non-interactive modes and keeps validation centralized.
- **Alternatives considered**:
  - Separate handlers for interactive/non-interactive paths (rejected: duplicates validation logic)
  - Late validation during file generation (rejected: higher rollback risk)

## Decision 2: Prompting Boundary

- **Decision**: Prompt only when terminal is interactive and required values are missing. With `--no-input`, never prompt and fail fast for missing required values.
- **Rationale**: Matches automation expectations and avoids hanging CI jobs.
- **Alternatives considered**:
  - Always prompt unless explicit all-flags mode (rejected: unsafe for automation)
  - Silent defaults for missing required values (rejected: hidden behavior)

## Decision 3: Existing Target Directory Policy

- **Decision**: If target directory exists and is non-empty, fail immediately before generation.
- **Rationale**: Prevents accidental overwrites and preserves deterministic outcomes.
- **Alternatives considered**:
  - Merge by default (rejected: non-deterministic conflicts)
  - Prompt for overwrite in interactive mode (rejected: bifurcated semantics)

## Decision 4: Workspace Layout Baseline

- **Decision**: Standardize layered root layout: `src/`, `tests/`, `docs/`, and root-level solution/config files.
- **Rationale**: Improves discoverability and aligns with current repository conventions.
- **Alternatives considered**:
  - Flat root (rejected: root clutter)
  - Service-first only layout (rejected: weak workspace-level orchestration)

## Decision 5: Namespace Rule

- **Decision**: Use workspace-root base namespace with service and layer suffixes.
- **Rationale**: Ensures predictable naming and avoids collisions across generated services.
- **Alternatives considered**:
  - Service-only namespace roots (rejected: inconsistent multi-service naming)
  - Fixed template namespace only (rejected: weak workspace identity)

## Decision 6: Solution Organization

- **Decision**: Generate one root workspace solution and per-service solution files.
- **Rationale**: Supports both workspace-wide and service-focused workflows.
- **Alternatives considered**:
  - Root-only solution (rejected: less isolated service workflows)
  - Service-only solutions (rejected: no workspace aggregation)

## Decision 7: Configuration Format

- **Decision**: Use YAML only for baseline generated configuration files.
- **Rationale**: Single canonical format simplifies validation, docs, and generated contracts.
- **Alternatives considered**:
  - YAML + JSON dual support (rejected: higher maintenance and ambiguity)
  - Template-specific format freedom (rejected: weak consistency guarantees)

## Decision 8: Error Contract

- **Decision**: Validation and runtime failures must include actionable messages and stable non-zero exits; successful execution must be side-effect complete.
- **Rationale**: Required for reliable scripting and predictable CLI UX.
- **Alternatives considered**:
  - Generic error messages (rejected: poor remediation)
  - Best-effort partial generation (rejected: ambiguous state)

## Result

All phase-0 clarifications needed for planning are resolved for this feature.
