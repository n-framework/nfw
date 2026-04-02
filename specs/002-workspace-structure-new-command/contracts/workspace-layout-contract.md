# Workspace Layout Contract

## Canonical Root Layout

A generated workspace MUST include this layered root structure:

```text
<workspace-root>/
├── src/
├── tests/
├── docs/
├── <root-workspace-solution-files>
└── <root-yaml-configuration-files>
```

## Solution Organization Rules

- One workspace-level root solution file MUST exist.
- Per-service solution files MUST exist according to service generation rules.
- Solution naming and placement MUST be deterministic for identical input.

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
