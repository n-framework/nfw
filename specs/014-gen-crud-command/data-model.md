# Data Model: Generate CRUD Command

The `nfw gen crud` command orchestrates code generation but does not persist state itself. The "data model" here refers to the inputs required by the command and the structure of the generated output artifacts.

## CLI Command Inputs

| Input         | Type    | Description                                     | Required |
| ------------- | ------- | ----------------------------------------------- | -------- |
| `entity_name` | String  | The name of the target entity (e.g., `Product`) | Yes      |
| `--no-api`    | Boolean | Skips generating Presentation layer endpoints   | No       |
| `--secured`   | Boolean | Adds security markers/attributes to handlers    | No       |
| `--cached`    | Boolean | Adds caching behavior markers to handlers       | No       |
| `--force`     | Boolean | Overwrites existing files without prompting     | No       |

## Orchestrated Artifacts (Generated Files)

The command generates a standard Clean Architecture vertical slice for the specified entity.

### Application Layer (`Application/Features/[Entity]/`)

- **DTOs**: `Create[Entity]Request.cs`, `Update[Entity]Request.cs`, `[Entity]Response.cs`
- **Commands**: `Create[Entity]Command.cs`, `Update[Entity]Command.cs`, `Delete[Entity]Command.cs`
- **Queries**: `Get[Entity]ByIdQuery.cs`, `List[Entity]sQuery.cs`
- **Handlers**: Corresponding `*Handler.cs` for all Commands and Queries

### Domain/Contract Layer (`Domain/Repositories/` or `Application/Contracts/`)

- **Repository Interface**: `I[Entity]Repository.cs`

### Presentation/Api Layer (`Presentation.WebApi/Endpoints/[Entity]/`)

- **Endpoints**: `Create[Entity]Endpoint.cs`, `Update[Entity]Endpoint.cs`, `Delete[Entity]Endpoint.cs`, `Get[Entity]ByIdEndpoint.cs`, `List[Entity]sEndpoint.cs` (Unless `--no-api` is passed)

## State Transitions (Interactive Mode)

1. **Start**: User runs `nfw gen crud <Entity>`
2. **Validation State**:
   - Check if `<Entity>` exists in the domain layer.
   - If missing: Prompt "Entity &lt;Entity&gt; not found. Create it now? (Y/n)". If yes, trigger entity generation. If no, abort.
3. **Check Existing State**:
   - Check if feature folder / CRUD files already exist.
   - If yes: Prompt "Files exist. Overwrite? (y/N)". If yes, set overwrite flag. If no, abort or skip existing.
4. **Options State**:
   - Check if optional flags were provided.
   - If missing: Prompt for options ("Generate API Endpoints?", "Include caching?", "Include security?").
5. **Generation State**: Execute template rendering for all required artifacts.
6. **End**: Report success and execution time.
