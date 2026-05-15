# Data Model: Generator-Based `nfw add service`

## Entity: AddServiceCommandRequest

Normalized command input after parsing.

### AddServiceCommandRequest Fields

- `service_name` (string, required)
- `generator_id` (string, optional at parse stage)
- `no_input` (boolean)
- `is_interactive_terminal` (boolean)

### AddServiceCommandRequest Validation Rules

- `service_name` must satisfy naming rules
- If non-interactive (`no_input=true` or no TTY), `generator_id` is required
- Unknown/invalid option combinations fail before generation

## Entity: ServiceGeneratorResolution

Resolved generator selection and metadata from existing generator system.

### ServiceGeneratorResolution Fields

- `generator_id` (string)
- `resolved_version` (semantic version)
- `generator_type` (string; must be `service`)
- `generator_cache_path` (path)

### ServiceGeneratorResolution Validation Rules

- Generator must exist in catalog/cache
- `generator_type` must equal `service`
- Resolved version must be fully determined before writes

## Entity: ServiceGenerationPlan

Filesystem generation plan derived from generator + request.

### ServiceGenerationPlan Fields

- `service_name` (string)
- `output_root` (path, fixed `src/<ServiceName>/`)
- `render_operations` (list)
- `placeholder_values` (map)

### ServiceGenerationPlan Validation Rules

- `output_root` must not already exist with conflicting content
- Placeholder set must satisfy generator requirements
- Plan must be complete before filesystem writes begin

## Entity: ServiceGeneratorProvenanceRecord

Workspace metadata record for generator traceability.

### ServiceGeneratorProvenanceRecord Fields

- `service_name` (string)
- `generator_id` (string)
- `generator_version` (semantic version)
- `generated_at_utc` (timestamp)

### ServiceGeneratorProvenanceRecord Validation Rules

- Record must be written to `nfw.yaml` on successful generation
- `service_name` key must match generated service root
- `generator_id` + `generator_version` must match resolved generator

## State Transitions

### `nfw add service` Lifecycle

1. `Parsed` -> CLI parsed into `AddServiceCommandRequest`
2. `Validated` -> request-level validation complete
3. `GeneratorResolved` -> `ServiceGeneratorResolution` complete
4. `Planned` -> `ServiceGenerationPlan` complete
5. `Generated` -> files written to `src/<ServiceName>/`
6. `ProvenancePersisted` -> `nfw.yaml` updated
7. `Completed` -> success result emitted

Failure transitions:

- `Parsed` -> `FailedValidation` (invalid name/options)
- `Validated` -> `FailedPrecondition` (missing generator in non-interactive flow)
- `GeneratorResolved` -> `FailedGeneratorValidation` (missing generator, wrong type)
- `Generated` -> `FailedRuntime` (render/write failure)
- any state after write start -> `FailedWithCleanup` (partial output removed)

## Cardinality and Relationships

- One `AddServiceCommandRequest` yields at most one `ServiceGeneratorResolution`
- One `ServiceGeneratorResolution` yields one `ServiceGenerationPlan`
- One successful `ServiceGenerationPlan` yields one generated service root and one `ServiceGeneratorProvenanceRecord`
