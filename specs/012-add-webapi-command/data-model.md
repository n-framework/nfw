# Phase 1: Data Model

## Entities

### `WebApiConfig`

Configuration structure representing the options for generating the WebAPI layer.

- `service_name`: `String` - Target service name.
- `use_openapi`: `bool` - Flag to include Swagger/OpenAPI setup (default true).
- `use_health_checks`: `bool` - Flag to include HealthChecks (default true).
- `use_cors`: `bool` - Flag to include CORS middleware (default true).
- `use_problem_details`: `bool` - Flag to include Problem Details middleware (default true).

### `WorkspaceConfig` (nfw.yaml mapping)

Relevant section of the workspace configuration.

- `services`: `HashMap<String, ServiceConfig>`
  - `ServiceConfig`: Contains `modules: Vec<String>` where "webapi" will be appended.

### `GenerationTransaction`

Tracks operations for rollback support.

- `created_files`: `Vec<PathBuf>` - Files created during the command.
- `config_backup`: `Option<String>` - Original content of `nfw.yaml` before modification.
