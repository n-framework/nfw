# Data Model: Template System

**Feature**: Template Metadata Schema, Discovery, and Versioning
**Date**: 2026-03-29

**Architecture Note**: Template management entities (TemplateMetadata, TemplateSource, TemplatesService) are in **nfw** package. Template rendering is handled by **core-template-rust** package.

## Repository Organization

**nfw** (this repository): Template management

- TemplateMetadata, TemplateSource, TemplatesService
- Template discovery, caching, version resolution
- `nfw templates` commands

**core-template-rust** (separate repository): Template rendering

- PlaceholderRenderer, FileGenerator, TemplateContext
- Pure rendering engine - no git, no caching, no discovery

## Entity Definitions

### TemplateMetadata

Represents the parsed content of a `template.yaml` file.

**Location**: `nfw/src/features/template-management/models/`

```rust
// NO external dependencies - plain Rust struct
pub struct TemplateMetadata {
    /// Unique identifier (kebab-case)
    pub id: String,

    /// Human-readable display name
    pub name: String,

    /// One-line summary
    pub description: String,

    /// Semantic version (custom type, NOT semver::Version)
    pub version: Version,

    /// Target language
    pub language: Language,

    /// Searchable keywords (optional)
    pub tags: Option<Vec<String>>,

    /// Template maintainer (optional)
    pub author: Option<String>,

    /// Minimum CLI version required (optional)
    pub min_cli_version: Option<Version>,

    /// Canonical repository URL (optional)
    pub source_url: Option<String>,
}
```

**Note**: No `#[serde(...)]` attributes. Serialization handled by YamlParser adapter via `ParseYaml`/`SerializeYaml` traits.

#### Validation Rules

| Field         | Rule                                       | Error Message                            |
| ------------- | ------------------------------------------ | ---------------------------------------- |
| `id`          | Matches `^[a-z][a-z0-9-]*$`                | "Template ID must be kebab-case: {id}"   |
| `name`        | Not empty, max 100 chars                   | "Template name required (max 100 chars)" |
| `description` | Not empty, max 200 chars                   | "Description required (max 200 chars)"   |
| `version`     | Valid semver (parsed by VersionComparator) | "Invalid semantic version: {version}"    |
| `language`    | One of: `dotnet`, `go`, `rust`             | "Unsupported language: {language}"       |

---

### Version

Custom semantic version type in domain (NOT the semver crate's type).

**Location**: `src/nfw/src/nframework-nfw/core/nframework-nfw-domain/src/features/versioning/`

```rust
// NO external dependencies - plain Rust struct
pub struct Version {
    /// Major version
    pub major: u64,

    /// Minor version
    pub minor: u64,

    /// Patch version
    pub patch: u64,

    /// Pre-release identifier (e.g., "alpha", "beta.1")
    pub pre: Option<String>,

    /// Build metadata
    pub build: Option<String>,
}

impl Version {
    /// Create a new stable version
    pub fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: None,
            build: None,
        }
    }

    /// Create a pre-release version
    pub fn pre_release(major: u64, minor: u64, patch: u64, pre: &str) -> Self {
        Self {
            major,
            minor,
            patch,
            pre: Some(pre.to_string()),
            build: None,
        }
    }

    /// Check if this is a stable release (no pre-release identifier)
    pub fn is_stable(&self) -> bool {
        self.pre.is_none()
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)?;
        if let Some(ref pre) = self.pre {
            write!(f, "-{}", pre)?;
        }
        if let Some(ref build) = self.build {
            write!(f, "+{}", build)?;
        }
        Ok(())
    }
}

impl FromStr for Version {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Simplified parsing - actual implementation uses VersionComparator adapter
        // This is a basic fallback; real parsing delegated to adapter
        Err("Version parsing delegated to VersionComparator adapter".to_string())
    }
}
```

**Note**: Version comparison, parsing, and validation delegated to `VersionComparator` trait. The struct itself is a plain data container.

---

### Language Enum

Supported target languages for templates.

**Location**: `nfw/src/features/template-management/models/`

```rust
// NO external dependencies - plain Rust enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Dotnet,
    Go,
    Rust,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Dotnet => "dotnet",
            Language::Go => "go",
            Language::Rust => "rust",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "dotnet" => Some(Language::Dotnet),
            "go" => Some(Language::Go),
            "rust" => Some(Language::Rust),
            _ => None,
        }
    }
}

impl Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```

---

### TemplateSource

A registered template source (git repository).

**Location**: `nfw/src/features/template-management/models/`

```rust
// NO external dependencies - plain Rust struct
pub struct TemplateSource {
    /// Source identifier (user-defined or derived from URL)
    pub name: String,

    /// Git repository URL
    pub url: String,

    /// Whether source is active for discovery
    pub enabled: bool,
}

impl TemplateSource {
    pub fn new(name: String, url: String) -> Self {
        Self {
            name,
            url,
            enabled: true,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }
}
```

#### TemplateSource Validation Rules

| Field  | Rule                                               | Error Message                               |
| ------ | -------------------------------------------------- | ------------------------------------------- |
| `name` | Unique across sources, kebab-case                  | "Source name must be unique and kebab-case" |
| `url`  | Valid git URL (validated by GitRepository adapter) | "Invalid git URL: {url}"                    |

---

### CachedTemplate

A template discovered and cached from a source.

**Location**: `nfw/src/features/template-management/models/`

```rust
// NO external dependencies - plain Rust struct
use std::path::PathBuf;

pub struct CachedTemplate {
    /// Source this template belongs to
    pub source_name: String,

    /// Parsed metadata
    pub metadata: TemplateMetadata,

    /// Local cache directory path
    pub cache_path: PathBuf,

    /// When cache was last refreshed (Unix epoch seconds)
    pub last_refreshed: u64,
}
```

---

### TemplatesService

In-memory index of all discovered templates.

**Location**: `src/nframework-nfw/core/nframework-nfw-application/src/features/template_management/services/`

```rust
// NO external dependencies - depends on trait objects only
use std::collections::HashMap;
use std::path::PathBuf;

pub struct TemplatesService {
    /// All discovered templates indexed by qualified ID
    templates: HashMap<QualifiedTemplateId, CachedTemplate>,

    /// Registered sources
    sources: Vec<TemplateSource>,

    /// Trait objects for external operations
    git: Box<dyn traits::GitRepository>,
    yaml: Box<dyn traits::YamlParser>,
    fs: Box<dyn traits::FileSystem>,
    path_resolver: Box<dyn traits::PathResolver>,
    version_comparator: Box<dyn traits::VersionComparator>,
    validator: Box<dyn traits::Validator>,
}

/// Fully-qualified template identifier (source/template)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct QualifiedTemplateId {
    pub source: String,
    pub template: String,
}

impl QualifiedTemplateId {
    pub fn new(source: String, template: String) -> Self {
        Self { source, template }
    }

    pub fn unqualified(template: String) -> Self {
        Self {
            source: String::new(),
            template,
        }
    }

    pub fn is_qualified(&self) -> bool {
        !self.source.is_empty()
    }
}

impl TemplatesService {
    pub fn new(
        git: Box<dyn traits::GitRepository>,
        yaml: Box<dyn traits::YamlParser>,
        fs: Box<dyn traits::FileSystem>,
        path_resolver: Box<dyn traits::PathResolver>,
        version_comparator: Box<dyn traits::VersionComparator>,
        validator: Box<dyn traits::Validator>,
    ) -> Self {
        Self {
            templates: HashMap::new(),
            sources: Vec::new(),
            git,
            yaml,
            fs,
            path_resolver,
            version_comparator,
            validator,
        }
    }

    /// Discover all templates from registered sources
    pub fn discover(&mut self) -> Result<Vec<CachedTemplate>, RegistryError> {
        // Implementation uses trait objects only
        // No direct external library calls
    }

    /// Resolve a template by qualified or unqualified ID
    pub fn resolve(&self, id: &QualifiedTemplateId) -> Option<&CachedTemplate> {
        // Implementation
    }

    /// Add a new template source
    pub fn add_source(&mut self, source: TemplateSource) -> Result<(), RegistryError> {
        // Validates URL via GitRepository trait
        // Stores source
    }

    /// Remove a template source
    pub fn remove_source(&mut self, name: &str) -> Result<bool, RegistryError> {
        // Removes source and cleans up cache
    }
}
```

**Lookup Behavior**:

- Unqualified ID (`microservice`): Searches all sources, returns first match
- Qualified ID (`official/microservice`): Direct lookup in specific source
- Conflict warning: When multiple sources have same template ID

---

## Relationships

```text
TemplateSource (1) ──────┬── (0..*) CachedTemplate
                          │
                          ├── TemplateMetadata (core, no deps)
                          ├── cache_path: PathBuf
                          └── last_refreshed: DateTime

TemplatesService (1) ─────┬── (0..*) TemplateSource
                          │
                          └── (0..*) CachedTemplate (indexed by QualifiedTemplateId)
                              │
                              └── Uses trait objects:
                                  ├── GitRepository
                                  ├── YamlParser
                                  ├── FileSystem
                                  ├── PathResolver
                                  ├── VersionComparator
                                  └── Validator
```

---

## State Transitions

### Template Source Lifecycle

```text
[Unregistered] ──add──> [Registered & Enabled]
                        │
                        │ disable
                        ▼
                   [Registered & Disabled]
                        │
                        │ remove
                        ▼
                    [Unregistered]
```

### Cache Lifecycle

```text
[Empty] ──clone──> [Cached] ──refresh──> [Cached (updated)]
   │                              │
   │                              │
   │ corrupt/error                │
   ▼                              ▼
[Corrupted] ──reclone──> [Cached (fresh)]
```

---

## Configuration Schema

### sources.yaml

```yaml
# Registered template sources
sources:
  - name: official
    url: https://github.com/n-framework/nfw-templates
    enabled: true

  - name: my-company
    url: https://github.com/mycompany/templates
    enabled: true
```

**Parsed by**: `FileConfigStore` adapter (uses `YamlParser` adapter)

---

## Template Repository Structure

### Single Template Repository

```bash
my-template/
├── template.yaml          # Required: Template metadata
├── content/               # Required: Template files
│   ├── src/
│   ├── tests/
│   └── ...
└── .nfwignore             # Optional: Excludes from generation
```

### Catalog Repository (Multiple Templates)

```bash
nfw-templates/
├── microservice/
│   ├── template.yaml
│   └── content/
├── grpc-service/
│   ├── template.yaml
│   └── content/
└── worker/
    ├── template.yaml
    └── content/
```

---

## Trait Dependencies

### Core Depends On Traits (Not Concretions)

```rust
// In core - only trait bounds
use crate::traits::{
    GitRepository, YamlParser, FileSystem,
    PathResolver, VersionComparator, Validator,
};

pub struct TemplatesService {
    git: Box<dyn GitRepository>,       // Trait object
    yaml: Box<dyn YamlParser>,         // Trait object
    fs: Box<dyn FileSystem>,            // Trait object
    // etc.
}
```

### Adapters Implement Traits

```rust
// In adapters - concrete implementations use external libs
use serde_yaml;  // External lib isolated here

pub struct SerdeYamlParser;

impl YamlParser for SerdeYamlParser {
    fn parse<T>(&self, content: &str) -> Result<T, YamlError> {
        // serde_yaml::from_str(...)
        // Convert serde_yaml::Error to YamlError
    }
}
```

---

## Dependency Graph

```text
[nfw internal versioning module] (NO external dependencies)
    │
    └── src/features/
        └── versioning/ (Version value objects, VersionComparator trait)

[core-cli-rust] (NO external dependencies - CLI abstractions only)
    │
    └── Command trait, CliAdapter trait, ClapAdapter

[nfw internal git module] (NO external dependencies - uses system git CLI)
    │
    └── GitRepository trait, CliGitRepository implementation

[core-template-rust] (NO external dependencies - rendering only)
    │
    └── PlaceholderRenderer, FileGenerator, TemplateContext

[nfw] (depends on core-* packages, clap)
    │
    └── src/
        ├── features/
        │   ├── template-management/models/ (TemplateMetadata, TemplateSource, Language)
        │   ├── template-management/value-objects/ (QualifiedTemplateId)
        │   └── template-management/services/ (TemplatesService)
        ├── config/ (ConfigStore, FileConfigStore)
        ├── path/ (PathResolver, PlatformPathResolver)
        ├── validation/ (Validator, RegexValidator)
        └── commands/templates/ (Template list/add/remove commands)
```

## Package Dependencies

```text
nfw
  ↓ + ↓ + ↓ + ↓
core-cli-rust + core-template-rust + nfw internal git/versioning modules
```
