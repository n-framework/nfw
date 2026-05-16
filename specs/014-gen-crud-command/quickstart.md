# Quickstart: Generate CRUD Command

This guide demonstrates how to use the `nfw gen crud` command to quickly scaffold an entire feature vertical slice in your NFramework workspace.

## Prerequisites

- Ensure you are inside a valid NFramework `.NET` workspace.
- Ensure the target entity exists in your domain layer (or be prepared to let the CLI create it for you).

## Basic Usage

To generate a full CRUD implementation for a `Product` entity, run:

```bash
nfw gen crud Product
```

If you haven't specified any flags, the CLI will interactively ask you a few questions:

```text
? Generate API Endpoints? Yes
? Include caching markers? No
? Include security markers? Yes
```

In under 2 seconds, the CLI will generate all DTOs, Commands, Queries, Handlers, the Repository contract, and API endpoints, placing them in the correct `Application` and `Presentation` folders.

## Advanced Usage (CI/CD / Automation)

If you are scripting the generation or just prefer to bypass interactive prompts, use flags:

```bash
# Generate everything, skip interactive prompts, overwrite existing files
nfw gen crud Product --secured --cached --force --no-input

# Generate only Application/Domain contracts (no HTTP API endpoints)
nfw gen crud Product --no-api
```

## What Gets Generated?

For the `Product` entity, you will get:

- `Application/Features/Product/CreateProductCommand.cs` (and Update/Delete)
- `Application/Features/Product/GetProductByIdQuery.cs` (and List)
- DTOs like `ProductResponse.cs`
- `Application/Contracts/IProductRepository.cs`
- `Presentation.WebApi/Endpoints/Product/CreateProductEndpoint.cs` (and other endpoints)

## Validating Generation

Because NFramework prioritizes clean code generation, the output is guaranteed to compile. Run:

```bash
dotnet build
```

Your new CRUD flow is ready to use!
