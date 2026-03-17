namespace NFramework.NFW.Application.Features.Templates;

public sealed class LocalTemplatesSubmoduleReader
{
    private const string CatalogFileName = "catalog.yaml";

    public static string? ReadCatalog()
    {
        var catalogPath = TryGetCatalogPath();
        if (catalogPath is null)
        {
            return null;
        }

        try
        {
            return File.ReadAllText(catalogPath);
        }
        catch (FileNotFoundException)
        {
            return null;
        }
        catch (DirectoryNotFoundException)
        {
            return null;
        }
        catch (IOException)
        {
            return null;
        }
        catch (UnauthorizedAccessException)
        {
            return null;
        }
    }

    public static string? TryGetCatalogPath()
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
            Path.Combine("src", "nfw", "packages", "nfw-templates", CatalogFileName),
            Path.Combine("packages", "nfw-templates", CatalogFileName),
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
