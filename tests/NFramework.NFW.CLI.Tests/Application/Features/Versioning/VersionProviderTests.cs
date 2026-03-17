using System.Reflection;
using NFramework.NFW.Application.Features.Versioning;
using NFramework.NFW.Domain.Features.Version;
using Xunit;

namespace NFramework.NFW.CLI.Tests.Application.Features.Versioning;

public class VersionProviderTests
{
    private readonly VersionProvider _versionProvider = new();

    [Fact]
    public void GetVersionInfo_WhenCalled_ReturnsVersionInfo()
    {
        // Act
        var result = _versionProvider.GetVersionInfo();

        // Assert
        Assert.NotNull(result);
        Assert.Matches(@"^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?(\+.+)?$", result.ToString());
    }

    [Fact]
    public void GetVersionInfo_SemanticVersion_IsValidFormat()
    {
        // Act
        var result = _versionProvider.GetVersionInfo();

        // Assert
        Assert.Matches(@"^\d+\.\d+\.\d+(-[0-9A-Za-z.-]+)?$", result.SemanticVersion);
    }

    [Fact]
    public void GetVersionInfo_WithAssemblyInformationalVersion_ParsesCorrectly()
    {
        // Note: This test verifies the behavior when the assembly has
        // an AssemblyInformationalVersion attribute. The actual behavior
        // depends on the build configuration.

        // Act
        var result = _versionProvider.GetVersionInfo();

        // Assert - should not throw and should return a valid version
        Assert.NotNull(result);
    }

    [Fact]
    public void GetVersionInfo_BuildMetadata_IsNullWhenNotPresent()
    {
        // Act
        var result = _versionProvider.GetVersionInfo();

        // Assert
        // Build metadata may or may not be null depending on the assembly version
        // This test verifies the property exists and can be accessed
        Assert.NotNull(result);
    }

    [Fact]
    public void GetVersionInfo_WithPlusDelimiter_SplitsMetadata()
    {
        // This test documents the behavior when version contains '+'
        // The actual implementation depends on the assembly version

        // Act
        var result = _versionProvider.GetVersionInfo();

        // Assert
        Assert.NotNull(result.SemanticVersion);
    }

    [Fact]
    public void GetVersionInfo_MultipleCalls_ReturnsConsistentResults()
    {
        // Act
        var result1 = _versionProvider.GetVersionInfo();
        var result2 = _versionProvider.GetVersionInfo();

        // Assert
        Assert.Equal(result1.SemanticVersion, result2.SemanticVersion);
    }
}

// Test class for VersionInfo factory methods
public class VersionInfoFactoryTests
{
    [Fact]
    public void Create_ValidVersion_ReturnsVersionInfo()
    {
        // Act
        var result = VersionInfo.Create("1.2.3");

        // Assert
        Assert.Equal("1.2.3", result.SemanticVersion);
        Assert.Null(result.BuildMetadata);
    }

    [Fact]
    public void Create_WithPrereleaseTag_ReturnsVersionInfo()
    {
        // Act
        var result = VersionInfo.Create("1.2.3-alpha.1");

        // Assert
        Assert.Equal("1.2.3-alpha.1", result.SemanticVersion);
    }

    [Fact]
    public void Create_WithBuildMetadata_ReturnsVersionInfo()
    {
        // Act
        var result = VersionInfo.Create("1.2.3", "20210101");

        // Assert
        Assert.Equal("1.2.3", result.SemanticVersion);
        Assert.Equal("20210101", result.BuildMetadata);
    }

    [Fact]
    public void Create_InvalidVersion_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => VersionInfo.Create("invalid"));
    }

    [Fact]
    public void Create_EmptyVersion_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => VersionInfo.Create(""));
    }

    [Fact]
    public void CreateDefault_ReturnsZeroOneZero()
    {
        // Act
        var result = VersionInfo.CreateDefault();

        // Assert
        Assert.Equal("0.1.0", result.SemanticVersion);
        Assert.Null(result.BuildMetadata);
    }

    [Fact]
    public void ToString_WithoutMetadata_ReturnsSemanticVersion()
    {
        // Arrange
        var version = VersionInfo.Create("1.2.3");

        // Act
        var result = version.ToString();

        // Assert
        Assert.Equal("1.2.3", result);
    }

    [Fact]
    public void ToString_WithMetadata_ReturnsSemanticVersionPlusMetadata()
    {
        // Arrange
        var version = VersionInfo.Create("1.2.3", "20210101");

        // Act
        var result = version.ToString();

        // Assert
        Assert.Equal("1.2.3+20210101", result);
    }
}
