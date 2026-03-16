# Research: nfw CLI Skeleton

## Decisions

### 1) Language/Runtime

- Decision: C# on .NET 11
- Rationale: Matches the `.slnx`-based structure of this repository and enables a consistent cross-platform CLI toolchain.
- Alternatives considered: .NET 10; Go/Rust (not aligned with current project layout).

### 2) CLI Framework

- Decision: Spectre.Console.Cli + Microsoft.Extensions.DependencyInjection with a `TypeRegistrar`/`TypeResolver` integration pattern
- Rationale: Stable, ergonomic command routing and help output, and the DI integration keeps command construction testable.
- Alternatives considered: `System.CommandLine` (API stability concerns and ecosystem churn); hand-rolled argument parser (higher maintenance).

### 3) Dev Tooling (Format/Analysis)

- Decision: Use `dotnet tool restore` and run `dotnet csharpier .` + `dotnet roslynator analyze` in CI/dev workflows.
- Rationale: Matches the old project’s conventions and keeps formatting and analysis consistent.
- Alternatives considered: `dotnet format`; analyzer-only enforcement.

### 3.1) Code Generation Reuse

- Decision: Extract code generation utilities into a standalone `n-framework-core-codegen` repository and consume it as a submodule under `src/nfw/packages/n-framework-core-codegen/`.
- Rationale: Reuses the proven codegen building blocks from the old project while keeping the CLI skeleton focused.
- Alternatives considered: Keep codegen inside the CLI repo; publish as NuGet from day one.

### 4) Configuration File Parsing

- Decision: Load YAML from `nfw.yaml` in the current working directory; environment variables (`NFW_<SETTING_NAME>`) override file keys.
- Rationale: Project-scoped config supports repo-local behavior; YAML is human-friendly for incremental expansion.
- Alternatives considered: JSON (`nfw.json`); TOML (`nfw.toml`).

### 5) Config Failure Behavior

- Decision: Missing `nfw.yaml` is not an error; malformed YAML prints an error and continues with default configuration.
- Rationale: Keeps the skeleton CLI usable without requiring a config, while still surfacing actionable diagnostics.
- Alternatives considered: Hard-fail on parse error; warn only when `--verbose`.

### 6) Flag Conventions (Version vs Verbose)

- Decision: `--version` and `-v` show version; `--verbose` enables diagnostics (no short flag).
- Rationale: Avoids ambiguity in parsing and keeps `-v` consistent with version expectation in this spec.
- Alternatives considered: Reserve `-v` for verbose and use `-V` for version; no short flag for version.

### 7) Exit Codes

- Decision: `0` success (including `--help` and no-args help), `2` for usage errors (unknown command/flags/invalid args), `1` for runtime failures, `130` for SIGINT.
- Rationale: Matches common CLI conventions and enables reliable scripting.
- Alternatives considered: `1` for all errors (less testable, less script-friendly).

### 8) Unknown Command Suggestions

- Decision: On unknown commands, print an error + a small set of suggested commands based on prefix and edit-distance, and exit `2`.
- Rationale: Helps onboarding without increasing surface area (no new commands or modes).
- Alternatives considered: Show only top-level help; require an external completion plugin.

### 9) Template Catalog Source

- Decision: Template source is not user-configurable.
- Rationale: Keeps behavior deterministic while supporting fast local iteration in debugging environments.
- Alternatives considered: Local `Templates/` folder; user-configurable remote source.
- Notes: Debugging environments read from `src/nfw/packages/n-framework-nfw-templates` (git submodule). Production fetches `github.com/n-framework/nfw-templates` at the release tag matching the CLI version.
