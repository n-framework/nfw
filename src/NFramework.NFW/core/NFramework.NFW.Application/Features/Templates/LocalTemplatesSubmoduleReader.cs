namespace NFramework.NFW.Application.Features.Templates;

public sealed class LocalTemplatesSubmoduleReader
{
    private const string CatalogFileName = "catalog.yaml";

    public string? ReadCatalog()
    {
        var catalogPath = TryGetCatalogPath();
        return catalogPath is null ? null : File.ReadAllText(catalogPath);
    }

    public string? TryGetCatalogPath()
    {
        foreach (var root in EnumerateCurrentAndParents(Directory.GetCurrentDirectory()))
        {
            foreach (var relativePath in RelativeCatalogPaths)
            {
                var candidatePath = Path.Combine(root, relativePath);
                if (File.Exists(candidatePath))
                {
                    return candidatePath;
                }
            }
        }

        return null;
    }

    private static IEnumerable<string> RelativeCatalogPaths =>
        [
            Path.Combine("src", "nfw", "packages", "n-framework-nfw-templates", CatalogFileName),
            Path.Combine("packages", "n-framework-nfw-templates", CatalogFileName),
        ];

    private static IEnumerable<string> EnumerateCurrentAndParents(string startDirectory)
    {
        var current = new DirectoryInfo(startDirectory);
        while (current is not null)
        {
            yield return current.FullName;
            current = current.Parent;
        }
    }
}
