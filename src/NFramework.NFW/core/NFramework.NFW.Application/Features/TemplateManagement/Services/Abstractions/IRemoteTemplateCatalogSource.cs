namespace NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;

public interface IRemoteTemplateCatalogSource
{
    Task<string> FetchCatalogAsync(string cliVersion, CancellationToken cancellationToken);
}
