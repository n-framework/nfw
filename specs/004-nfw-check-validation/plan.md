# Implementation Plan: `nfw check` Architecture Validation

**Branch**: `004-nfw-check-validation` | **Date**: 2026-04-05 | **Spec**: `/home/ac/Code/n-framework/src/nfw/specs/004-nfw-check-validation/spec.md`
**Input**: Feature specification from `/src/nfw/specs/004-nfw-check-validation/spec.md`

## Summary

Implement `nfw check` as a non-interactive architecture audit command that scans workspace project references, namespace usage, and direct package references; reports all findings with remediation hints; and exits non-zero on violations. Delivery includes fixture-backed validation covering valid and invalid cases for each violation class plus unreadable artifact handling.

## Technical Context

**Language/Version**: Rust 1.85+ (2024 edition)  
**Primary Dependencies**: clap (CLI command wiring), serde/serde_yaml (workspace/rule config parsing), regex (namespace/package pattern checks)  
**Storage**: File system workspace artifacts and project/source files  
**Testing**: cargo test (workspace/unit+integration in `src/nfw`)  
**Target Platform**: Cross-platform CLI (Linux/macOS/Windows terminals; CI runners)  
**Project Type**: CLI application (module: `src/nfw`)  
**Performance Goals**: Complete validation in non-interactive mode with deterministic results across repeated runs on the same workspace inputs  
**Constraints**: Non-interactive execution, stable exit behavior, actionable stderr diagnostics, deterministic fixture results  
**Scale/Scope**: Validate architecture rules across all projects in one workspace; support multi-violation reporting in a single run

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

- Single-step build/test: PASS (`make build`/`make test` at module level already defined)
- CLI I/O and exit codes: PASS (plan enforces stable 0/non-zero and SIGINT behavior)
- No suppression: PASS (no warning suppression or skipped tests introduced)
- Deterministic tests: PASS (fixture-based local tests, no real network dependency)
- Documentation as delivery: PASS (quickstart and contract artifacts included)

Post-Phase-1 re-check: PASS (research, data model, contract, and quickstart preserve all gates).

## Project Structure

### Documentation (this feature)

```text
src/nfw/specs/004-nfw-check-validation/
├── plan.md
├── research.md
├── data-model.md
├── quickstart.md
├── contracts/
│   └── nfw-check-cli-contract.md
└── tasks.md
```

### Source Code (repository root)

```text
src/nfw/
├── src/
│   ├── ... existing CLI command routing and check command implementation files
│   └── ... architecture validation rule/evaluator components
└── tests/
    ├── integration/
    │   └── ... nfw check fixture-driven integration tests
    └── unit/
        └── ... rule parsing and finding aggregation tests
```

**Structure Decision**: Keep implementation in existing `src/nfw` CLI module and add fixture-driven tests in `src/nfw/tests` to match current repository/module boundaries.

## Phase Plan

### Phase 0: Research
- Finalize validation policy for unreadable artifacts and dependency scope (resolved in `research.md`).
- Confirm single-run multi-finding reporting strategy and deterministic exit semantics.

### Phase 1: Design & Contracts
- Define entities and rule/finding relationships in `data-model.md`.
- Define command/exit/output contract in `contracts/nfw-check-cli-contract.md`.
- Publish executable usage expectations in `quickstart.md`.
- Update agent context for current technology surface.

### Phase 2: Task Planning (for `/speckit.tasks`)
- Break implementation into command wiring, rule evaluation, output formatting, and fixture test tasks.
- Sequence tasks to deliver failing fixture tests first, then command behavior, then output polish.
- Include verification tasks for exit codes, unreadable artifact behavior, and direct-only package checks.

## Complexity Tracking

No constitution violations requiring justification.
