# Tasks: Template Catalog Listing and Selection

**Spec Type**: Project-Based  
**Project**: nfw  
**Input**: Design documents from [`/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/`](/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/)  
**Prerequisites**: `plan.md`, `spec.md`, `research.md`, `data-model.md`, `contracts/cli.md`, `quickstart.md`

**Tests**: Deterministic test tasks are included because the plan and constitution require explicit validation for template ordering, identifier resolution, interactive behavior, and non-interactive failure paths.

**Organization**: Tasks are grouped by user story so each story can be implemented and validated independently.

## Format: `[ID] [P?] [Story] Description`

- **[P]**: Can run in parallel (different files, no dependencies)
- **[Story]**: Which user story this task belongs to (`[US1]`, `[US2]`, `[US3]`)
- Every task includes an exact file path

## Phase 1: Setup (Shared Infrastructure)

**Purpose**: Create the command surface and root-level developer entrypoints required before feature behavior is added.

- [x] T001 Create the workspace-creation command scaffold in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/New/NewCliCommand.cs`
- [x] T002 [P] Create command settings for workspace creation in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/New/NewCliCommandSettings.cs`
- [x] T003 Register the `new` command, help text, and examples in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs`
- [x] T004 [P] Add the repository-root build wrapper for the CLI workflow in `/home/ac/Code/n-framework/n-framework/scripts/build.sh`
- [x] T005 [P] Add the repository-root test wrapper for the CLI workflow in `/home/ac/Code/n-framework/n-framework/scripts/test.sh`
- [x] T006 Document the repository-root build/test entrypoints for this CLI surface in `/home/ac/Code/n-framework/n-framework/README.md`

**Checkpoint**: The repository has a recognized `nfw new` command path and documented root-level build/test entrypoints for the feature workflow.

---

## Phase 2: Foundational (Blocking Prerequisites)

**Purpose**: Build the shared template-selection model, validation rules, and deterministic test harness used by every story.

**⚠️ CRITICAL**: No user story work should begin until this phase is complete.

- [x] T007 Extend the template domain model with stable identifier and display metadata in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Domain/Features/Templates/TemplateDescriptor.cs`
- [x] T008 [P] Add the ordered template catalog model in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Domain/Features/Templates/TemplateCatalog.cs`
- [x] T009 Update YAML parsing to populate identifiers, display names, deterministic order, and duplicate-rejection behavior in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateCatalogParser.cs`
- [x] T010 [P] Add the template selection request model in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateSelectionRequest.cs`
- [x] T011 [P] Add the template selection result model in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateSelectionResult.cs`
- [x] T012 [P] Add the terminal interactivity abstraction in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Cli/Terminal/ITerminalSession.cs`
- [x] T013 Implement the template selection service skeleton with empty-catalog, unavailable-template, and case-normalization hooks in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateSelectionService.cs`
- [x] T014 Register template-selection services and terminal abstractions in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/ApplicationServiceRegistration.cs`
- [x] T015 [P] Expand parser coverage for deterministic order and duplicate identifiers in `/home/ac/Code/n-framework/n-framework/src/nfw/tests/unit/NFramework.NFW/presentation/NFramework.NFW.CLI/core/NFramework.NFW.Application/Features/Templates/TemplateCatalogParserTests.cs`
- [x] T016 [P] Add selection-service coverage for empty catalogs and case-insensitive identifier resolution in `/home/ac/Code/n-framework/n-framework/src/nfw/tests/unit/NFramework.NFW/presentation/NFramework.NFW.CLI/core/NFramework.NFW.Application/Features/Templates/TemplateSelectionServiceTests.cs`

**Checkpoint**: The codebase has a shared catalog/selection model plus deterministic tests for the foundational parsing and identifier rules.

---

## Phase 3: User Story 1 - Review Available Templates (Priority: P1) 🎯 MVP

**Goal**: Make `nfw templates` show stable identifiers plus readable metadata in a deterministic order.

**Independent Test**: Run `nfw templates` repeatedly and confirm the command shows every available template with identifier, display name, and description in the same order each time.

- [x] T017 [US1] Update template retrieval to return the validated ordered catalog for listing in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplatesService.cs`
- [x] T018 [US1] Render stable identifiers, display names, and descriptions in deterministic order in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/Templates/TemplatesCliCommand.cs`
- [x] T019 [US1] Add CLI coverage for template listing output and deterministic ordering in `/home/ac/Code/n-framework/n-framework/src/nfw/tests/unit/NFramework.NFW/presentation/NFramework.NFW.CLI/presentation/NFramework.NFW.CLI/Features/Templates/TemplatesCliCommandTests.cs`
- [x] T020 [US1] Document stable template identifiers and template-listing verification in `/home/ac/Code/n-framework/n-framework/src/nfw/README.md`

**Checkpoint**: User Story 1 is complete when template discovery works independently and its deterministic behavior is validated by tests and documentation.

---

## Phase 4: User Story 2 - Choose a Template Interactively (Priority: P2)

**Goal**: Let interactive terminal users complete `nfw new` by supplying any missing required input, including template choice, through prompts.

**Independent Test**: Run `nfw new` in a real terminal with no workspace name and no `--template` and confirm the CLI prompts for the missing values, uses the chosen identifier, and leaves no partial output when cancelled.

- [x] T021 [P] [US2] Implement the console-backed terminal session in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/New/CliTerminalSession.cs`
- [x] T022 [P] [US2] Create the interactive template prompt renderer in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/New/InteractiveTemplatePrompt.cs`
- [x] T023 [US2] Implement interactive-selection resolution and cancellation handling in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateSelectionService.cs`
- [x] T024 [US2] Wire prompt-driven template selection and cancellation-safe command flow in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/New/NewCliCommand.cs`
- [x] T025 [US2] Register terminal-session dependencies for the `new` command in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Program.cs`
- [x] T026 [US2] Add interactive prompt and cancellation coverage in `/home/ac/Code/n-framework/n-framework/src/nfw/tests/unit/NFramework.NFW/presentation/NFramework.NFW.CLI/presentation/NFramework.NFW.CLI/Features/New/NewCliCommandInteractiveTests.cs`
- [x] T027 [US2] Document interactive template selection behavior in `/home/ac/Code/n-framework/n-framework/src/nfw/README.md`

**Checkpoint**: User Story 2 is complete when interactive selection works, cancellation is safe, and both behaviors are documented and validated.

---

## Phase 5: User Story 3 - Use Templates Reliably In Automation (Priority: P3)

**Goal**: Require explicit template identifiers for non-interactive execution and fail early on unknown, unavailable, or missing identifiers.

**Independent Test**: Run `nfw new --no-input <workspace-name> --template <identifier>` in a non-interactive context and confirm it succeeds without prompting; run the same command with missing required inputs and confirm it exits with usage guidance before creating files.

- [x] T028 [US3] Add the explicit `--template` option and mode guidance in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/New/NewCliCommandSettings.cs`
- [x] T029 [US3] Implement explicit identifier resolution, canonical identifier messaging, unavailable-template handling, and non-interactive failure rules in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateSelectionService.cs`
- [x] T030 [US3] Enforce resolve-before-generate behavior for explicit and non-interactive flows in `/home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/New/NewCliCommand.cs`
- [x] T031 [US3] Add explicit-selection and non-interactive failure coverage in `/home/ac/Code/n-framework/n-framework/src/nfw/tests/unit/NFramework.NFW/presentation/NFramework.NFW.CLI/presentation/NFramework.NFW.CLI/Features/New/NewCliCommandNonInteractiveTests.cs`
- [x] T032 [US3] Document explicit non-interactive template selection and canonical identifier behavior in `/home/ac/Code/n-framework/n-framework/src/nfw/README.md`

**Checkpoint**: User Story 3 is complete when automation can rely on explicit identifiers and all invalid input paths fail before file creation with tested behavior.

---

## Phase 6: Polish & Cross-Cutting Concerns

**Purpose**: Reconcile the final contracts and execute the documented validation paths.

- [x] T033 Update the final listing/selection contract to match implemented CLI behavior in `/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/contracts/cli.md`
- [x] T034 Update the manual verification and repository-root execution steps in `/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/quickstart.md`
- [x] T035 [P] Execute repository-root build verification using `/home/ac/Code/n-framework/n-framework/scripts/build.sh`
- [x] T036 [P] Execute repository-root test verification using `/home/ac/Code/n-framework/n-framework/scripts/test.sh`
- [x] T037 Execute end-to-end verification using `/home/ac/Code/n-framework/n-framework/src/nfw/specs/002-nfw-template-catalog-selection/quickstart.md`

---

## Dependencies & Execution Order

### Phase Dependencies

- **Phase 1: Setup**: No dependencies, starts immediately.
- **Phase 2: Foundational**: Depends on Phase 1 and blocks all user stories.
- **Phase 3: US1**: Depends on Phase 2 only. This is the recommended MVP.
- **Phase 4: US2**: Depends on Phase 2 and Phase 3 because the interactive prompt must present the same listing metadata users already see in `nfw templates`.
- **Phase 5: US3**: Depends on Phase 2 and follows Phase 4 because `TemplateSelectionService.cs` and `NewCliCommand.cs` are shared implementation files that should not be edited concurrently.
- **Phase 6: Polish**: Depends on all selected user stories being complete.

### User Story Dependency Graph

`Setup` -> `Foundational` -> `US1` -> `US2` -> `US3` -> `Polish`

### Within Each User Story

- Deterministic validation tasks run alongside or immediately after behavior changes in the same story.
- Shared model and service updates come before CLI command wiring.
- Documentation updates happen after the corresponding behavior is stable.
- Repository-root verification runs only after story work is complete.

---

## Parallel Execution Examples

### Foundational Phase

```bash
Task: "Add the ordered template catalog model in /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Domain/Features/Templates/TemplateCatalog.cs"
Task: "Add the template selection request model in /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateSelectionRequest.cs"
Task: "Add the template selection result model in /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Templates/TemplateSelectionResult.cs"
Task: "Add the terminal interactivity abstraction in /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/core/NFramework.NFW.Application/Features/Cli/Terminal/ITerminalSession.cs"
```

### User Story 2

```bash
Task: "Implement the console-backed terminal session in /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/New/CliTerminalSession.cs"
Task: "Create the interactive template prompt renderer in /home/ac/Code/n-framework/n-framework/src/nfw/src/NFramework.NFW/presentation/NFramework.NFW.CLI/Features/New/InteractiveTemplatePrompt.cs"
```

### Polish Phase

```bash
Task: "Execute repository-root build verification using /home/ac/Code/n-framework/n-framework/scripts/build.sh"
Task: "Execute repository-root test verification using /home/ac/Code/n-framework/n-framework/scripts/test.sh"
```

---

## Implementation Strategy

### MVP First

1. Complete Phase 1 and Phase 2.
2. Complete Phase 3 (User Story 1) only.
3. Validate `nfw templates` through the story-specific tests and repository-root build path.
4. Stop and review before expanding into `nfw new`.

### Incremental Delivery

1. Deliver template listing with stable identifiers and deterministic validation.
2. Add interactive selection for human-driven workspace creation.
3. Add explicit non-interactive selection and early failure semantics for automation.
4. Finish with contract alignment, quickstart updates, and repository-root verification.

### Parallel Team Strategy

1. One engineer completes Setup and Foundational tasks.
2. After Foundation:
   - Engineer A takes US1 listing output and its tests.
   - Engineer B takes the file-independent US2 prompt infrastructure (`CliTerminalSession.cs`, `InteractiveTemplatePrompt.cs`).
3. Shared-file work in `TemplateSelectionService.cs`, `NewCliCommand.cs`, and `Program.cs` is serialized in story order to avoid edit conflicts.
4. One engineer closes Polish tasks and verification.

---

## Notes

- All tasks use the required checklist format: checkbox, task ID, optional `[P]`, required story label for story phases, and exact file path.
- User Story 1 is the suggested MVP because it delivers value without requiring workspace generation to exist end to end.
- Root-level wrapper and documentation tasks are included to satisfy the constitution requirement that build/test commands remain runnable from the repository root.
