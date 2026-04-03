# Workspace Layout Contract

## Canonical Root Layout

A generated workspace MUST include this layered root structure:

```text
<workspace-root>/
├── src/
├── tests/
├── docs/
├── <template-generated-artifacts>
└── <root-yaml-configuration-files>
```

## Template Artifact Rules

- Generated artifacts MUST come from selected template `content/` tree.
- Placeholder rendering in paths and file contents MUST be deterministic for identical input.
- The workspace generator MUST NOT hardcode solution file creation.

## Namespace Rules

- Namespace base MUST derive from workspace identity.
- Service and layer suffixes MUST be applied consistently.
- Namespace output MUST be stable for identical inputs.

## Configuration Rules

- Baseline configuration format is YAML only.
- Generated baseline configuration files MUST be placed at documented root locations.
- Non-YAML baseline config generation is out of scope for this feature.

## Failure Rules

- If output path exists and is non-empty, generation MUST fail before writes.
- In `--no-input` mode, missing required values MUST fail before writes.
- Invalid command shape MUST fail with actionable diagnostics.
