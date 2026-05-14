# Quickstart: Gen Endpoint Command

## Usage

Generate an endpoint by referencing an existing feature and command/query name:

```bash
# General syntax
nfw gen endpoint <OPERATION_TYPE> <NAME> <FEATURE>

# Examples
nfw gen endpoint GET GetInventoryItem Inventory
nfw gen endpoint POST CreateProduct Catalog
nfw gen endpoint PUT UpdateOrder Orders
nfw gen endpoint DELETE DeleteUser Management
```

## Generated Artifacts

This will generate an endpoint file like `Api/Endpoints/Inventory/GetInventoryItemEndpoint.cs` containing:

```csharp
namespace Catalog.Presentation.WebApi.Endpoints.Inventory;

public static class GetInventoryItemEndpoint
{
    public static void MapGetInventoryItemEndpoint(this IEndpointRouteBuilder app)
    {
        app.MapGet("/api/inventory/getinventoryitem", async ([AsParameters] GetInventoryItem request, ISender sender) =>
        {
            var result = await sender.Send(request);
            return result.IsSuccess ? Results.Ok(result.Value) : Results.NotFound();
        })
        .WithName("GetInventoryItem")
        .WithTags("Inventory")
        .Produces<object>(StatusCodes.Status200OK);
    }
}
```
