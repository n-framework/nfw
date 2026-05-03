# Tasks: Template Metadata Schema, Discovery, and Versioning

**Feature**: Template Metadata Schema, Discovery, and Versioning
**Branch**: `001-nfw-template-system`
**Created**: 2026-03-29

## Context

This feature implements the NFW template system following **Clean Architecture** with feature-based organization. The architecture mirrors the existing nfw-rust-rewrite project structure.

**Tech Stack**: Rust 1.85+ (2024 edition), serde, serde_yaml, semver, regex, dirs, clap
**Naming Convention**: project/crate/repo/folder names use kebab-case; Rust source modules/files use snake_case; Rust types use PascalCase; functions/variables use snake_case.

**Repository Structure**:

```bash
github.com/n-framework/
├── core-cli-rust                    # CLI terminal abstractions (separate repo)
├── core-template-rust        # Template rendering engine (separate repo)
└── nfw/                             # This repo - NFW CLI
```

**Clean Architecture (this repo)**:

```bash
src/nfw/src/nframework-nfw/
├── core/
│   ├── nframework-nfw-domain/              # Domain: entities, value objects (ZERO external deps)
│   │   └── src/features/
│   │       ├── template_management/
│   │       │   ├── language.rs
│   │       │   ├── template_metadata.rs
│   │       │   ├── template_source.rs
│   │       │   └── template_catalog.rs
│   │       └── versioning/
│   │           └── version_info.rs
│   │
│   └── nframework-nfw-application/         # Application: use cases, services, trait abstractions
│       └── src/features/
│           ├── cli/
│           │   ├── configuration/
│           │   │   ├── abstraction/       # INfwConfigurationLoader trait
│           │   │   └── nfw_configuration.rs
│           │   └── exit_codes.rs
│           ├── template_management/
│           │   ├── models/                 # DTOs (ListedTemplate, etc.)
│           │   ├── queries/
│           │   │   └── list_templates/
│           │   ├── services/
│           │   │   ├── abstraction/       # YamlParser, ConfigStore, PathResolver, Validator traits
│           │   │   ├── template_registry.rs
│           │   │   ├── template_catalog_parser.rs
│           │   │   ├── template_catalog_source_resolver.rs
│           │   │   └── template_selection_service.rs
│           │   └── template_rendering/
│           │       └── abstraction/
│           └── versioning/
│               ├── abstraction/           # VersionComparator trait (nfw internal)
│               └── version_resolver.rs
│
├── infrastructure/
│   ├── nframework-nfw-infrastructure-filesystem/
│   │   └── src/features/
│   │       ├── cli/configuration/          # NfwConfigurationLoader impl
│   │       └── template_management/services/  # FileConfigStore, LocalTemplateSource
│   ├── nframework-nfw-infrastructure-git/
│   │   └── src/features/
│   │       └── template_management/services/  # CliGitRepository adapter
│   ├── nframework-nfw-infrastructure-yaml/
│   │   └── src/features/
│   │       └── template_management/services/  # SerdeYamlParser adapter
│   └── nframework-nfw-infrastructure-versioning/
│       └── src/features/
│           └── versioning/services/        # SemverVersionComparator adapter
│
└── presentation/
    └── n-framework-nfw-cli/
        └── src/
            ├── main.rs
            ├── args.rs
            └── commands/
                └── templates/              # list, add, remove, refresh commands
```

**Tests** (mirrors source structure):

```bash
src/nfw/tests/unit/nframework-nfw/
├── core/
│   ├── nframework-nfw-domain/
│   └── nframework-nfw-application/
└── infrastructure/
    ├── nframework-nfw-infrastructure-filesystem/
    ├── nframework-nfw-infrastructure-git/
    ├── nframework-nfw-infrastructure-yaml/
    └── nframework-nfw-infrastructure-versioning/
```

**Dependency Flow**:

```text
nfw CLI (presentation)
  ↓ depends on
nframework-nfw-application + infrastructure_*
  ↓ depends on
nframework-nfw-domain + core-cli-rust + core-template-rust
```

---

## Phase 1: Setup

Project initialization per clean architecture structure.

- [x] T001 Initialize nfw-internal versioning skeleton under `src/nfw/src/nframework-nfw/core/.../features/versioning/`
- [x] T002 Initialize nfw-internal git abstraction/adapters skeleton under application + infrastructure git crates
- [x] T003 Initialize `core-cli-rust` repo: create `Cargo.toml`, `src/lib.rs`, `tests/` directory
- [x] T004 Initialize `core-template-rust` repo: create `Cargo.toml`, `src/lib.rs`, `tests/` directory
- [x] T005 Create nfw workspace `src/nfw/Cargo.toml` with members for each clean architecture crate and path dependencies on external cores still used (`core-cli-rust`, `core-template-rust`)
- [x] T006 Create clean architecture directory structure:
      `src/nfw/src/nframework-nfw/core/nframework-nfw-domain/src/features/`
      `src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/`
      `src/nfw/src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-{filesystem,git,yaml,versioning}/src/features/`
      `src/nfw/src/nframework-nfw/presentation/n-framework-nfw-cli/src/`
      `src/nfw/tests/unit/nframework-nfw/core/`
      `src/nfw/tests/unit/nframework-nfw/infrastructure/`

---

## Phase 2: Foundational

Blocking prerequisites. Internal nfw foundations + domain layer + application abstractions.

### Foundation (nfw internal + external cores still used)

- [x] T007 [P] Define `Version` struct in `src/nfw/src/nframework-nfw/core/nframework-nfw-domain/src/features/versioning/version.rs` (major, minor, patch, pre, build - plain Rust, no deps)
- [x] T008 [P] Implement `Display`, `FromStr`, `is_stable()` for `Version` in `src/nfw/src/nframework-nfw/core/nframework-nfw-domain/src/features/versioning/version.rs`
- [x] T009 [P] Define `VersionComparator` trait in `src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/versioning/abstraction/version_comparator.rs` (parse, compare, is_stable, satisfies)
- [x] T010 Implement `SemverVersionComparator` in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-versioning/src/features/versioning/services/semver_version_comparator.rs` using semver crate (depends on T009)
- [x] T011 [P] Define `GitRepository` trait in `src/nfw/src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/services/abstraction/git_repository.rs` (clone, fetch, current_branch, is_valid_repo)
- [x] T012 [P] Implement `CliGitRepository` in `src/nfw/src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-git/src/features/template_management/services/cli_git_repository.rs` using `std::process::Command` (no git2)
- [x] T013 [P] Define `Command` trait and `CliAdapter` trait in `src/core-cli-rust/src/nframework-core-cli-abstraction/abstraction/{command.rs,cli_adapter.rs}`
- [x] T014 [P] Implement `ClapAdapter` in `src/core-cli-rust/src/nframework-core-cli-clap/clap_adapter.rs` using clap crate

### Domain Layer (nframework-nfw-domain)

- [x] T017 [P] Define `VersionInfo` value object in `src/nframework-nfw/core/nframework-nfw-domain/src/features/versioning/version_info.rs`
- [x] T018 [P] Define `TemplateMetadata` entity in `src/nframework-nfw/core/nframework-nfw-domain/src/features/template_management/template_metadata.rs` (id, name, description, version, language, tags, author, min_cli_version, source_url)
- [x] T019 [P] Define `TemplateSource` entity in `src/nframework-nfw/core/nframework-nfw-domain/src/features/template_management/template_source.rs` (name, url, enabled)
- [x] T020 [P] Define `Language` enum in `src/nframework-nfw/core/nframework-nfw-domain/src/features/template_management/language.rs` (Dotnet, Go, Rust)

### Application Layer - Abstractions (nframework-nfw-application)

- [x] T021 [P] Define `YamlParser` trait in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/services/abstraction/yaml_parser.rs` (parse, serialize)
- [x] T022 [P] Define `ConfigStore` trait in `src/nframework-nfw/core/nframework-nfw-application/src/features/cli/configuration/abstraction/config_store.rs` (load_sources, save_sources)
- [x] T023 [P] Define `PathResolver` trait in `src/nframework-nfw/core/nframework-nfw-application/src/features/cli/configuration/abstraction/path_resolver.rs` (cache_dir, config_dir)
- [x] T024 [P] Define `Validator` trait in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/services/abstraction/validator.rs` (is_kebab_case, is_git_url)
- [x] T025 Define `NfwConfiguration` struct in `src/nframework-nfw/core/nframework-nfw-application/src/features/cli/configuration/nfw_configuration.rs`
- [x] T026 Define `ExitCodes` in `src/nframework-nfw/core/nframework-nfw-application/src/features/cli/exit_codes.rs`

---

## Phase 3: User Story 1 - Define and Validate Template Metadata (P1)

**Goal**: Template authors can describe templates using a standard `template.yaml` metadata file that the CLI parses and validates.

**Independent Test**: Create a template directory with a `template.yaml` file and verify the CLI parses and validates all required and optional fields.

### Phase 3: Application Services

- [x] T027 [US1] Implement metadata validation in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/services/template_catalog_parser.rs` (validate id kebab-case, name/description non-empty, version semver, language supported)
- [x] T028 [US1] Define validation error types in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/models/errors/template_catalog_error.rs` with actionable messages per FR-007
- [x] T029 [US1] Define `ListedTemplate` DTO in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/models/listed_template.rs`

### Phase 3: Infrastructure Adapters

- [x] T030 [P] [US1] Implement `SerdeYamlParser` in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-yaml/src/features/template_management/services/serde_yaml_parser.rs` using serde_yaml

### Domain Tests

- [x] T031 [P] [US1] Write domain tests for TemplateMetadata validation in `src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-domain/features/template_management/template_metadata.tests.rs` (valid, missing fields, invalid values, optional fields)

### Phase 3: Application Tests

- [x] T032 [P] [US1] Write application tests for TemplateCatalogParser in `src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-application/features/template_management/services/template_catalog_parser.tests.rs` (parse valid YAML, invalid YAML, missing required fields)

---

## Phase 4: User Story 2 - Structure a Template Repository (P1)

**Goal**: Template authors have a defined repository layout so the CLI can reliably locate template files, content, and configuration.

**Independent Test**: Create a template repository following the defined layout and verify the CLI locates all expected directories and files.

### Phase 4: Application Services

- [x] T033 [US2] Implement `TemplateCatalogSourceResolver` in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/services/template_catalog_source_resolver.rs` (scan repo for subdirectories with template.yaml, build catalog)
- [x] T034 [US2] Define `TemplateCatalog` value object in `src/nframework-nfw/core/nframework-nfw-domain/src/features/template_management/template_catalog.rs` (source name, list of template descriptors)
- [x] T035 [US2] Define `TemplateDescriptor` value object in `src/nframework-nfw/core/nframework-nfw-domain/src/features/template_management/template_descriptor.rs` (metadata + cache_path)

### Phase 4: Infrastructure Adapters

- [x] T036 [US2] Implement `LocalTemplatesCatalogSource` in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/template_management/services/local_templates_catalog_source.rs` (scan content directory, apply .nfwignore exclusions)
- [x] T036b [P] [US2] Implement placeholder detection in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/template_management/services/placeholder_detector.rs` (scan file/directory names for `__PascalCase__` pattern per FR-012)

### Phase 4: Application Tests

- [x] T037 [P] [US2] Write tests for TemplateCatalogSourceResolver in `src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-application/features/template_management/services/template_catalog_source_resolver.tests.rs`

---

## Phase 5: User Story 3 - Discover Templates from Remote Git Repositories (P1)

**Goal**: The CLI discovers and caches templates from remote git repositories so users can access official and community templates without manual setup.

**Independent Test**: Configure a remote template source and verify the CLI clones, caches, and reads templates from it.

### Phase 5: Application Services

- [x] T038 [US3] Implement `TemplatesService` in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/services/templates_service.rs` (orchestrates discovery: iterate sources, clone/fetch, parse metadata, index)
- [x] T039 [US3] Implement `TemplateSelectionService` in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/services/template_selection_service.rs` (resolve template by ID, handle qualified/unqualified lookup)
- [x] T040 [US3] Define `TemplateSelectionResult` in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/services/template_selection_result.rs`
- [x] T041 [US3] Implement `ListTemplatesQueryHandler` in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/queries/list_templates/list_templates_query_handler.rs`

### Query/Command Models

- [x] T042 [P] [US3] Define `ListTemplatesQuery` in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/queries/list_templates/list_templates_query.rs`
- [x] T043 [P] [US3] Define `ListTemplatesQueryResult` in `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/queries/list_templates/list_templates_query_result.rs`

### Phase 5: Infrastructure Adapters

- [x] T044 [US3] Implement `GitTemplateCatalogSource` (git-based source) in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-git/src/features/template_management/services/git_template_catalog_source.rs` (clone to `~/.nfw/templates/`, incremental fetch, corruption detection)

### Edge Cases

- [x] T045 [US3] Handle unreachable sources in `TemplatesService` (fall back to cache, warn user)
- [x] T046 [US3] Handle conflicting template identifiers in `TemplateSelectionService` (qualified ID `source/template`, warn user)

### Phase 5: Tests

- [x] T047 [P] [US3] Write application tests for TemplatesService in `src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-application/features/template_management/services/templates_service.tests.rs` (mock git, single/multiple sources, empty source, unreachable source)
- [x] T048 [P] [US3] Write integration tests in `src/nfw/tests/integration/nframework-nfw/features/template_discovery/discovery_test.rs` (real git clone, cache refresh, multi-template catalog)

---

## Phase 6: User Story 4 - Version Templates Deterministically (P2)

**Goal**: Template versions follow a deterministic scheme so workspace generation is reproducible across machines and CI environments.

**Independent Test**: Create two versions of a template and verify the CLI resolves the correct version based on version constraints.

### Phase 6: Application Services

- [x] T049 [US4] Implement `VersionResolver` in `src/nframework-nfw/core/nframework-nfw-application/src/features/versioning/version_resolver.rs` (find latest stable, handle pre-release, resolve by constraint)
- [x] T050 [US4] Define `VersionProvider` abstraction in `src/nframework-nfw/core/nframework-nfw-application/src/features/versioning/abstraction/version_provider.rs`
- [x] T051 [US4] Implement `VersionProvider` in `src/nframework-nfw/core/nframework-nfw-application/src/features/versioning/version_provider.rs` (wraps nfw-internal VersionComparator)

### Phase 6: Tests

- [x] T052 [P] [US4] Write tests for VersionResolver in `src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-application/features/versioning/version_resolver.tests.rs` (latest stable, pre-release excluded, explicit version, minCliVersion warning)

---

## Phase 7: User Story 5 - Register and Manage Template Sources (P2)

**Goal**: Users can add and remove template sources to access templates from different providers.

**Independent Test**: Add a custom template source URL and verify its templates appear in `nfw templates` output.

### Domain

- [x] T053 [US5] Define `QualifiedTemplateId` value object in `src/nframework-nfw/core/nframework-nfw-domain/src/features/template_management/qualified_template_id.rs` (source, template, new, unqualified, is_qualified)

### Phase 7: Application Services

- [x] T054 [US5] Implement source management in `TemplatesService` (add_source: validate URL via GitRepository trait, persist to config; remove_source: clean up cache)
- [x] T055 [US5] Implement default official source initialization in `TemplatesService` (add `https://github.com/n-framework/nfw-templates` on first run)

### Phase 7: Infrastructure Adapters

- [x] T056 [US5] Implement `NfwConfigurationLoader` in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/cli/configuration/nfw_configuration_loader.rs` (read/write sources.yaml using serde_yaml)
- [x] T057 [US5] Implement `FileSystemWorkspaceArtifactWriter` in `src/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/src/features/template_management/services/file_system_config_store.rs` (persist source config to `~/.nfw/sources.yaml`)

### Phase 7: Tests

- [x] T058 [P] [US5] Write tests for source management in `src/nfw/tests/unit/nframework-nfw/core/nframework-nfw-application/features/template_management/services/source_management.tests.rs` (add, remove, duplicate, default init)
- [x] T059 [P] [US5] Write tests for NfwConfigurationLoader in `src/nfw/tests/unit/nframework-nfw/infrastructure/nframework-nfw-infrastructure-filesystem/features/cli/configuration/nfw_configuration_loader.tests.rs`

---

## Phase 8: Polish & Cross-Cutting Concerns

CLI presentation layer wiring and final integration.

### Presentation Layer (n-framework-nfw-cli)

- [x] T060 Set up CLI entry point in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/main.rs` with clap (parse args, route to commands)
- [x] T061 Define CLI args in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/args.rs` (template subcommands: list, add, remove, refresh)
- [x] T062 [P] Implement `TemplatesCliCommand` in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/templates/list_templates.rs` (calls ListTemplatesQueryHandler)
- [x] T063 [P] Implement `AddSourceCliCommand` in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/templates/add_source.rs`
- [x] T064 [P] Implement `RemoveSourceCliCommand` in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/templates/remove_source.rs`
- [x] T065 [P] Implement `RefreshTemplatesCliCommand` in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/commands/templates/refresh.rs`

### Dependency Injection / Wiring

- [x] T066 Create `CliServiceCollectionFactory` in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/startup/cli_service_collection_factory.rs` (wire domain → application → infrastructure → presentation)
- [x] T067 Create `CliBootstrapper` in `src/nframework-nfw/presentation/n-framework-nfw-cli/src/startup/cli_bootstrapper.rs` (initialize config, resolve path, load sources)

### Integration & Benchmarks

- [x] T068 [P] Write integration test for full template workflow in `src/nfw/tests/integration/nframework-nfw/features/template_discovery/e2e_test.rs` (add source → discover → list → resolve version)
- [x] T069 [P] Add benchmark for SC-001 (< 50ms metadata validation) in `src/nfw/tests/integration/nframework-nfw/benches/metadata_bench.rs`
- [x] T070 [P] Add benchmark for SC-003 (< 500ms listing 50 templates) in `src/nfw/tests/integration/nframework-nfw/benches/listing_bench.rs`
- [x] T071 [P] Write reproducibility test for SC-006 in `src/nfw/tests/integration/nframework-nfw/features/template_discovery/reproducibility_test.rs` (generate workspace twice from same template+version, verify identical file structures)

---

## Dependencies

```text
Phase 1: Setup
  ↓
Phase 2: Foundational (core packages + domain + application abstractions)
  ↓
Phase 3: US1 (Metadata) ──────────────┐  ← can run in parallel
Phase 4: US2 (Repository Structure) ──┘
  ↓
Phase 5: US3 (Discovery) ← depends on US1 + US2
  ↓
Phase 6: US4 (Versioning) ← depends on US1
Phase 7: US5 (Source Management) ← depends on US3
  ↓
Phase 8: Polish (presentation wiring)
```

## Parallel Execution

**Phase 2**: All core package tasks (T007-T014) run in parallel across repos. Domain entities (T017-T020) run in parallel. Application abstractions (T021-T024) run in parallel.

**Phase 3**: T030 (YAML adapter) and T031-T032 (tests) can run in parallel with T027-T029.

**Phase 4**: T037 (tests) can run in parallel.

**Phase 5**: T042-T043 (query models) and T047-T048 (tests) can run in parallel.

**Phase 8**: All CLI commands (T062-T065) and benchmarks (T069-T071) can run in parallel.

## Implementation Strategy

**MVP Scope**: Phases 1-3 (Setup + Foundational + US1 Metadata). Delivers a working metadata schema with validation. 30 tasks.

**Incremental Delivery**:

1. **MVP** (Phases 1-3): Template metadata schema, validation, YAML parsing
2. **v0.2** (Phase 4): Repository structure scanning
3. **v0.3** (Phase 5): Remote template discovery with git caching
4. **v0.4** (Phase 6): Deterministic version resolution
5. **v0.5** (Phase 7): Source management
6. **v1.0** (Phase 8): Full CLI integration, benchmarks

## Task Format Validation

All tasks follow the required checklist format:

- ✅ Checkbox: `- [ ]`
- ✅ Task ID: Sequential (T001-T071, with T015-T016 deferred to separate spec)
- ✅ [P] marker: On parallelizable tasks (different files, no blocking deps)
- ✅ [Story] label: On user story tasks ([US1]-[US5])
- ✅ Description: Clear action with exact file path following clean architecture
