using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;

namespace NFramework.NFW.Infrastructure.FileSystem.Features.TemplateManagement.Services;

public sealed class LocalTemplatesSubmoduleReader : ILocalTemplateCatalogSource
{
    private const string CatalogFileName = "catalog.yaml";

    public string? ReadCatalog()
    {
        string? catalogPath = TryGetCatalogPath();
        if (catalogPath is null)
            return null;

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

    public string? TryGetCatalogPath()
    {
        foreach (string root in EnumerateSearchRoots())
        {
            foreach (string relativePath in RelativeCatalogPaths)
            {
                string candidatePath = Path.Combine(root, relativePath);
                if (File.Exists(candidatePath))
                    return candidatePath;
            }
        }

        return null;
    }

    private static IEnumerable<string> RelativeCatalogPaths =>
        [
            Path.Combine("src", "nfw", "packages", "nfw-templates", CatalogFileName),
            Path.Combine("packages", "nfw-templates", CatalogFileName),
        ];

    private static IEnumerable<string> EnumerateSearchRoots()
    {
        HashSet<string> visitedRoots = new(StringComparer.OrdinalIgnoreCase);
        foreach (string startDirectory in EnumerateStartDirectories())
        {
            foreach (string root in EnumerateCurrentAndParents(startDirectory))
            {
                if (visitedRoots.Add(root))
                    yield return root;
            }
        }
    }

    private static IEnumerable<string> EnumerateStartDirectories()
    {
        yield return Directory.GetCurrentDirectory();

        string baseDirectory = AppContext.BaseDirectory;
        if (!string.IsNullOrWhiteSpace(baseDirectory))
            yield return baseDirectory;
    }

    private static IEnumerable<string> EnumerateCurrentAndParents(string startDirectory)
    {
        DirectoryInfo? current = new(startDirectory);
        while (current is not null)
        {
            yield return current.FullName;
            current = current.Parent;
        }
    }
}
