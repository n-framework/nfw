# Implementation Plan: Generator Metadata Schema, Discovery, and Versioning

**Branch**: `001-nfw-generator-system` | **Date**: 2026-03-29 | **Spec**: [spec.md](./spec.md)
**Spec Type**: Project-Based
**Project**: nfw
**Input**: Feature specification from [`src/nfw/specs/001-nfw-generator-system/spec.md`](./spec.md)

## Summary

Define the generator metadata schema (YAML-based), generator repository structure, git-based generator discovery with local caching, and semver-based versioning for the nfw CLI. The official generator source is `https://github.com/n-framework/nfw-generators`.

**Architecture Principle**: Abstraction + Adapter pattern. Core business logic depends only on trait abstractions. External libraries (serde_yaml, semver, etc.) are isolated behind adapter implementations. Git operations use the system's git CLI directly (no git2 library), providing actionable error messages aligned with PRD Principle 3. This ensures core remains independent of implementation choices and enables testing without external dependencies.

## Technical Context

**Language/Version**: Rust 1.85+ (2024 edition)
**Naming Convention**: project/crate/repo/folder names use kebab-case; Rust source modules/files use snake_case; Rust types use PascalCase; functions/variables use snake_case.

**Architecture**: Multi-Crate Clean Architecture

- **Domain Layer** (`nframework-nfw-domain`): Pure Rust entities and value objects. ZERO external dependencies. No serde, no external crates.
- **Application Layer** (`nframework-nfw-application`): Use cases, services, trait abstractions. Depends on domain + core-\* packages. No direct external library calls.
- **Infrastructure Layer** (`nframework-nfw-infrastructure-*`): External adapter implementations. Each adapter is a separate crate (filesystem, git, yaml, versioning). Isolated external dependencies.
- **Presentation Layer** (`n-framework-nfw-cli`): CLI entry point, argument parsing (clap), command routing. Depends on application layer.
- **External Core Packages**: Shared libraries still separate from nfw (`core-cli-rust`, `core-generator-rust`).
- **NFW Internal Modules**: Git and versioning are implemented inside nfw clean-architecture crates (application abstractions + infrastructure adapters).

**Testing**: Domain/Application tested with mock implementations. Infrastructure adapters tested in isolation. Integration tests use real adapters with real git operations.

**Primary Abstractions** (in Core, no dependencies):

| Trait               | Purpose                             | Methods                                                                                                                                   |
| ------------------- | ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| `YamlParser`        | Parse YAML to/from structures       | `parse<T>(&str) -> Result<T>`, `serialize<T>(&T) -> Result<String>`                                                                       |
| `GitRepository`     | Git operations (clone, fetch, pull) | `clone(&Url, &Path) -> Result<()>`, `fetch(&Path) -> Result<()>`, `get_current_branch(&Path) -> Result<String>`                           |
| `VersionComparator` | Semantic version operations         | `parse(&str) -> Result<Version>`, `compare(&Version, &Version) -> Ordering`, `is_stable(&Version) -> bool`                                |
| `FileSystem`        | File system operations              | `read_file(&Path) -> Result<String>`, `write_file(&Path, &str) -> Result<()>`, `create_dir(&Path) -> Result<()>`, `exists(&Path) -> bool` |
| `ConfigStore`       | Configuration persistence           | `load_sources() -> Result<Vec<Source>>`, `save_sources(&Vec<Source>) -> Result<()>`                                                       |
| `PathResolver`      | Platform-specific path resolution   | `cache_dir() -> PathBuf`, `config_dir() -> PathBuf`                                                                                       |
| `Validator`         | String validation (kebab-case, URL) | `is_kebab_case(&str) -> bool`, `is_git_url(&str) -> bool`                                                                                 |

**Adapter Implementations** (isolated, each behind feature flag):

| Adapter            | External Library                 | Feature Flag                  |
| ------------------ | -------------------------------- | ----------------------------- |
| `SerdeYamlParser`  | `serde`, `serde_yaml`            | `adapter-yaml-serde`          |
| `CliGitRepository` | **NONE** (uses system `git` CLI) | `adapter-git-cli` (always on) |
| `SemverComparator` | `semver` crate                   | `adapter-version-semver`      |
| `StdFileSystem`    | `std::fs`, `std::io`             | `adapter-fs-std` (always on)  |
| `FileConfigStore`  | `std::fs`, `serde_yaml`          | `adapter-config-file`         |
| `DirsPathResolver` | `dirs` crate                     | `adapter-path-dirs`           |
| `RegexValidator`   | `regex` crate                    | `adapter-validator-regex`     |

**Storage**:

- Local file system cache: Platform-specific user data directory
  - Linux/macOS: `~/.nfw/generators/`
  - Windows: `%USERPROFILE%\.nfw\generators\`
- User configuration file: Platform-specific user config directory
  - Linux/macOS: `~/.nfw/sources.yaml`
  - Windows: `%USERPROFILE%\.nfw\sources.yaml`

**Testing**:

- Core unit tests: Use in-memory mock implementations (no external deps)
- Adapter unit tests: Test each adapter in isolation
- Integration tests: Use real adapters with real git operations

**Target Platform**: Linux, macOS, Windows (cross-platform CLI)

**Project Type**: CLI tool / generator system library

**Performance Goals**:

- Metadata validation: < 50ms per generator (SC-001)
- Generator listing from cache: < 500ms for 50 generators (SC-003)

**Constraints**:

- Git-only generator sources (no local directories, no archives in initial release)
- Use system `git` CLI for git operations (no git2 library) - provides actionable errors per PRD Principle 3
- Delegate authentication to system git credential helper (configured in user's git)
- Single-step build and test (Constitution I)
- Unit tests must not use real network access (Constitution IV)
- **Core module MUST NOT depend on external libraries** (PRD Pure Core principle)
- **External libraries isolated behind adapter traits** (PRD Infrastructure as Replaceable Adapters)

**Scale/Scope**:

- Support for unlimited generator sources (no hard limit)
- Support for unlimited generators per source (no hard limit)
- Versioned generators following semver

## Constitution Check

| Principle                                | Status | Notes                                                          |
| ---------------------------------------- | ------ | -------------------------------------------------------------- |
| **I. Single-Step Build And Test**        | Pass   | Will ensure `cargo build` and `cargo test` work from repo root |
| **II. CLI I/O And Exit Codes**           | Pass   | CLI output to stdout, errors to stderr; documented exit codes  |
| **III. No Suppression**                  | Pass   | No warning suppression, no test skipping                       |
| **IV. Deterministic Tests**              | Pass   | Core unit tests use mocks; adapter tests explicitly labeled    |
| **V. Documentation Is Part Of Delivery** | Pass   | Quickstart will be generated for generator authors             |

## Project Structure

### Repository Organization

```bash
github.com/n-framework/
├── core-cli-rust           # CLI interface abstraction (separate repo)
│   ├── Command trait       # Abstraction for CLI commands
│   ├── CliAdapter trait     # Abstraction for CLI frameworks (clap, etc.)
│   └── clap adapter        # Concrete implementation using clap
│
├── core-generator-rust   # Generator rendering engine (separate repo)
│   ├── PlaceholderRenderer # Substitutes `__PlaceholderName__` → actual values
│   ├── FileGenerator        # Reads generator files, generates output directory
│   └── GeneratorContext      # Variables and values for rendering
│
└── nfw/                    # NFW CLI (this repo)
    ├── Generator management (GeneratorMetadata, sources, discovery, caching)
    ├── Config management (ConfigStore, sources.yaml)
    ├── Internal git module (application abstraction + infrastructure git adapter)
    ├── Internal versioning module (domain/application + semver adapter)
    ├── Path resolution (cache/config directories)
    ├── Validation (kebab-case, URL validation)
    └── CLI commands (uses core-cli-rust + core-generator-rust)
```

**Dependency Flow:**

```nfw (CLI commands)
  ↓ depends on
core-generator-rust + core-cli-rust + nfw internal git/versioning modules
```

### Documentation (this feature)

```text
src/nfw/specs/001-nfw-generator-system/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
└── contracts/           # Phase 1 output
    ├── generator-metadata-schema.yaml
    └── generator-repository-structure.md
```

### Source Code (nfw project - Clean Architecture)

**Note**: NFW depends on external core packages (`core-cli-rust`, `core-generator-rust`) and keeps git/versioning modules internal to nfw clean-architecture crates.

```text
src/nfw/
├── Cargo.toml               # Workspace root
├── src/nframework-nfw/
│   ├── core/
│   │   ├── nframework-nfw-domain/              # Entities, value objects (ZERO external deps)
│   │   │   └── src/features/
│   │   │       ├── generator_management/
│   │   │       │   ├── generator_metadata.rs
│   │   │       │   ├── generator_source.rs
│   │   │       │   └── generator_catalog.rs
│   │   │       └── versioning/
│   │   │           └── version_info.rs
│   │   │
│   │   └── nframework-nfw-application/         # Use cases, services, trait abstractions
│   │       └── src/features/
│   │           ├── cli/configuration/          # ConfigStore, PathResolver traits
│   │           ├── generator_management/
│   │           │   ├── queries/list_generators/ # ListGeneratorsQuery, handler
│   │           │   ├── services/               # GeneratorsService, GeneratorSelectionService
│   │           │   └── models/                 # DTOs (ListedGenerator)
│   │           └── versioning/                 # VersionResolver
│   │
│   ├── infrastructure/
│   │   ├── nframework-nfw-infrastructure-filesystem/  # File system adapters
│   │   ├── nframework-nfw-infrastructure-git/         # Git CLI adapter
│   │   ├── nframework-nfw-infrastructure-yaml/        # SerdeYamlParser adapter
│   │   └── nframework-nfw-infrastructure-versioning/  # Semver adapter
│   │
│   └── presentation/
│       └── n-framework-nfw-cli/                 # CLI entry point, commands, args
│
├── tests/
│   ├── unit/nframework-nfw/                    # Unit tests (mirrors source structure)
│   │   ├── core/nframework-nfw-domain/
│   │   ├── core/nframework-nfw-application/
│   │   └── infrastructure/nframework-nfw-infrastructure-*/
│   │
│   └── integration/nframework-nfw/             # Integration tests (real git)
│       └── features/
│           ├── generator_discovery/
│           └── version_resolution/
│
└── examples/                   # Usage examples
    └── basic_usage.rs
```

### Feature Flags (Cargo.toml)

```toml
[workspace]
members = [
    "src/nframework-nfw",
    "tests/unit/nframework-nfw",
    "tests/integration/nframework-nfw",
]

[dependencies]
# Core packages from separate repos
core-cli-rust = { version = "0.1", path = "../core-cli-rust" }
core-generator-rust = { version = "0.1", path = "../core-generator-rust" }

[features]
default = []

# Optional features for external core packages
# Git/versioning feature toggles are internal to nfw crates
full-git = []                                  # Reserved for nfw internal git module
yaml = ["core-generator-rust/yaml"]          # YAML support
semver = []                                    # Reserved for nfw internal versioning module

# All features
all = ["full-git", "yaml", "semver"]

# Testing
test-utils = []
```

## Phase 0: Research

### Unknowns to Resolve

1. **Trait API Design**: Design ergonomic trait APIs for all abstractions that balance flexibility with simplicity
2. **Error Handling Strategy**: Define core error types that work across all adapters without leaking external error types
3. **Mock Implementation Complexity**: Determine how to create useful mock implementations for testing without excessive boilerplate
4. **Adapter Composition**: Handle cases where adapters depend on other adapters (e.g., ConfigStore depends on YamlParser)
5. **Feature Flag Interaction**: Ensure optional adapters can be swapped without breaking core functionality

### Research Tasks

| Task  | Question                                                                  | Output                               |
| ----- | ------------------------------------------------------------------------- | ------------------------------------ |
| R-001 | What trait API patterns work best for abstraction + adapter in Rust?      | Trait design guidelines              |
| R-002 | How to handle errors from external libraries without leaking their types? | Error type strategy                  |
| R-003 | How to compose adapters that depend on other adapters?                    | Adapter dependency injection pattern |
| R-004 | What's the minimal viable mock for each trait?                            | Mock implementation specifications   |
| R-005 | How to ensure feature flags don't create unbuildable combinations?        | Feature flag validation strategy     |

### Deliverable

**research.md** documenting decisions with rationale for each research task.

## Phase 1: Design

### 1.1 Data Model (Core - No External Dependencies)

### Entity: GeneratorMetadata

Plain Rust struct with no serde derives. Serialization handled by YamlParser adapter.

```rust
// In core/generator/metadata.rs - NO external dependencies
pub struct GeneratorMetadata {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: Version,  // Custom struct, not semver::Version
    pub language: Language,
    pub tags: Option<Vec<String>>,
    pub author: Option<String>,
    pub min_cli_version: Option<Version>,
    pub source_url: Option<String>,
}

// Custom Version type (core defines the abstraction)
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
    pub build: Option<String>,
}

// Language enum (core)
pub enum Language {
    Dotnet,
    Go,
    Rust,
}
```

**Entity: GeneratorSource** (Core - No External Dependencies)

```rust
pub struct GeneratorSource {
    pub name: String,
    pub url: String,
    pub enabled: bool,
}
```

**Entity: CachedGenerator** (Core - No External Dependencies)

```rust
pub struct CachedGenerator {
    pub source_name: String,
    pub metadata: GeneratorMetadata,
    pub cache_path: std::path::PathBuf,
    pub last_refreshed: u64,  // Unix epoch seconds — no external datetime dependency in core
}
```

**Note**: Uses `u64` Unix timestamp to avoid external dependencies in core. Infrastructure adapters can convert to/from human-readable dates using a `TimeProvider` trait if needed in the future.

### 1.2 Trait Definitions (Core - No External Dependencies)

```rust
// In core/traits/yaml.rs
pub trait YamlParser: Send + Sync {
    fn parse<T>(&self, content: &str) -> Result<T, YamlError>
    where
        T: ParseYaml;

    fn serialize<T>(&self, value: &T) -> Result<String, YamlError>
    where
        T: SerializeYaml;
}

// In core/traits/git.rs
pub trait GitRepository: Send + Sync {
    fn clone(&self, url: &Url, dest: &Path) -> Result<(), GitError>;
    fn fetch(&self, path: &Path) -> Result<(), GitError>;
    fn current_branch(&self, path: &Path) -> Result<String, GitError>;
    fn is_valid_repo(&self, path: &Path) -> bool;
}

// In core/traits/version.rs
pub trait VersionComparator: Send + Sync {
    fn parse(&self, version: &str) -> Result<Version, VersionError>;
    fn compare(&self, a: &Version, b: &Version) -> Ordering;
    fn is_stable(&self, version: &Version) -> bool;
    fn satisfies(&self, version: &Version, min: &Version) -> bool;
}
```

### 1.3 Adapter Implementations

Each adapter implements a trait using external libraries:

```rust
// In adapters/yaml/serde.rs
pub struct SerdeYamlParser;

impl YamlParser for SerdeYamlParser {
    fn parse<T>(&self, content: &str) -> Result<T, YamlError>
    where
        T: ParseYaml,
    {
        // Uses serde_yaml::from_str
        // Converts serde_yaml::Error to YamlError
    }
}

// In adapters/git/cli.rs
pub struct CliGitRepository {
    pub git_path: PathBuf,  // Path to git executable (default: "git")
}

impl GitRepository for CliGitRepository {
    fn clone(&self, url: &str, dest: &Path) -> Result<(), GitError> {
        let output = Command::new(&self.git_path)
            .args(["clone", url, dest.to_str().unwrap()])
            .output()
            .map_err( | e | GitError::CloneFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Git CLI provides actionable errors (PRD Principle 3)
            return Err(GitError::CloneFailed(stderr.to_string()));
        }
        Ok(())
    }

    fn fetch(&self, path: &Path) -> Result<(), GitError> {
        let output = Command::new(&self.git_path)
            .args(["fetch"])
            .current_dir(path)
            .output()
            .map_err( | e | GitError::FetchFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::FetchFailed(stderr.to_string()));
        }
        Ok(())
    }

    fn current_branch(&self, path: &Path) -> Result<String, GitError> {
        let output = Command::new(&self.git_path)
            .args(["rev-parse", "--abbrev-ref", "HEAD"])
            .current_dir(path)
            .output()
            .map_err( | e | GitError::CommandFailed(e.to_string()))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(GitError::CommandFailed(stderr.to_string()));
        }

        let branch = String::from_utf8(output.stdout)
            .map_err( | e | GitError::CommandFailed(e.to_string()))?;
        Ok(branch.trim().to_string())
    }

    fn is_valid_repo(&self, path: &Path) -> bool {
        Command::new(&self.git_path)
            .args(["rev-parse", "--git-dir"])
            .current_dir(path)
            .output()
            .map( | o | o.status.success())
            .unwrap_or(false)
    }
}
```

**Note**: Uses `std::process::Command` to invoke system git. No external git2 dependency. Git CLI error messages are forwarded directly to users, providing actionable feedback (PRD Principle 3).

### 1.4 Contracts

Same as before (unchanged by architecture):

- `contracts/generator-metadata-schema.yaml`
- `contracts/generator-repository-structure.md`
- `contracts/user-config-schema.yaml`

### 1.5 Quickstart

Updated to include architecture guidance for generator authors (no changes to user-facing behavior).

## Phase 2: Implementation Outline

### Module Breakdown by Repository

#### External Core Packages (separate repositories)

These packages provide general-purpose functionality that can be used by any Rust project:

1. **core-cli-rust** (separate repo)
   - `Command` trait: Abstraction for CLI commands
   - `CliAdapter` trait: Abstraction for CLI frameworks
   - ClapAdapter: Concrete implementation using clap
   - NO business logic (config, paths, validation are application-specific)

2. **nfw internal git module** (inside nfw workspace)
   - GitRepository trait with clone/fetch/branch methods
   - CliGitRepository implementation (uses system git CLI)
   - Zero external dependencies (no git2)

3. **nfw internal versioning module** (inside nfw workspace)
   - Version struct (MAJOR.MINOR.PATCH + pre/build)
   - VersionComparator trait
   - SemverVersionComparator adapter (optional, uses semver crate)

4. **core-generator-rust** (separate repo)
   - `PlaceholderRenderer`: Substitutes `__PlaceholderName__` → actual values
   - `FileGenerator`: Reads generator files, generates output directory
   - `GeneratorContext`: Variables and values for rendering
   - Pure rendering engine - no git, no caching, no discovery
   - Used by nfw when generating new projects

#### NFW CLI (this repository)

NFW provides the CLI commands that use the core packages:

1. **nfw-cli**: CLI commands
   - `src/commands/generators/list.rs`: List available generators (uses GeneratorsService)
   - `src/commands/generators/add.rs`: Register generator sources
   - `src/commands/generators/remove.rs`: Remove generator sources
   - `src/commands/generators/refresh.rs`: Refresh generator cache
   - `src/models/`: CLI-specific DTOs
   - Uses clap for argument parsing

### Implementation Tasks by Repository

#### core-cli-rust Tasks

- [ ] Define Command trait (abstraction for CLI commands)
- [ ] Define CliAdapter trait (abstraction for CLI frameworks)
- [ ] Implement ClapAdapter using clap crate
- [ ] Add unit tests for CLI abstractions

#### nfw internal git module Tasks

- [ ] Define GitRepository trait
- [ ] Implement CliGitRepository using std::process::Command
- [ ] Add error handling with actionable messages (PRD Principle 3)
- [ ] Add unit tests with mock git binary

#### nfw internal versioning module Tasks

- [ ] Define Version struct (no external deps)
- [ ] Define VersionComparator trait
- [ ] Implement SemverVersionComparator adapter (uses semver crate)
- [ ] Add Display and FromStr implementations for Version

#### core-generator-rust Tasks

- [ ] Define PlaceholderRenderer trait
- [ ] Implement placeholder substitution (`__Name__` → value)
- [ ] Define FileGenerator for output creation
- [ ] Define GeneratorContext for variables
- [ ] Add conditional rendering support (if/else)
- [ ] Add loop support in generators

#### nfw (this repo) Tasks

- [ ] Define GeneratorMetadata entity
- [ ] Define GeneratorSource entity
- [ ] Define GeneratorsService with trait dependencies
- [ ] Implement generator discovery logic (uses nfw internal git module)
- [ ] Implement version resolution logic (uses nfw internal versioning module)
- [ ] Implement generator caching (uses core-cli-rust for directory ops)
- [ ] Define ConfigStore trait with load_sources/save_sources methods
- [ ] Implement FileConfigStore (YAML-based)
- [ ] Define PathResolver trait with cache_dir/config_dir methods
- [ ] Implement PlatformPathResolver (uses dirs crate)
- [ ] Define Validator trait with is_kebab_case/is_git_url methods
- [ ] Implement RegexValidator (uses regex crate)
- [ ] Create CLI project structure with clap (uses core-cli-rust)
- [ ] Implement generators list command
- [ ] Implement generators add command
- [ ] Implement generators remove command
- [ ] Implement generators refresh command
- [ ] Implement `nfw new` command (uses core-generator-rust)
- [ ] Add integration tests

### Testing Strategy

| Repository                     | Test Type                               | Location           |
| ------------------------------ | --------------------------------------- | ------------------ |
| core-cli-rust                  | Unit tests (CLI abstractions)           | tests/unit/        |
| nfw internal git module        | Unit tests (mock git CLI)               | tests/unit/        |
| nfw internal versioning module | Unit tests (Version, VersionComparator) | tests/unit/        |
| core-generator-rust            | Unit tests (PlaceholderRenderer)        | tests/unit/        |
| nfw domain                     | Unit tests (entities, value objects)    | tests/unit/        |
| nfw application                | Unit tests (services with mocks)        | tests/unit/        |
| nfw infrastructure             | Unit tests (adapters in isolation)      | tests/unit/        |
| nfw integration                | Integration tests (real git, e2e)       | tests/integration/ |

## Success Criteria Validation

| Criterion                                    | Validation Method                                                                            |
| -------------------------------------------- | -------------------------------------------------------------------------------------------- |
| SC-001: < 50ms metadata validation           | Benchmark test with 100 valid metadata files (using real adapter)                            |
| SC-002: Generator authoring without guidance | Quickstart walkthrough test                                                                  |
| SC-003: < 500ms listing for 50 generators    | Benchmark test with cached 50-generator catalog (using real adapter)                         |
| SC-004: Fresh + cached installations work    | Integration test for both scenarios (using real adapters)                                    |
| SC-005: Version resolution correctness       | Unit tests for all version constraint scenarios (using mock adapter)                         |
| SC-006: Reproducible output                  | Integration test comparing two generations from same generator/version (using real adapters) |

**Additional Validation**: Core module compiles with zero external dependencies.

## Dependencies

- **Upstream Spec**: Orchestrator Spec 001 (M1-T001)
- **Downstream Specs**: 002-nfw-generator-catalog-selection (depends on this)
- **External Repository**: `https://github.com/n-framework/nfw-generators` (official generator source)

## Architecture Principles

1. **Pure Core**: Domain types have ZERO external dependencies (no serde, no external crates)
2. **Abstraction First**: All external operations go through trait definitions in core-\* packages
3. **Adapter Isolation**: Each external library wrapped in adapter implementing trait (separate core-\* repo)
4. **Testability**: Core tested with mocks; adapters tested in isolation
5. **Feature Flags**: All adapters optional via Cargo features in core-\* packages
6. **Multi-Repo**: General-purpose packages in separate repos (`core-*-rust`), NFW-specific logic in this repo
7. **Error Boundaries**: Core errors don't leak external error types

## Open Questions

None - all clarifications resolved in spec Session 2026-03-29.
