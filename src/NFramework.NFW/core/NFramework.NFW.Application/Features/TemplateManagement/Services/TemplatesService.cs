using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;
using NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;

namespace NFramework.NFW.Application.Features.TemplateManagement.Services;

public sealed class TemplatesService(
    TemplateCatalogParser templateCatalogParser,
    ILocalTemplateCatalogSource localTemplateCatalogSource,
    IRemoteTemplateCatalogSource remoteTemplateCatalogSource,
    TemplateCatalogSourceResolver templateCatalogSourceResolver
)
{
    private readonly TemplateCatalogParser _templateCatalogParser = templateCatalogParser;
    private readonly ILocalTemplateCatalogSource _localTemplateCatalogSource = localTemplateCatalogSource;
    private readonly IRemoteTemplateCatalogSource _remoteTemplateCatalogSource = remoteTemplateCatalogSource;
    private readonly TemplateCatalogSourceResolver _templateCatalogSourceResolver = templateCatalogSourceResolver;

    public async Task<TemplateCatalog> GetCatalogAsync(string cliVersion, CancellationToken cancellationToken)
    {
        TemplateCatalogSourceDecision sourceDecision = _templateCatalogSourceResolver.Resolve(cliVersion);
        string? catalogContent = sourceDecision.SourceKind switch
        {
            TemplateCatalogSourceKind.LocalDebugSubmodule => _localTemplateCatalogSource.ReadCatalog(),
            TemplateCatalogSourceKind.RemoteReleaseTag => await _remoteTemplateCatalogSource.FetchCatalogAsync(
                cliVersion,
                cancellationToken
            ),
            _ => throw new TemplateCatalogException("Template catalog source is not supported."),
        };

        if (string.IsNullOrWhiteSpace(catalogContent))
        {
            return TemplateCatalog.Empty;
        }

        return _templateCatalogParser.Parse(catalogContent);
    }
}
