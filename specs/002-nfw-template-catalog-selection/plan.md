# Implementation Plan: Template Catalog Listing and Selection

**Branch**: `002-nfw-template-catalog-selection` | **Date**: 2026-03-17 | **Spec**: [`/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/spec.md`](/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/spec.md)  
**Spec Type**: Project-Based  
**Project**: nfw  
**Input**: Feature specification from [`/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/spec.md`](/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/spec.md)

## Summary

Extend the existing template catalog support in `nfw` so template metadata becomes a stable public contract for workspace creation. The implementation adds explicit template identifiers, deterministic catalog presentation, interactive selection only when the command is running in a real terminal, and strict explicit-template behavior for non-interactive execution. The work stays centered in the `src/nfw` module while adding any minimal repository-root wrapper or documentation updates needed to satisfy constitution requirements for root-level build/test usage.

## Technical Context

**Language/Version**: C# on .NET 11; Markdown for planning artifacts  
**Primary Dependencies**: Spectre.Console, Spectre.Console.Cli, Microsoft.Extensions.DependencyInjection, Microsoft.Extensions.Http, YamlDotNet, xUnit, FluentAssertions  
**Storage**: File system for template catalog YAML and generated workspace files; no database  
**Testing**: xUnit and FluentAssertions under `/home/ac/Code/n-framework/n-framework/src/nfw/tests/unit/NFramework.NFW/presentation/NFramework.NFW.CLI`, executed via `make test`  
**Target Platform**: Linux, macOS, and Windows terminals for local development and CI runners for non-interactive execution  
**Project Type**: CLI tool  
**Performance Goals**: Template listing and template-resolution validation should complete within the existing CLI expectations, and the workspace creation flow must continue supporting the PRD target of template-driven setup in seconds rather than minutes  
**Constraints**: CLI normal output stays on stdout and diagnostics/errors stay on stderr; exit codes remain stable (`0`, `1`, `2`, `130`); unit tests cannot require real network access; interactive prompting is allowed only when terminal interactivity is confirmed; no files may be generated before template resolution succeeds; build/test commands for the delivered CLI surface must remain documented and runnable from the repository root  
**Scale/Scope**: Single `src/nfw` project feature spanning current template listing behavior and the planned `nfw new` command, with one official catalog and a small number of user-selectable templates

## Constitution Check

_GATE: Must pass before Phase 0 research. Re-check after Phase 1 design._

### Pre-Research Gate Review

- **Single-Step Build And Test**: PASS with follow-up work. `src/nfw` already exposes `make build` and `make test`, and this feature will add or update repository-root wrapper/documentation support so the same build/test flow is runnable from the repository root.
- **CLI I/O And Exit Codes**: PASS. The plan keeps listing and prompt output on stdout, preserves diagnostics on stderr, and uses the documented exit-code contract for success, usage failures, runtime failures, and interruptions.
- **No Suppression**: PASS. The plan relies on existing warnings-as-errors and test enforcement and does not introduce suppression or swallowed failures.
- **Deterministic Tests**: PASS. Template selection tests will use fixture catalogs and fake console/interactivity adapters instead of real network access or brittle console dependencies.
- **Documentation Is Part Of Delivery**: PASS. This plan adds a quickstart plus explicit CLI contracts for listing and selection behavior.

## Project Structure

### Documentation (this feature)

```text
/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/
├── spec.md
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
└── contracts/
   └── cli.md
```

### Source Code (repository root)

```text
/home/ac/Code/n-framework/n-framework/
├── README.md                   # repository-root developer entrypoint documentation
├── scripts/
│  ├── build.sh                 # repository-root build wrapper for CLI validation
│  └── test.sh                  # repository-root test wrapper for CLI validation
└── src/nfw/
   ├── docs/
   │  └── PRD.md
   ├── packages/
   │  └── core-codegen/
   ├── src/
   │  └── NFramework.NFW/
   │     ├── core/
   │     │  ├── NFramework.NFW.Domain/
   │     │  │  └── Features/
   │     │  │     └── Templates/
   │     │  └── NFramework.NFW.Application/
   │     │     └── Features/
   │     │        ├── Cli/
   │     │        └── Templates/
   │     └── presentation/
   │        └── NFramework.NFW.CLI/
   │           ├── Program.cs
   │           └── Features/
   │              ├── Templates/
   │              └── New/                  # planned command surface for workspace creation
   └── tests/
      └── NFramework.NFW.CLI.Tests/
         ├── Application/
         │  └── Features/
         │     └── Templates/
         ├── Domain/
         │  └── Features/
         │     └── Templates/
         └── Cli/
```

**Structure Decision**: Keep template metadata rules and catalog validation in the existing Domain/Application layers, and keep user interaction, terminal detection, and workspace-creation orchestration in the CLI presentation layer. This preserves clean ownership boundaries already present in `src/nfw` and avoids pushing console-specific behavior into core packages.

## Planning Decisions

1. The stable user-facing selector will be the template identifier, while display-friendly names remain separate metadata for listing and prompt presentation.
2. Listing order and interactive prompt order will both follow the validated catalog sequence so users see one consistent order everywhere.
3. Interactive prompting will happen only after terminal interactivity is confirmed; otherwise `nfw new` must require an explicit `--template`.
4. Missing or unknown template identifiers will be treated as usage errors, not runtime failures, because the user can correct them without changing system state.
5. Template resolution will finish before workspace generation starts so cancellation and validation errors never leave partial output behind.
6. Deterministic validation includes explicit tests for listing order, duplicate identifiers, case-insensitive identifier handling, and non-interactive failure behavior.
7. Repository-root wrapper/documentation updates are part of delivery for this CLI surface, even though the core implementation remains in `src/nfw`.

## Complexity Tracking

No constitution violations require justification for this feature.

## Phase 0: Research Output

Artifacts:

- `/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/research.md`

Research resolves the design choices for stable identifiers, catalog ordering, terminal interactivity rules, selection error semantics, and deterministic test strategy.

## Phase 1: Design & Contracts Output

Artifacts:

- `/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/data-model.md`
- `/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/contracts/cli.md`
- `/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/quickstart.md`

### Post-Design Constitution Check (Re-evaluation)

- **Single-Step Build And Test**: PASS. The design requires both the `src/nfw/Makefile` entrypoints and repository-root wrapper/documentation support for build and test.
- **CLI I/O And Exit Codes**: PASS. The CLI contract specifies stdout for normal listing/prompt flow, stderr for validation/runtime errors, usage exit code `2` for template-selection mistakes, and `130` for interruption.
- **No Suppression**: PASS. The design keeps existing warnings-as-errors and test expectations intact.
- **Deterministic Tests**: PASS. The data model and contract assume fixture-driven catalog parsing plus fake prompt/interactivity collaborators in tests.
- **Documentation Is Part Of Delivery**: PASS. The plan, contract, and quickstart all describe the same selection behavior without contradictions.

## Phase 2: Planning Notes (Stop Point)

This plan stops before generating `tasks.md`. The next step should translate the design into implementation work in five slices:

1. Evolve the template domain/application model to represent stable identifiers, display metadata, ordering, and duplicate detection.
2. Extend `nfw templates` to present the richer catalog contract while preserving deterministic output.
3. Implement the `nfw new` template-resolution path, including terminal interactivity detection, explicit identifier validation, and interruption-safe behavior.
4. Add deterministic unit and CLI tests covering listing order, duplicate rejection, case-insensitive identifiers, explicit selection, interactive prompting, and non-interactive failures.
5. Add repository-root wrapper/documentation updates so build and test remain runnable from the repository root.
