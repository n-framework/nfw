# Feature Specification: Generator Metadata Schema, Discovery, and Versioning

**Naming Convention**: project/crate/repo/folder names use kebab-case; Rust source modules/files use snake_case; Rust types use PascalCase; functions/variables use snake_case.

## User Scenarios & Testing

### User Story 1 - Define and Validate Generator Metadata (Priority: P1)

As a generator author, I want to describe my generator using a standard metadata file so that the CLI can discover, validate, and present my generator to users without manual registration.

**Why this priority**: Generator metadata is the foundation of the entire generator system. Without a defined schema, no generator can be discovered, listed, or selected. All other generator features depend on this.

**Independent Test**: Can be fully tested by creating a generator directory with a metadata file and verifying the CLI can parse and validate it. Delivers immediate value: a machine-readable contract for generator authors.

**Acceptance Scenarios**:

1. **Given** a generator directory containing a metadata file, **When** the CLI reads the metadata, **Then** all required fields (identifier, name, description, version) are parsed successfully and `language` is parsed when present
2. **Given** a metadata file missing a required field, **When** the CLI validates the metadata, **Then** an actionable error identifies the missing field and the expected format
3. **Given** a metadata file with an invalid field value, **When** the CLI validates the metadata, **Then** an actionable error explains why the value is invalid and what values are acceptable
4. **Given** a valid metadata file, **When** the CLI reads the metadata, **Then** optional fields (tags, author, minimum CLI version) are parsed without errors when present and ignored when absent

---

### User Story 2 - Structure a Generator Repository (Priority: P1)

As a generator author, I want a defined repository layout so that my generator's files, generators, hooks, and configuration are organized predictably for the CLI to process.

**Why this priority**: The repository format is the second foundational piece. Without a defined layout, the CLI cannot reliably locate generator files, rendering metadata useless. Must ship alongside metadata schema.

**Independent Test**: Can be fully tested by creating a generator repository following the defined layout and verifying the CLI can locate all expected directories and files. Delivers immediate value: a reproducible structure for generator packaging.

**Acceptance Scenarios**:

1. **Given** a generator repository following the defined layout, **When** the CLI scans the repository, **Then** the content directory, metadata file, and configuration are located at their expected paths
2. **Given** a generator repository missing the content directory, **When** the CLI scans the repository, **Then** an actionable error identifies the missing directory and the expected structure
3. **Given** a generator repository with a valid content directory, **When** the CLI lists generator files, **Then** all source files, configuration files, and scaffolding generators are discoverable
4. **Given** a generator repository with a `.nfwignore` exclusion file, **When** the CLI processes the generator, **Then** excluded files are not included in generated output

---

### User Story 3 - Discover Generators from Remote Git Repositories (Priority: P1)

As a developer, I want the CLI to discover and cache generators from remote git repositories so that I can access official and community generators without manual setup.

**Why this priority**: Git-based discovery enables the distributed generator ecosystem described in the PRD. Official generators are distributed as git repositories (per orchestrator spec clarification). This is required for `nfw generators` to list anything beyond built-in defaults.

**Independent Test**: Can be fully tested by configuring a remote generator source and verifying the CLI clones, caches, and reads generators from it. Delivers immediate value: access to official and third-party generator catalogs.

**Acceptance Scenarios**:

1. **Given** a configured remote generator source URL, **When** the CLI discovers generators, **Then** the repository is cloned to a local cache directory and all valid generators are indexed
2. **Given** a previously cached generator source, **When** the CLI refreshes generators, **Then** the cache is updated to reflect remote changes without requiring a full re-clone
3. **Given** an unreachable or invalid git URL, **When** the CLI attempts discovery, **Then** an actionable error explains the failure and suggests checking the URL or network connectivity
4. **Given** a generator source repository containing multiple generators, **When** the CLI discovers generators, **Then** each valid generator in the repository is indexed individually with its own metadata

---

### User Story 4 - Version Generators Deterministically (Priority: P2)

As a platform engineer, I want generator versions to follow a deterministic scheme so that workspace generation is reproducible across machines and CI environments.

**Why this priority**: Reproducibility is critical for CI and team consistency, but initial generator usage can proceed with unversioned generators by always using the latest version. Versioning becomes essential as generators mature.

**Independent Test**: Can be fully tested by creating two versions of a generator and verifying the CLI resolves the correct version based on version constraints. Delivers value: reproducible workspace generation.

**Acceptance Scenarios**:

1. **Given** a generator with a version declared in its metadata, **When** the CLI reads the generator, **Then** the version is parsed and stored as part of the generator record
2. **Given** multiple versions of a generator available, **When** a user requests a specific version, **Then** the CLI resolves and uses the requested version
3. **Given** a generator source with multiple versions, **When** no version is specified, **Then** the CLI uses the latest stable version by default
4. **Given** a minimum CLI version declared in generator metadata, **When** the current CLI version is older, **Then** the CLI warns the user and provides guidance on upgrading

---

### User Story 5 - Register and Manage Generator Sources (Priority: P2)

As a developer, I want to add and remove generator sources so that I can access generators from different providers (official, organizational, community).

**Why this priority**: Source management enables the extensibility model, but a default official source can be included out of the box so initial usage works without configuration.

**Independent Test**: Can be fully tested by adding a custom generator source URL and verifying its generators appear in `nfw generators` output. Delivers value: extensible generator ecosystem.

**Acceptance Scenarios**:

1. **Given** a valid git repository URL, **When** the user adds a generator source, **Then** the source is persisted and its generators are available for listing and selection
2. **Given** a registered generator source, **When** the user removes the source, **Then** its generators are no longer listed and its cache is cleaned up
3. **Given** a fresh CLI installation, **When** the user lists generators, **Then** the default official generator source is available without manual configuration
4. **Given** multiple registered sources, **When** the user lists generators, **Then** generators from all sources are displayed with source attribution

---

### Edge Cases

- **Corrupted cache**: When the local generator cache is corrupted (partial clone, disk error), the CLI detects the corruption, deletes the affected cache entry, and re-clones from the remote source.
- **Invalid metadata**: When a generator metadata file contains valid syntax but semantically invalid values (e.g., version "not-a-version"), the CLI skips the generator and logs a warning identifying the generator and the invalid field.
- **Empty generator source**: When a registered source repository contains no valid generators, the CLI logs an informative warning and continues listing generators from other sources.
- **Network offline**: When the CLI cannot reach a remote generator source, it falls back to cached generators and informs the user that the listing may be stale.
- **Conflicting generator identifiers**: When two sources provide generators with the same identifier, the CLI uses a qualified identifier (source/identifier) to disambiguate and warns the user.
- **Git authentication required**: When a private generator source requires authentication, the CLI delegates to the system git credential helper and surfaces a clear error if authentication fails.
- **Generator source URL change**: When a source URL is updated to point to a different repository, the old cache is invalidated and the new repository is cloned on next discovery.
- **Disk space exhaustion**: When the local cache directory cannot be written due to disk space, the CLI reports the error with the cache location and suggests freeing space.

## Requirements

### Functional Requirements

#### Generator Metadata Schema

- **FR-001**: The generator metadata file MUST be located at the root of a generator directory with the filename `nfw.generator.yaml`
- **FR-002**: The metadata schema MUST require the following fields: `id` (unique identifier, kebab-case), `name` (human-readable display name), `description` (one-line summary), and `version` (semantic version)
- **FR-003**: The metadata schema SHOULD support the following optional fields: `tags` (list of searchable keywords), `author` (generator maintainer), `minCliVersion` (minimum CLI version required), and `sourceUrl` (canonical repository URL)
- **FR-004**: The `id` field MUST follow kebab-case naming rules (lowercase alphanumeric and hyphens, must start with a letter)
- **FR-005**: The `version` field MUST follow semantic versioning (MAJOR.MINOR.PATCH)
- **FR-006**: The optional `language` field MUST use a defined set of supported language identifiers (`dotnet`, `go`, `rust`, `neutral`), and when omitted the CLI MUST treat it as language-agnostic (`neutral`)
- **FR-007**: The CLI MUST validate all required fields on generator discovery and report actionable errors for missing or malformed fields

#### Generator Repository Format

- **FR-008**: A generator repository MUST contain a root metadata file and a content directory holding the generator files to be rendered
- **FR-009**: A generator repository MAY contain a `.nfwignore` file defining exclusion patterns (glob format) for files to skip during generation
- **FR-010**: A generator source repository (catalog) MAY contain multiple generators, each in its own subdirectory with its own metadata file
- **FR-011**: The content directory MUST support a file tree structure that mirrors the expected output workspace or service structure (see `contracts/generator-repository-structure.md` for the canonical layout)
- **FR-012**: Files and directories within the content directory MUST support placeholder naming using the `__PlaceholderName__` pattern (double underscores, PascalCase name). The initial set of supported placeholders is: `__ServiceName__`, `__Namespace__`, `__ProjectGuid__`. The CLI replaces these during generation with user-provided values. Additional placeholders may be added in future releases.
- **FR-013**: The CLI MUST respect exclusion patterns defined in the `.nfwignore` file (glob format) to skip files from generated output (e.g., `.git` directories, documentation source files)

#### Git-Based Generator Discovery

- **FR-014**: The CLI MUST support discovering generators from git repositories by cloning them to a local cache directory
- **FR-015**: The CLI MUST maintain a local cache of discovered generators, stored in `~/.nfw/generators/` (Linux/macOS) or `%USERPROFILE%\.nfw\generators\` (Windows)
- **FR-016**: The CLI MUST support incremental cache updates (git fetch) to avoid full re-clones on subsequent discovery runs
- **FR-017**: The CLI MUST include the official generator source (`https://github.com/n-framework/nfw-generators`) as a default that is available without manual configuration
- **FR-018**: The CLI MUST support registering additional generator sources via git repository URLs
- **FR-019**: The CLI MUST validate that a generator source URL points to a valid git repository before registration
- **FR-020**: The CLI MUST handle unreachable or invalid generator sources gracefully by logging a warning and continuing with available sources
- **FR-020b**: When a generator source requires git authentication, the CLI MUST delegate to the system git credential helper and surface a clear error if authentication fails
- **FR-021**: The CLI MUST fall back to cached generators when a remote source is unreachable and inform the user that the listing may be stale
- **FR-022**: Unit tests for discovery operations MUST mock git operations (no network access); integration tests MUST use real git operations clearly labeled as integration tests
- **FR-022b**: When the local generator cache is corrupted (partial clone, disk error), the CLI MUST detect the corruption, delete the affected cache entry, and re-clone from the remote source
- **FR-022c**: When the local cache directory cannot be written due to disk space, the CLI MUST report the error with the cache location and suggest freeing space

#### Generator Versioning

- **FR-023**: Each generator version MUST be derived from the `version` field in its metadata file
- **FR-024**: When multiple versions of a generator exist in a source, the CLI MUST resolve the latest stable version when no version is specified
- **FR-025**: The CLI MUST support selecting a specific generator version when requested (e.g., via a `--version` flag or qualified identifier)
- **FR-026**: Pre-release versions (e.g., `1.0.0-alpha`) MUST NOT be selected as the latest stable version by default but MUST be selectable explicitly
- **FR-027**: The CLI MUST warn when a generator declares a `minCliVersion` greater than the current CLI version

#### Generator Source Management

- **FR-028**: The CLI MUST persist registered generator sources in a `sources.yaml` file stored in the user configuration directory (see `contracts/user-config-schema.yaml` for the schema)
- **FR-029**: The CLI MUST support listing currently registered generator sources
- **FR-030**: The CLI MUST support removing a registered generator source and cleaning up its cached data
- **FR-031**: When generator identifiers collide across sources, the CLI MUST use a qualified identifier format (source-name/generator-id) for disambiguation and MUST warn the user about the collision

### Key Entities

- **Generator Metadata**: The schema definition file describing a generator's identity, version, supported language, and optional attributes. Located at the root of each generator directory.
- **Generator Source**: A git repository (remote or local) containing one or more generators. The official source is `https://github.com/n-framework/nfw-generators`. Registered in user configuration and cached locally for discovery.
- **Generator Cache**: A local directory holding cloned copies of registered generator sources. Used for offline access and fast discovery without repeated network operations.
- **Generator Content**: The file tree within a generator that defines the output structure to be generated. Supports placeholder substitution and conditional includes.
- **Generator Catalog**: A generator source repository that contains multiple generators organized in subdirectories, each with its own metadata file.
- **Qualified Generator Identifier**: A disambiguation format (source-name/generator-id) used when the same generator identifier exists in multiple sources.

## Success Criteria

### Measurable Outcomes

- **SC-001**: Generator metadata validation completes in under 50ms per generator for valid metadata files
- **SC-002**: A generator author following the quickstart.md walkthrough produces a valid `nfw.generator.yaml` that passes CLI validation on first attempt (verified by automated quickstart test)
- **SC-003**: Generator discovery from a cached source lists all generators in under 500ms for catalogs containing up to 50 generators
- **SC-004**: Generator source registration, discovery, and listing works for both fresh installations and installations with existing cached sources
- **SC-005**: Version resolution always selects the correct generator version based on the specified constraints, with zero ambiguity in test scenarios
- **SC-006**: Generated workspaces from the same generator and version produce identical file structures (reproducibility verification)

## Assumptions

- Generators are distributed as git repositories, as established in the orchestrator spec clarification (Session 2026-03-28)
- The metadata file format is YAML (consistent with the project's preference for YAML configuration, e.g., `nfw.yaml`)
- The official generator source repository is `https://github.com/n-framework/nfw-generators` and provides at least one generator for standalone .NET service workspace creation (per PRD FR-14)
- Generator versioning follows semantic versioning conventions, which are well-understood by the target audience
- Git credential handling is delegated to the system's git credential helper rather than reimplemented in the CLI
- The CLI cache directory is `~/.nfw/generators/` on Linux/macOS and `%USERPROFILE%\.nfw\generators\` on Windows (consistent with plan and data model)
- Pre-release version semantics follow the semver specification (hyphen-separated identifiers after patch version)
- Generator discovery is an explicit operation triggered by the user or CLI, not a background process

## Dependencies

- **PRD US-001**: Workspace creation depends on generator selection, which depends on generator discovery and metadata
- **PRD US-006**: `nfw generators` command depends on this specification for generator listing data
- **Orchestrator Spec (001)**: M1-T001 is the upstream task that triggers this spec; M2-T005 (`nfw generators` implementation) depends on this spec
- **Spec 002-nfw-generator-catalog-selection**: Generator catalog selection and interactive workflows depend on the metadata schema and discovery mechanism defined here

## Clarifications

- Q: What format should the generator metadata file use? → A: YAML, consistent with `nfw.yaml` and project conventions
- Q: Should the CLI support generator sources other than git (e.g., local directories, archives)? → A: Git only for initial release. Local directory support is deferred to reduce scope. Generator authors can test locally by pointing to a local git repository path.
- Q: How should the CLI handle generator source authentication for private repositories? → A: Delegate to the system git credential helper. The CLI does not implement its own authentication mechanism.
- Q: Should generators support inheritance or composition (generators based on other generators)? → A: Not in initial release. Each generator is standalone. Composition can be considered post-beta.
- Q: What is the official generator source repository URL? → A: `https://github.com/n-framework/nfw-generators`

## Non-Goals

- Generator inheritance or composition (generators extending other generators)
- Local directory generator sources (non-git)
- Generator archive distribution (zip, tar)
- Background or automatic generator discovery/refresh
- Generator build pipelines or validation CI within the CLI
- Generator marketplace or centralized registry beyond git repositories
- Generator signing or verification of generator integrity
- Generator migration workflows (updating a workspace from one generator version to another)
- Custom generator engines beyond placeholder substitution
