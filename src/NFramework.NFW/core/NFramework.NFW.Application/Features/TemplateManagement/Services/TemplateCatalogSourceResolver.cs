using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;

namespace NFramework.NFW.Application.Features.TemplateManagement.Services;

public sealed class TemplateCatalogSourceResolver(ILocalTemplateCatalogSource localTemplateCatalogSource)
{
    private readonly ILocalTemplateCatalogSource _localTemplateCatalogSource = localTemplateCatalogSource;

    public TemplateCatalogSourceDecision Resolve(string cliVersion)
    {
#if DEBUG
        string? localCatalogPath = _localTemplateCatalogSource.TryGetCatalogPath();
        if (!string.IsNullOrWhiteSpace(localCatalogPath))
        {
            return new TemplateCatalogSourceDecision(TemplateCatalogSourceKind.LocalDebugSubmodule, localCatalogPath);
        }
#endif
        return new TemplateCatalogSourceDecision(TemplateCatalogSourceKind.RemoteReleaseTag, $"v{cliVersion}");
    }
}

public enum TemplateCatalogSourceKind
{
    LocalDebugSubmodule,
    RemoteReleaseTag,
}

public sealed record TemplateCatalogSourceDecision(TemplateCatalogSourceKind SourceKind, string Reference);
