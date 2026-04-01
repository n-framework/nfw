# Research: Template Metadata Schema, Discovery, and Versioning

**Feature**: Template Metadata Schema, Discovery, and Versioning
**Date**: 2026-03-29
**Status**: Complete

## Architecture Decision: Abstraction + Adapter Pattern

**Decision**: Core business logic depends only on trait abstractions. External libraries isolated behind adapter implementations.

**Rationale**:

- Aligns with NFramework PRD principles: "Pure Core" and "Infrastructure as Replaceable Adapters"
- Core module has ZERO external dependencies (no serde, git2, semver, regex, dirs in Cargo.toml)
- Enables testing core business logic with mock implementations
- Allows swapping implementations without changing core code
- Prevents external library types from leaking into core domain models

**Benefits**:

- Core remains AOT-friendly (no runtime reflection dependencies)
- Easy to test (no complex test doubles needed)
- Clear dependency boundaries (core → traits ← adapters)
- Future-proof (can add new adapters without breaking core)

## Research Decisions (Adapter Implementations)

### R-001: YAML Parsing Adapter

**Decision**: Implement `YamlParser` trait using `serde_yaml`

**Adapter**: `SerdeYamlParser` (feature: `adapter-yaml-serde`)

**Rationale**:

- serde_yaml is de facto standard for YAML in Rust
- Will be used in adapter layer only
- Core defines `YamlParser` trait with `parse<T>()` and `serialize<T>()` methods
- Core's `TemplateMetadata` uses plain Rust structs (no serde derives)

**Trait API**:

```rust
pub trait YamlParser: Send + Sync {
    fn parse<T>(&self, content: &str) -> Result<T, YamlError>
    where
        T: ParseYaml;

    fn serialize<T>(&self, value: &T) -> Result<String, YamlError>
    where
        T: SerializeYaml;
}
```

**Alternatives Considered**:

- Manual YAML parsing: Too complex, error-prone
- Other YAML crates: Less ecosystem support than serde_yaml

---

### R-002: Git Operations Adapter

**Decision**: Implement `GitRepository` trait using system git CLI (NOT git2 library)

**Adapter**: `CliGitRepository` (feature: `adapter-git-cli`)

**Rationale**:

- **Zero external dependencies** for git operations (no git2 crate)
- System git CLI is pre-installed on developer machines
- Git CLI provides **actionable error messages** (PRD Principle 3)
- User's existing git configuration (SSH keys, credentials) works automatically
- Simpler adapter implementation using `std::process::Command`
- No library version conflicts or compilation issues

**Trait API**:

```rust
pub trait GitRepository: Send + Sync {
    fn clone(&self, url: &str, dest: &Path) -> Result<(), GitError>;
    fn fetch(&self, path: &Path) -> Result<(), GitError>;
    fn current_branch(&self, path: &Path) -> Result<String, GitError>;
    fn is_valid_repo(&self, path: &Path) -> bool;
}
```

**Implementation**:

```rust
pub struct CliGitRepository {
    pub git_path: PathBuf,  // Path to git executable (default: "git")
}

impl GitRepository for CliGitRepository {
    fn clone(&self, url: &str, dest: &Path) -> Result<(), GitError> {
        let output = Command::new(&self.git_path)
            .args(["clone", url, dest.to_str().unwrap()])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            // Forward git CLI's actionable error directly
            return Err(GitError::CloneFailed(stderr.into()));
        }
        Ok(())
    }
    // ... other methods
}
```

**Error Messages (from git CLI)**:

| Git CLI Output                           | Actionable Message                            |
| ---------------------------------------- | --------------------------------------------- |
| `fatal: repository 'url' not found`      | Repository not found. Check the URL.          |
| `fatal: destination path already exists` | Directory exists. Use `--force` to overwrite. |
| `fatal: could not read Username`         | Git credentials needed. Run `git config`      |
| `fatal: unable to access`                | Access denied. Check SSH key or permissions.  |

**Notes**:

- For private repos, user configures git credentials normally (CLI delegates to system git)
- Unit tests of core use mock git; integration tests use real git CLI
- Git CLI must be available on system PATH (documented prerequisite)

---

### R-003: Version Comparison Adapter

**Decision**: Implement `VersionComparator` trait using custom `Version` type + optional semver adapter

**Adapter**: `SemverComparator` (feature: `adapter-version-semver`)

**Rationale**:

- Core defines its own `Version` struct (MAJOR.MINOR.PATCH + pre/build metadata)
- Core defines `VersionComparator` trait for version operations
- Adapter uses semver crate internally but returns core types
- Core never depends on semver::Version

**Core Version Type**:

```rust
// In core - no external dependencies
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Option<String>,
    pub build: Option<String>,
}
```

**Trait API**:

```rust
pub trait VersionComparator: Send + Sync {
    fn parse(&self, version: &str) -> Result<Version, VersionError>;
    fn compare(&self, a: &Version, b: &Version) -> Ordering;
    fn is_stable(&self, version: &Version) -> bool;
    fn satisfies(&self, version: &Version, min: &Version) -> bool;
}
```

**Alternatives Considered**:

- Direct semver dependency: Would leak external type into core
- Manual version parsing: Complex, error-prone edge cases

---

### R-004: Placeholder Syntax

**Decision**: Use double-underscore pattern: `__PlaceholderName__`

**Rationale**:

- Double underscores are rare in real filenames (unlike single underscores)
- Visually distinct from regular identifiers
- Pattern is unambiguous for regex matching
- Common convention in template systems

**Examples**:

- `__ServiceName__` → replaced with actual service name
- `__Namespace__` → replaced with project namespace
- `__ProjectGuid__` → replaced with generated GUID

**Validation**: Implemented in `Validator` trait with `is_kebab_case()` and placeholder pattern methods

**Regex Pattern** (in adapter only): `\b__([A-Z][a-zA-Z0-9]*)__\b`

---

### R-005: Cache Refresh Strategy

**Decision**: Lazy refresh with explicit force option

**Strategy**:

1. **Default**: Use cached templates without remote check
2. **Explicit refresh**: User runs `nfw templates --refresh` to fetch latest
3. **Auto-refresh on error**: If cache is corrupted, auto-fetch from remote
4. **Stale warning**: When using cached templates, display "last refreshed" timestamp

**Rationale**:

- Fastest UX (no network latency on normal operations)
- Deterministic behavior (templates don't change unexpectedly)
- Explicit control for users who need freshness
- Aligns with spec assumption: "Template discovery is an explicit operation"

**Implemented via**: `GitRepository` trait's `fetch()` method called explicitly by CLI command

---

### R-006: Configuration File Format

**Decision**: YAML format for `sources.yaml`

**Adapter**: `FileConfigStore` uses `YamlParser` adapter

**Rationale**:

- Consistent with project preference (`nfw.yaml`, `template.yaml`)
- Human-readable and editable
- Supports comments for documentation
- Familiar to target audience (devs know YAML)
- Reuses YAML parsing adapter (no new dependency)

**Location**: Platform-specific user home directory (Clean Architecture pattern)

- Config: `~/.nfw/sources.yaml` (Linux/macOS) or `%USERPROFILE%\.nfw\sources.yaml` (Windows)
- Cache: `~/.nfw/templates/` (Linux/macOS) or `%USERPROFILE%\.nfw\templates\` (Windows)

**Rationale**:

- Follows CLI convention of using `~/.cli-name/` for user data
- Clean separation: user data separate from project source code
- Cross-platform: works on Linux, macOS, Windows
- Simple: single directory for all CLI data

---

### R-007: Error Handling Strategy

**Decision**: Core defines own error types; adapters convert external errors

**Rationale**:

- Core doesn't leak external error types (e.g., no serde_yaml::Error, git2::Error)
- Clean API boundary between core and adapters
- Easier to change adapter implementations
- AOT-friendly (no generic error types from external libraries)

**Core Error Types**:

```rust
// In core/error.rs - no external dependencies
pub enum YamlError {
    ParseFailed(String),
    SerializeFailed(String),
    InvalidUtf8,
}

pub enum GitError {
    CloneFailed(String),
    FetchFailed(String),
    InvalidUrl(String),
    AuthenticationFailed(String),
}

pub enum VersionError {
    InvalidFormat(String),
    OutOfRange,
}
```

**Adapter Conversion**:

```rust
// Adapter converts external errors to core errors
impl From<serde_yaml::Error> for YamlError {
    fn from(err: serde_yaml::Error) -> Self {
        YamlError::ParseFailed(err.to_string())
    }
}
```

---

### R-008: Adapter Composition

**Decision**: Adapters receive trait objects as dependencies

**Rationale**:

- `FileConfigStore` depends on `YamlParser` (trait object)
- `TemplateRegistry` depends on `GitRepository`, `YamlParser`, `FileSystem`, etc.
- Enables testing by injecting mock implementations
- Clear dependency graph through trait bounds

**Example**:

```rust
pub struct TemplateRegistry {
    git: Box<dyn GitRepository>,
    yaml: Box<dyn YamlParser>,
    fs: Box<dyn FileSystem>,
    // ...
}

impl TemplateRegistry {
    pub fn new(
        git: Box<dyn GitRepository>,
        yaml: Box<dyn YamlParser>,
        fs: Box<dyn FileSystem>,
    ) -> Self {
        Self { git, yaml, fs }
    }
}
```

---

### R-009: Mock Implementation Strategy

**Decision**: Provide in-memory mock implementations for testing

**Rationale**:

- Core tests need fast, deterministic behavior
- No external dependencies in mocks
- Easy to inject error conditions for edge case testing

**Mock Implementations**:

- `MockYamlParser`: In-memory YAML parsing (simple key-value)
- `MockGitRepository`: No-op git operations (tracks calls)
- `MockFileSystem`: In-memory file system (HashMap-based)
- `MockVersionComparator`: Simple string comparison for versions

---

## Additional Decisions

### Template Metadata Filename

**Decision**: `template.yaml` (not `template.json`)

**Rationale**:

- Consistent with YAML preference throughout project
- More human-readable/editable for template authors
- Comments supported for documentation
- Parsed via YAML adapter

---

### Source Name Derivation

**Decision**: Derive from repository URL, allow user override

**Default Derivation**:

- Extract last path component from URL
- Remove `.git` suffix if present
- Normalize to kebab-case
- Example: `https://github.com/org/nfw-templates.git` → `nfw-templates`

**Override**: Users can specify custom `--name` when adding sources

---

### Cache Directory Structure

**Decision**: Flat structure with source-named subdirectories in `~/.nfw/templates/`

```bash
~/.nfw/templates/
├── official/           # Cloned from github.com/n-framework/nfw-templates
├── my-company/         # Cloned from custom source
└── .registry.json      # Index of cached templates
```

**Rationale**:

- Simple to navigate and debug
- Easy to invalidate (delete source directory)
- Source name matches registered source name

---

## Implementation Notes

### Error Handling Across Layers

| Layer   | Error Type                      | Conversion                          |
| ------- | ------------------------------- | ----------------------------------- |
| Adapter | External library error          | `From<ExternalError> for CoreError` |
| Core    | Core error type                 | Returned directly                   |
| CLI     | Displays user-friendly messages | `match` on core error               |

### Testing Strategy

| Test Type          | Dependencies             | Location             |
| ------------------ | ------------------------ | -------------------- |
| Core Unit Tests    | None (mocks from mocks/) | `tests/core/`        |
| Adapter Unit Tests | External library only    | `tests/adapters/`    |
| Integration Tests  | All external libraries   | `tests/integration/` |

### Feature Flag Validation

```toml
[features]
default = ["adapter-yaml-serde", "adapter-git-cli", "adapter-version-semver", "adapter-path-dirs", "adapter-validator-regex"]

# Testing feature (no external deps)
test-utils = []

# Individual adapters (can be mixed and matched)
adapter-yaml-serde = ["serde", "serde_yaml"]
adapter-git-cli = []  # No external deps - uses system git CLI
adapter-version-semver = ["semver"]
adapter-path-dirs = ["dirs"]
adapter-validator-regex = ["regex"]
```

### Module Dependency Graph

```text
core/
├── traits/          (no deps)
├── template/        (depends on traits/)
├── error.rs         (no deps)
└── lib.rs           (re-exports)

adapters/
├── yaml/serde.rs    (depends on: serde, serde_yaml; implements traits::yaml)
├── git/cli.rs        (depends on: std ONLY; implements traits::git) ← NO external deps!
├── version/semver.rs (depends on: semver; implements traits::version)
```

### Version Resolution Algorithm

1. Parse all template versions from metadata (via VersionComparator)
2. Filter out pre-releases (via VersionComparator::is_stable())
3. Sort by semver (via VersionComparator::compare())
4. Select highest version
5. Validate minCliVersion constraint (via VersionComparator::satisfies())

### Kebab-Case Validation Pattern

```regex
^[a-z][a-z0-9-]*$
```

Rules:

- Must start with lowercase letter
- May contain lowercase letters, numbers, hyphens
- No consecutive hyphens
- No trailing hyphen
