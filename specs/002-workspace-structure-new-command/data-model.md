# Data Model: Workspace Structure and `nfw new` Command

## Entity: WorkspaceBlueprint

Represents canonical generated workspace structure constraints and required artifact categories.

### WorkspaceBlueprint Fields

- `root_directories` (set): Required directories at root (`src`, `tests`, `docs`)
- `baseline_config_files` (list): Required YAML config files with canonical locations
- `documentation_files` (list): Minimum generated docs (build/test quickstart pointers)
- `template_artifact_policy` (object): Template-driven artifact generation and placeholder rendering policy

### WorkspaceBlueprint Validation Rules

- Root directories must all exist after generation
- Baseline config files must be YAML
- Artifact generation must come from selected template content after placeholder rendering

## Entity: NamespaceConvention

Defines namespace derivation from workspace identity.

### NamespaceConvention Fields

- `workspace_base_namespace` (string)
- `service_suffix_rule` (string)
- `layer_suffix_rule` (string)

### NamespaceConvention Validation Rules

- Base namespace must be derivable from workspace name
- Generated namespace segments must follow documented naming rules
- Service/layer suffixes must be deterministic for same inputs

## Entity: NewCommandRequest

Normalized command input after parsing.

### NewCommandRequest Fields

- `workspace_name` (string, required)
- `template_id` (string, optional)
- `no_input` (boolean)
- `is_interactive_terminal` (boolean)
- `provided_values` (map)

### NewCommandRequest Validation Rules

- `workspace_name` must pass naming validation
- If `no_input=true`, no prompt path may execute
- Missing required values in non-interactive mode must fail before generation

## Entity: NewCommandResolution

Resolved generation inputs after defaults, prompt answers, and template lookup.

### NewCommandResolution Fields

- `workspace_name` (string)
- `template_id` (string)
- `template_cache_path` (path)
- `namespace_base` (string)
- `output_path` (path)

### NewCommandResolution Validation Rules

- Template must exist and be selectable
- Resolution must be complete before filesystem writes
- Output path must not be a non-empty existing directory

## Entity: CommandRouteDefinition

Deterministic routing map from parsed CLI input to handler behavior.

### CommandRouteDefinition Fields

- `command_path` (string)
- `accepted_options` (set)
- `invalid_combinations` (set)
- `handler_id` (string)

### CommandRouteDefinition Validation Rules

- Valid input shape maps to exactly one handler
- Invalid options/combinations produce deterministic validation errors

## State Transitions

### `nfw new` Lifecycle

1. `Parsed` -> CLI input parsed into `NewCommandRequest`
2. `Validated` -> request-level validation complete
3. `Resolved` -> prompt/default/template resolution complete
4. `Generated` -> workspace artifacts written
5. `Completed` -> success result emitted

Failure transitions:

- `Parsed` -> `FailedValidation` (invalid flags/name/template id shape)
- `Resolved` -> `FailedPrecondition` (non-empty target directory)
- `Generated` -> `FailedRuntime` (filesystem/config write error)

## Cardinality and Relationships

- One `NewCommandRequest` yields at most one `NewCommandResolution`
- One `NewCommandResolution` yields one `WorkspaceBlueprint` instance on success
- One `CommandRouteDefinition` corresponds to one handler path for this command surface
