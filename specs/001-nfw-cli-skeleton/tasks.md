# Tasks: nfw CLI Skeleton

**Feature**: nfw CLI Skeleton  
**Branch**: `001-nfw-cli-skeleton`  
**Docs**: `spec.md`, `plan.md`, `research.md`, `data-model.md`, `contracts/cli.md`, `quickstart.md`

---

## Phase 1: Setup (Shared Infrastructure)

**Goal**: Establish the solution/project scaffolding and developer workflow so implementation work is straightforward and repeatable.

- [x] T001 Create project directory skeleton under `src/nfw/src/NFramework.NFW/` and `src/nfw/tests/NFramework.NFW.CLI.Tests/`
- [x] T002 [P] Create `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/NFramework.NFW.Application.csproj` targeting .NET 11
- [x] T003 [P] Create `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Domain/NFramework.NFW.Domain.csproj` targeting .NET 11
- [x] T004 [P] Create `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/NFramework.NFW.CLI.csproj` targeting .NET 11 with Spectre.Console.Cli + YAML + DI refs
- [x] T005 [P] Create `src/nfw/tests/NFramework.NFW.CLI.Tests/NFramework.NFW.CLI.Tests.csproj` targeting .NET 11 (xUnit)
- [x] T006 Add git submodules in `src/nfw/.gitmodules` for `src/nfw/packages/n-framework-core-codegen` and `src/nfw/packages/n-framework-nfw-templates`
- [x] T007 Update `src/nfw/NFramework.Nfw.slnx` to include all created projects and the codegen project at `src/nfw/packages/n-framework-core-codegen/src/NFramework.NFW.Core.CodeGen/NFramework.NFW.Core.CodeGen.csproj`
- [x] T008 Add dotnet tool manifest at `src/nfw/.config/dotnet-tools.json` (csharpier + roslynator)
- [x] T009 Add shared build settings in `src/nfw/Directory.Build.props` (nullable, implicit usings, warnings-as-errors baseline)
- [x] T010 Create `src/nfw/Makefile` with single-step commands: `build`, `test`, `format`, `analyze`
- [x] T011 Update `src/nfw/README.md` with build/test/format/analyze commands from `Makefile`
- [x] T012 Document templates debug vs production sourcing in `src/nfw/README.md` (submodule vs GitHub tag matching CLI version)

**Checkpoint**: `make build` and `make test` are available (even if tests are initially empty).

---

## Phase 2: Foundational (Blocking Prerequisites)

**Goal**: Implement shared CLI infrastructure (exit codes, config, logging, wiring) that all user stories depend on.

- [x] T013 Create exit code constants in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Cli/ExitCodes.cs` (0, 1, 2, 130)
- [x] T014 Implement configuration loader in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Cli/Configuration/NfwConfigurationLoader.cs` (missing file is OK; parse failure prints error and continues with defaults; env overrides win)
- [x] T015 Implement diagnostic logger in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Cli/Logging/DiagnosticLogger.cs` (stderr only, gated by `--verbose`)
- [x] T016 Implement version provider in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Versioning/VersionProvider.cs` (semver + optional metadata)
- [x] T017 Wire application services in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/ApplicationServiceRegistration.cs` (config loader, template catalog, version provider)
- [x] T018 Create CLI DI adapter in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/IoC/TypeRegistrar.cs` (Spectre.Console.Cli + Microsoft DI)
- [x] T019 Create CLI DI adapter in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/IoC/TypeResolver.cs` (Spectre.Console.Cli + Microsoft DI)
- [x] T020 Add CLI bootstrap skeleton in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs` (DI container, CommandApp, encoding)
- [x] T021 Implement global argument pre-parse in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs` for `--verbose`, `--help`/`-h`, `--version`/`-v` precedence
- [x] T022 Implement SIGINT (Ctrl+C) handling in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs` (exit 130)
- [x] T023 Implement required configuration validation framework in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Cli/Configuration/RequiredConfigurationValidator.cs` (skeleton: no required keys)

**Checkpoint**: The CLI starts and returns correct exit codes for “usage error” parsing paths (unknown flags/commands) even before full feature behavior is implemented.

---

## Phase 3: User Story 1 - Discover CLI Capabilities (Priority: P1) 🎯 MVP

**Goal**: `nfw --help` (and `nfw` with no args) shows comprehensive, readable help for available commands.

**Independent Test**: Run `nfw --help`, `nfw -h`, and `nfw` and verify: commands are listed, usage shown, output fits 80 columns, and subcommand help works.

- [x] T024 [US1] Register root command descriptions/examples in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs`
- [x] T025 [P] [US1] Add `templates` command stub in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/Templates/TemplatesCliCommand.cs`
- [x] T026 [P] [US1] Add `templates` command settings in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/Templates/TemplatesCliCommandSettings.cs`
- [x] T027 [US1] Ensure `nfw` (no args) displays help in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs`
- [x] T028 [US1] Ensure subcommand help works (`nfw templates --help`) in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs`
- [x] T029 [US1] Enforce 80-column readability (wrap/format) in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs`

---

## Phase 4: User Story 2 - Verify CLI Version (Priority: P2)

**Goal**: `nfw --version` and `nfw -v` print semantic version (and optional build metadata).

**Independent Test**: Run `nfw --version` and `nfw -v`; confirm semver format and consistent output.

- [x] T030 [US2] Implement `--version`/`-v` output path in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs` using `VersionProvider`
- [x] T031 [US2] Ensure `--help` wins over `--version` when both provided in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs`

---

## Phase 5: User Story 3 - List Available Templates (Priority: P3)

**Goal**: `nfw templates` lists templates (Debug builds: local submodule if present; Release builds: remote tag `v{cliVersion}`) and handles unreachable remote with a clear error and exit code 1.

**Independent Test**: Run `nfw templates` and verify: Debug build uses submodule when present; Release build fetches remote by tag; unreachable remote yields a clear error and exit code 1; empty catalog is not an error.

- [x] T032 [P] [US3] Define template catalog schema and parsing in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateCatalogParser.cs`
- [x] T033 [P] [US3] Implement local submodule catalog reader in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/LocalTemplatesSubmoduleReader.cs` (read from `src/nfw/packages/n-framework-nfw-templates`)
- [x] T034 [P] [US3] Implement GitHub release-tag catalog client in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/GitHubTemplatesReleaseClient.cs` (tag `v{cliVersion}`)
- [x] T035 [US3] Implement template source selection in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateCatalogSourceResolver.cs` (Debug uses submodule if present; Release uses GitHub tag)
- [x] T036 [US3] Implement templates service in `src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplatesService.cs` (select source + map to descriptors)
- [x] T037 [US3] Implement `templates` command execution in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/Templates/TemplatesCliCommand.cs` (table output, empty message, error handling)
- [x] T038 [US3] Ensure unreachable remote exits 1 and prints error to stderr in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/Templates/TemplatesCliCommand.cs`
- [x] T039 [US3] Update contract doc with final catalog schema notes in `src/nfw/specs/001-nfw-cli-skeleton/contracts/cli.md`

---

## Phase 6: Polish & Cross-Cutting Concerns

**Goal**: Make the CLI robust, consistent, and shippable.

- [x] T040 [P] Add friendly unknown-command suggestions in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs` (exit 2)
- [x] T041 Add verbose diagnostics coverage (stderr only) in `src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs`
- [x] T042 Document configuration behavior in `src/nfw/README.md` (YAML + env overrides, parse failure continues with defaults)
- [x] T043 Document exit codes in `src/nfw/README.md` (0/1/2/130)
- [x] T044 Add publish instructions for single-binary distribution in `src/nfw/README.md` (`dotnet publish -c Release` for `linux-x64`, `osx-x64`, `win-x64` + `PublishSingleFile`)
- [x] T045 Add a lightweight perf measurement script in `src/nfw/scripts/perf/measure-help-startup.sh` (baseline SC-001/SC-004)
- [x] T046 Run `quickstart.md` end-to-end and update it if any commands/paths changed in `src/nfw/specs/001-nfw-cli-skeleton/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- Phase 1 (Setup) blocks everything else.
- Phase 2 (Foundational) blocks all user stories.
- US1 (Help) can proceed as soon as Phase 2 is complete.
- US2 (Version) can proceed after Phase 2 and is independent of US1.
- US3 (Templates) can proceed after Phase 2 and is independent of US1/US2 (except shared CLI wiring).

### User Story Dependency Graph

`Phase 1` → `Phase 2` → (`US1` || `US2` || `US3`) → `Polish`

---

## Parallel Execution Examples

### Setup Phase (Parallelizable)

- T002, T003, T004, T005 can be done in parallel (separate project files).
- T007, T008, T009 can be done in parallel (separate tooling/config files).

### After Foundation (Parallelizable)

- US1 tasks T025 and T026 can be done in parallel (separate files).
- US2 task T030 can be done in parallel with US1 tasks (separate layer/files).
- US3 tasks T032 and T033 can be done in parallel (parser vs local reader); T034 can be parallelized as the remote client.

---

## Implementation Strategy

### MVP Scope (Recommended)

Ship **US1 only** first: help output + command discovery is the onboarding gate for everything else.

### Incremental Delivery

1. Phase 1 + Phase 2: CLI boots reliably, config and exit codes are consistent.
2. US1: Help is correct and readable.
3. US2: Version reporting is correct.
4. US3: Remote templates listing is correct and resilient to failures.
5. Polish: Documentation, suggestions, and publish instructions.
