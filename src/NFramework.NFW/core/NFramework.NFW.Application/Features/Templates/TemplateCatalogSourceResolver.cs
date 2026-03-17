namespace NFramework.NFW.Application.Features.Templates;

public sealed class TemplateCatalogSourceResolver
{
    private readonly LocalTemplatesSubmoduleReader _localTemplatesSubmoduleReader;

    public TemplateCatalogSourceResolver(LocalTemplatesSubmoduleReader localTemplatesSubmoduleReader)
    {
        _localTemplatesSubmoduleReader = localTemplatesSubmoduleReader;
    }

    public TemplateCatalogSourceDecision Resolve(string cliVersion)
    {
#if DEBUG
        var localCatalogPath = _localTemplatesSubmoduleReader.TryGetCatalogPath();
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
