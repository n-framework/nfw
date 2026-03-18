namespace NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;

public interface ILocalTemplateCatalogSource
{
    string? ReadCatalog();

    string? TryGetCatalogPath();
}
