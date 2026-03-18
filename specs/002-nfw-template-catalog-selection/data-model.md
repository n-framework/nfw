# Data Model: Template Catalog Listing and Selection

## Template Catalog

**Purpose**: Represents the full ordered set of templates available for listing and selection.

**Fields**:

- `entries`: ordered collection of `Template Entry`
- `source`: catalog origin used only for diagnostics and debugging

**Validation Rules**:

- The catalog may be empty for listing, but workspace creation cannot continue without at least one selectable entry.
- Entry identifiers must be unique across the catalog using case-insensitive comparison.
- The catalog order is preserved after validation and becomes the authoritative order for listing and prompting.

## Template Entry

**Purpose**: Represents one user-selectable starter template.

**Fields**:

- `identifier`: stable public selection key used in CLI arguments and error messages
- `displayName`: human-readable label shown in listings and interactive prompts
- `description`: short explanation of the template’s intended use

**Validation Rules**:

- `identifier` is required, trimmed, and non-empty.
- `identifier` must remain stable across catalog revisions unless intentionally removed or deprecated.
- `displayName` must be present in user-facing output; if the catalog omits a dedicated display label, the identifier is used as the fallback label.
- `description` defaults to a safe placeholder only when the catalog omits it; blank descriptions are not surfaced as empty output.

## Template Selection Context

**Purpose**: Captures the inputs required to resolve one workspace-creation template choice.

**Fields**:

- `workspaceName`: requested workspace name
- `explicitTemplateIdentifier`: optional identifier supplied by the user
- `isInteractiveSession`: whether both input and output are attached to a terminal
- `availableEntries`: validated ordered catalog entries

**Validation Rules**:

- If `explicitTemplateIdentifier` is present, it is resolved before any prompt is considered.
- If `explicitTemplateIdentifier` is absent and `isInteractiveSession` is `true`, the CLI requests a user choice from `availableEntries`.
- If `explicitTemplateIdentifier` is absent and `isInteractiveSession` is `false`, resolution fails as a usage error before file generation begins.
- Unknown identifiers fail resolution before any workspace files are created.

## Template Selection Result

**Purpose**: Represents the outcome of template resolution before workspace generation begins.

**Fields**:

- `selectedEntry`: the chosen `Template Entry` when resolution succeeds
- `resolutionMode`: `explicit` or `interactive`
- `failureReason`: absent on success; present for missing identifier, unknown identifier, empty catalog, or interruption

**State Transitions**:

1. `pending` -> `catalog-validated`
2. `catalog-validated` -> `selected` when explicit resolution succeeds
3. `catalog-validated` -> `prompting` when interactive selection is required
4. `prompting` -> `selected` when the user chooses an entry
5. `catalog-validated` or `prompting` -> `failed` when the identifier is invalid, the catalog is unusable, or the session is non-interactive without an explicit selection
6. `prompting` -> `cancelled` when the user interrupts before selecting a template
