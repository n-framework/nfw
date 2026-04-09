# Implementation Plan: Workspace Structure and `nfw new` Command

**Branch**: `002-workspace-structure-new-command` | **Date**: 2026-04-02 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `src/nfw/specs/002-workspace-structure-new-command/spec.md`

## Summary

Implement deterministic workspace bootstrapping through `nfw new` with a layered workspace root (`src/`, `tests/`, `docs/`), YAML-only baseline configuration, template-driven artifact generation, and strict CLI routing/validation behavior for interactive and `--no-input` flows.

## Technical Context

**Language/Version**: Rust 1.85+ (2024 edition)
**Primary Dependencies**: clap (CLI parsing), serde + serde_yaml (YAML config/model I/O), semver (template version resolution), regex (validation)
**Storage**: File system (workspace artifacts, template cache, user config)
**Testing**: cargo test (unit + integration), benchmark-style integration checks for acceptance goals
**Target Platform**: Linux, macOS, Windows (CLI)
**Project Type**: CLI application (multi-crate clean architecture)
**Performance Goals**: Command parsing errors returned immediately; `nfw new` end-to-end generation remains deterministic and fast for standard template sizes
**Constraints**: `--no-input` must never prompt; existing non-empty target must fail immediately; YAML is the only baseline config format
**Scale/Scope**: Single workspace generation per command invocation; supports official and registered template catalogs already defined by `001-nfw-template-system`

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
| --- | --- | --- |
| I. Single-Step Build And Test | Pass | Generated workspace quickstart mandates one build command and one test command. |
| II. CLI I/O And Exit Codes | Pass | Plan requires deterministic success/failure outcomes and actionable CLI diagnostics. |
| III. No Suppression | Pass | Validation and runtime errors are surfaced; no warning/test suppression introduced. |
| IV. Deterministic Tests | Pass | Unit tests use isolated inputs; integration tests avoid uncontrolled external dependencies. |
| V. Documentation Is Part Of Delivery | Pass | `quickstart.md` and contract artifacts are produced in this phase. |

No constitutional violations identified before Phase 0.

## Phase 0: Research

Research questions resolved in [research.md](./research.md):

1. Deterministic command routing and option precedence for `nfw new`
2. Interactive vs non-interactive prompting boundaries
3. Canonical YAML-only baseline configuration strategy
4. Workspace/template namespace consistency rules
5. Failure handling for existing directories and invalid inputs

Outcome: all identified ambiguities resolved and encoded as implementation decisions.

## Phase 1: Design & Contracts

Design artifacts generated:

- [data-model.md](./data-model.md)
- [contracts/nfw-new-command-schema.yaml](./contracts/nfw-new-command-schema.yaml)
- [contracts/workspace-layout-contract.md](./contracts/workspace-layout-contract.md)
- [quickstart.md](./quickstart.md)

Post-design constitution re-check: **Pass** (all gates remain satisfied).

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/002-workspace-structure-new-command/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── nfw-new-command-schema.yaml
│   └── workspace-layout-contract.md
└── tasks.md                # created by /speckit.tasks
```

### Source Code (repository root)

```text
src/nfw/
├── src/
│   └── nframework-nfw/
│       ├── core/
│       │   ├── nframework-nfw-domain/
│       │   └── nframework-nfw-application/
│       ├── infrastructure/
│       │   ├── nframework-nfw-infrastructure-filesystem/
│       │   ├── nframework-nfw-infrastructure-git/
│       │   ├── nframework-nfw-infrastructure-yaml/
│       │   └── nframework-nfw-infrastructure-versioning/
│       └── presentation/
│           └── nframework-nfw-cli/
├── tests/
│   ├── unit/
│   └── integration/
└── specs/
    ├── 001-nfw-template-system/
    └── 002-workspace-structure-new-command/
```

**Structure Decision**: Existing `nframework-nfw` clean architecture crates are preserved. This feature adds/updates command parsing and workspace-generation flows within current presentation/application/infrastructure boundaries, plus spec artifacts under `src/nfw/specs/002-workspace-structure-new-command/`.

## Phase 2 Preview (for `/speckit.tasks`)

Expected task decomposition themes:

1. CLI routing surface and argument contract for `nfw new`
2. Workspace blueprint and namespace derivation rules
3. Template content and YAML baseline config generation
4. Interactive prompt orchestration and `--no-input` gating
5. Validation and deterministic failure paths
6. Unit/integration coverage for acceptance criteria

## Complexity Tracking

No constitution violations requiring justification.
