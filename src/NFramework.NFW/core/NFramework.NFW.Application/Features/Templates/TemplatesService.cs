using NFramework.NFW.Domain.Features.Templates;

namespace NFramework.NFW.Application.Features.Templates;

public sealed class TemplatesService
{
    private readonly TemplateCatalogParser _templateCatalogParser;
    private readonly LocalTemplatesSubmoduleReader _localTemplatesSubmoduleReader;
    private readonly GitHubTemplatesReleaseClient _gitHubTemplatesReleaseClient;
    private readonly TemplateCatalogSourceResolver _templateCatalogSourceResolver;

    public TemplatesService(
        TemplateCatalogParser templateCatalogParser,
        LocalTemplatesSubmoduleReader localTemplatesSubmoduleReader,
        GitHubTemplatesReleaseClient gitHubTemplatesReleaseClient,
        TemplateCatalogSourceResolver templateCatalogSourceResolver
    )
    {
        _templateCatalogParser = templateCatalogParser;
        _localTemplatesSubmoduleReader = localTemplatesSubmoduleReader;
        _gitHubTemplatesReleaseClient = gitHubTemplatesReleaseClient;
        _templateCatalogSourceResolver = templateCatalogSourceResolver;
    }

    public async Task<IReadOnlyList<TemplateDescriptor>> GetTemplatesAsync(
        string cliVersion,
        CancellationToken cancellationToken
    )
    {
        var sourceDecision = _templateCatalogSourceResolver.Resolve(cliVersion);
        var catalogContent = sourceDecision.SourceKind switch
        {
            TemplateCatalogSourceKind.LocalDebugSubmodule => _localTemplatesSubmoduleReader.ReadCatalog(),
            TemplateCatalogSourceKind.RemoteReleaseTag => await _gitHubTemplatesReleaseClient.FetchCatalogAsync(
                cliVersion,
                cancellationToken
            ),
            _ => throw new TemplateCatalogException("Template catalog source is not supported."),
        };

        if (string.IsNullOrWhiteSpace(catalogContent))
        {
            return [];
        }

        return _templateCatalogParser.Parse(catalogContent);
    }
}
