# Data Model: nfw CLI Skeleton

This feature is primarily a CLI surface, so the “data model” is mostly the command schema and configuration shape used at runtime.

## Entities

### CLIInvocation

- Fields
  - `args: string[]` (raw argv excluding executable)
  - `workingDirectory: string` (used to locate `nfw.yaml`)
  - `environment: map<string,string>` (used to apply `NFW_*` overrides)

### CommandDefinition

- Fields
  - `name: string` (e.g., `templates`)
  - `description: string`
  - `handlerId: string` (logical identifier for handler binding)
  - `options: OptionDefinition[]`

### OptionDefinition

- Fields
  - `longName: string` (e.g., `--help`)
  - `shortName: string?` (e.g., `-h`, optional)
  - `isFlag: bool`
  - `valueType: string?` (only if not a flag)
  - `appliesTo: string` (`global` or a command name)

### CommandResult

- Fields
  - `exitCode: int` (`0`, `1`, `2`, `130`)
  - `stdout: string`
  - `stderr: string`

### NfwConfiguration

- Fields
  - `filePath: string` (expected `./nfw.yaml`)
  - `values: map<string, string>` (flattened key-value view)
  - `sources: map<string, string>` (for each key: `env` or `file`, if present)
- Validation rules
  - No required keys in skeleton phase
  - Malformed YAML does not abort execution; defaults are used
  - `NFW_*` overrides apply after file parsing

### TemplateDescriptor

- Fields
  - `name: string`
  - `description: string`
- Notes
  - Templates are sourced from a remote repository; list can be empty.

### TemplateSource

- Fields
  - `ref: string` (branch, tag, or version identifier)
  - `fetchMode: string` (`release` or `branch`)
- Notes
  - Template source is not user-configurable in the skeleton phase.
  - Debugging environments use the local `src/nfw/packages/n-framework-nfw-templates` submodule.
  - Production uses a release tag in `n-framework/nfw-templates` that matches the CLI version.

### VersionInfo

- Fields
  - `semanticVersion: string` (e.g., `0.1.0`)
  - `buildMetadata: string?` (optional; commit hash and/or build date)

## Relationships

- `CLIInvocation` selects a `CommandDefinition`, which produces a `CommandResult`.
- `CLIInvocation` loads `NfwConfiguration` (file then env overrides).
- `templates` command returns `TemplateDescriptor[]` (possibly empty).
