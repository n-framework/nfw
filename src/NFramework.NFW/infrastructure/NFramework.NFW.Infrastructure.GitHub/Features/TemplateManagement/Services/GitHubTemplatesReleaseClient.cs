using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;

namespace NFramework.NFW.Infrastructure.GitHub.Features.TemplateManagement.Services;

public sealed class GitHubTemplatesReleaseClient : IRemoteTemplateCatalogSource
{
    private const string RawRepositoryBaseUrl = "https://raw.githubusercontent.com/n-framework/nfw-templates";
    private const int RequestTimeoutSeconds = 10;

    private readonly HttpClient _httpClient;

    public GitHubTemplatesReleaseClient(HttpClient httpClient)
    {
        _httpClient = httpClient;
        _httpClient.Timeout = TimeSpan.FromSeconds(RequestTimeoutSeconds);
    }

    public async Task<string> FetchCatalogAsync(string cliVersion, CancellationToken cancellationToken)
    {
        string tagName = $"v{cliVersion}";
        string catalogUrl = $"{RawRepositoryBaseUrl}/{tagName}/catalog.yaml";

        try
        {
            using HttpResponseMessage response = await _httpClient.GetAsync(catalogUrl, cancellationToken);
            if (!response.IsSuccessStatusCode)
                throw new TemplateCatalogException(
                    $"Failed to fetch templates from {catalogUrl} (HTTP {(int)response.StatusCode})."
                );

            return await response.Content.ReadAsStringAsync(cancellationToken);
        }
        catch (TemplateCatalogException)
        {
            throw;
        }
        catch (OperationCanceledException)
        {
            throw;
        }
        catch (Exception exception)
        {
            throw new TemplateCatalogException($"Unable to fetch templates from {catalogUrl}.", exception);
        }
    }
}
