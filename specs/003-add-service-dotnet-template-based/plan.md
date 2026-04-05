# Implementation Plan: Template-Based `nfw add service`

**Branch**: `feat/add-service-template-based` | **Date**: 2026-04-04 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `src/nfw/specs/003-add-service-dotnet-template-based/spec.md`

## Summary

Implement `nfw add service <name>` as a template-first workflow that requires template selection in automation, prompts in interactive mode, renders service templates from `nfw-templates` into `src/<ServiceName>/`, and records template provenance in `nfw.yaml`.

## Technical Context

**Language/Version**: Rust 1.85+ (2024 edition)
**Primary Dependencies**: clap (CLI parsing), serde + serde_yaml (workspace metadata read/write), semver (template version resolution), existing template catalog/cache components from spec 001
**Storage**: File system (workspace service artifacts), workspace `nfw.yaml`, local template cache
**Testing**: cargo test (unit + integration), CLI integration smoke scenarios for generated output and failure modes
**Target Platform**: Linux, macOS, Windows (CLI)
**Project Type**: CLI application (multi-crate clean architecture)
**Performance Goals**: No new latency SLA is introduced in this feature; runtime focus is deterministic pre-generation validation/resolution before writes.
**Constraints**: Output path fixed to `src/<ServiceName>/`; `--template` required in non-interactive mode; only templates with `type=service`; cleanup on interruption/failure; no separate service templating subsystem
**Scale/Scope**: One service generation per invocation; initial service baseline uses Domain/Application/Infrastructure/Api layers provided by the selected service template

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|---|---|---|
| I. Single-Step Build And Test | Pass | Quickstart includes one-command verification and keeps root build/test workflows intact. |
| II. CLI I/O And Exit Codes | Pass | Command contract defines deterministic success/error exits and keeps SIGINT behavior explicit. |
| III. No Suppression | Pass | Plan requires explicit validation failures and surfaced actionable errors. |
| IV. Deterministic Tests | Pass | Unit tests avoid network; integration tests rely on local fixtures/template cache only. |
| V. Documentation Is Part Of Delivery | Pass | Plan includes quickstart and contracts for CLI behavior and dependency rules. |

No constitutional violations identified before Phase 0.

## Phase 0: Research

Research questions resolved in [research.md](./research.md):

1. Template selection policy for interactive vs non-interactive execution
2. Canonical service output location and collision behavior
3. Service-template eligibility and validation boundary
4. Provenance persistence model in `nfw.yaml`
5. Layer dependency enforcement strategy from template output to validation guarantees

Outcome: critical ambiguity is resolved and mapped to concrete implementation decisions.

## Phase 1: Design & Contracts

Design artifacts generated:

- [data-model.md](./data-model.md)
- [contracts/add-service-command-schema.yaml](./contracts/add-service-command-schema.yaml)
- [contracts/service-layer-dependency-contract.md](./contracts/service-layer-dependency-contract.md)
- [quickstart.md](./quickstart.md)

Post-design constitution re-check: **Pass** (all gates remain satisfied).

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/003-add-service-dotnet-template-based/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   ├── add-service-command-schema.yaml
│   └── service-layer-dependency-contract.md
└── tasks.md                # created by /speckit.tasks
```

### Source Code (repository root)

```text
src/nfw/
├── src/
│   └── nframework-nfw/
│       ├── core/
│       ├── infrastructure/
│       └── presentation/
├── tests/
│   ├── unit/
│   └── integration/
└── specs/
    ├── 001-nfw-template-system/
    ├── 002-workspace-structure-new-command/
    └── 003-add-service-dotnet-template-based/
```

**Structure Decision**: Reuse existing template system and CLI composition boundaries. Add service-generation command handling in application command flow, with rendering and write operations in infrastructure services.

## Phase 2 Preview (for `/speckit.tasks`)

Expected task decomposition themes:

1. CLI command surface and request normalization for `nfw add service`
2. Template selection/prompting and non-interactive validation
3. Service template rendering into `src/<ServiceName>/`
4. `nfw.yaml` provenance persistence and tests
5. Failure-path cleanup behavior and deterministic exit assertions

## Complexity Tracking

No constitution violations requiring justification.
