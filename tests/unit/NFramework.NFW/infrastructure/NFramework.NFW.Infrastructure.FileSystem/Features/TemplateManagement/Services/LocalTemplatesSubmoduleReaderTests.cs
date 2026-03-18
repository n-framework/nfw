using NFramework.NFW.Infrastructure.FileSystem.Features.TemplateManagement.Services;
using Xunit;

namespace NFramework.NFW.Infrastructure.FileSystem.Tests.Features.TemplateManagement.Services;

[Collection("Cli command tests")]
public sealed class LocalTemplatesSubmoduleReaderTests
{
    [Fact]
    public void TryGetCatalogPath_WhenWorkingDirectoryIsOutsideRepo_StillFindsLocalCatalog()
    {
        string originalDirectory = Directory.GetCurrentDirectory();
        string temporaryDirectory = Path.Combine(Path.GetTempPath(), Path.GetRandomFileName());
        _ = Directory.CreateDirectory(temporaryDirectory);

        try
        {
            _ = Directory.CreateDirectory(temporaryDirectory);
            Directory.SetCurrentDirectory(temporaryDirectory);

            LocalTemplatesSubmoduleReader reader = new();

            string? catalogPath = reader.TryGetCatalogPath();

            string.IsNullOrWhiteSpace(catalogPath).ShouldBeFalse();
            catalogPath.ShouldEndWith(Path.Combine("packages", "nfw-templates", "catalog.yaml"));
        }
        finally
        {
            Directory.SetCurrentDirectory(originalDirectory);
            Directory.Delete(temporaryDirectory, recursive: true);
        }
    }
}
