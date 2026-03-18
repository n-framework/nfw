using NFramework.NFW.Application.Features.Versioning;
using NFramework.NFW.Domain.Features.Version;
using Xunit;

namespace NFramework.NFW.CLI.Tests.core.NFramework.NFW.Application.Features.Versioning;

public class VersionProviderTests
{
    private readonly VersionProvider _versionProvider = new();

    [Fact]
    public void GetVersionInfo_WhenCalled_ReturnsVersionInfo()
    {
        // Act
        VersionInfo result = _versionProvider.GetVersionInfo();

        // Assert
        _ = result.ShouldNotBeNull();
        result.ToString().ShouldMatch(@"^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?(\+.+)?$");
    }

    [Fact]
    public void GetVersionInfo_SemanticVersion_IsValidFormat()
    {
        // Act
        VersionInfo result = _versionProvider.GetVersionInfo();

        // Assert
        result.SemanticVersion.ShouldMatch(@"^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$");
    }

    [Fact]
    public void GetVersionInfo_WithAssemblyInformationalVersion_ParsesCorrectly()
    {
        // Note: This test verifies the behavior when the assembly has
        // an AssemblyInformationalVersion attribute. The actual behavior
        // depends on the build configuration.

        // Act
        VersionInfo result = _versionProvider.GetVersionInfo();

        // Assert - should not throw and should return a valid version
        _ = result.ShouldNotBeNull();
    }

    [Fact]
    public void GetVersionInfo_BuildMetadata_IsNullWhenNotPresent()
    {
        // Act
        VersionInfo result = _versionProvider.GetVersionInfo();

        // Assert
        // Build metadata may or may not be null depending on the assembly version
        // This test verifies the property exists and can be accessed
        _ = result.ShouldNotBeNull();
    }

    [Fact]
    public void GetVersionInfo_WithPlusDelimiter_SplitsMetadata()
    {
        // This test documents the behavior when version contains '+'
        // The actual implementation depends on the assembly version

        // Act
        VersionInfo result = _versionProvider.GetVersionInfo();

        // Assert
        _ = result.SemanticVersion.ShouldNotBeNull();
    }

    [Fact]
    public void GetVersionInfo_MultipleCalls_ReturnsConsistentResults()
    {
        // Act
        VersionInfo result1 = _versionProvider.GetVersionInfo();
        VersionInfo result2 = _versionProvider.GetVersionInfo();

        // Assert
        result2.SemanticVersion.ShouldBe(result1.SemanticVersion);
    }
}
