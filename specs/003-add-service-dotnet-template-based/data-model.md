# Data Model: Template-Based `nfw add service`

## Entity: AddServiceCommandRequest

Normalized command input after parsing.

### AddServiceCommandRequest Fields

- `service_name` (string, required)
- `template_id` (string, optional at parse stage)
- `no_input` (boolean)
- `is_interactive_terminal` (boolean)

### AddServiceCommandRequest Validation Rules

- `service_name` must satisfy naming rules
- If non-interactive (`no_input=true` or no TTY), `template_id` is required
- Unknown/invalid option combinations fail before generation

## Entity: ServiceTemplateResolution

Resolved template selection and metadata from existing template system.

### ServiceTemplateResolution Fields

- `template_id` (string)
- `resolved_version` (semantic version)
- `template_type` (string; must be `service`)
- `template_cache_path` (path)

### ServiceTemplateResolution Validation Rules

- Template must exist in catalog/cache
- `template_type` must equal `service`
- Resolved version must be fully determined before writes

## Entity: ServiceGenerationPlan

Filesystem generation plan derived from template + request.

### ServiceGenerationPlan Fields

- `service_name` (string)
- `output_root` (path, fixed `src/<ServiceName>/`)
- `render_operations` (list)
- `placeholder_values` (map)

### ServiceGenerationPlan Validation Rules

- `output_root` must not already exist with conflicting content
- Placeholder set must satisfy template requirements
- Plan must be complete before filesystem writes begin

## Entity: LayerDependencyMatrix

Contract defining allowed project references for generated service layers.

### LayerDependencyMatrix Fields

- `domain_allowed_refs` (set)
- `application_allowed_refs` (set)
- `infrastructure_allowed_refs` (set)
- `api_allowed_refs` (set)

### LayerDependencyMatrix Validation Rules

- `Domain` references none of service-layer projects
- `Application` references only `Domain`
- `Infrastructure` references only `Application` and `Domain`
- `Api` references only `Application` and `Infrastructure`

## Entity: ServiceTemplateProvenanceRecord

Workspace metadata record for template traceability.

### ServiceTemplateProvenanceRecord Fields

- `service_name` (string)
- `template_id` (string)
- `template_version` (semantic version)
- `generated_at_utc` (timestamp)

### ServiceTemplateProvenanceRecord Validation Rules

- Record must be written to `nfw.yaml` on successful generation
- `service_name` key must match generated service root
- `template_id` + `template_version` must match resolved template

## State Transitions

### `nfw add service` Lifecycle

1. `Parsed` -> CLI parsed into `AddServiceCommandRequest`
2. `Validated` -> request-level validation complete
3. `TemplateResolved` -> `ServiceTemplateResolution` complete
4. `Planned` -> `ServiceGenerationPlan` complete
5. `Generated` -> files written to `src/<ServiceName>/`
6. `ProvenancePersisted` -> `nfw.yaml` updated
7. `Completed` -> success result emitted

Failure transitions:

- `Parsed` -> `FailedValidation` (invalid name/options)
- `Validated` -> `FailedPrecondition` (missing template in non-interactive flow)
- `TemplateResolved` -> `FailedTemplateValidation` (missing template, wrong type)
- `Generated` -> `FailedRuntime` (render/write failure)
- any state after write start -> `FailedWithCleanup` (partial output removed)

## Cardinality and Relationships

- One `AddServiceCommandRequest` yields at most one `ServiceTemplateResolution`
- One `ServiceTemplateResolution` yields one `ServiceGenerationPlan`
- One successful `ServiceGenerationPlan` yields one generated service root and one `ServiceTemplateProvenanceRecord`
- One `LayerDependencyMatrix` applies to each generated service in this feature scope
