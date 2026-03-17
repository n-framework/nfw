using NFramework.NFW.Domain.Features.Version;
using Xunit;

namespace NFramework.NFW.CLI.Tests.Domain.Features.Version;

public class VersionInfoTests
{
    [Fact]
    public void Create_ValidSemanticVersion_ReturnsVersionInfo()
    {
        // Act
        var result = VersionInfo.Create("1.2.3");

        // Assert
        Assert.Equal("1.2.3", result.SemanticVersion);
        Assert.Null(result.BuildMetadata);
    }

    [Fact]
    public void Create_ValidSemanticVersionWithPrerelease_ReturnsVersionInfo()
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
    public void Create_TrimsSemanticVersion()
    {
        // Act
        var result = VersionInfo.Create("  1.2.3  ");

        // Assert
        Assert.Equal("1.2.3", result.SemanticVersion);
    }

    [Fact]
    public void Create_TrimsBuildMetadata()
    {
        // Act
        var result = VersionInfo.Create("1.2.3", "  20210101  ");

        // Assert
        Assert.Equal("20210101", result.BuildMetadata);
    }

    [Fact]
    public void Create_EmptySemanticVersion_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => VersionInfo.Create(""));
    }

    [Fact]
    public void Create_WhitespaceSemanticVersion_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => VersionInfo.Create("   "));
    }

    [Fact]
    public void Create_NullSemanticVersion_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => VersionInfo.Create(null!));
    }

    [Fact]
    public void Create_InvalidSemanticVersion_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => VersionInfo.Create("invalid"));
    }

    [Fact]
    public void Create_InvalidSemanticVersionMajorOnly_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => VersionInfo.Create("1"));
    }

    [Fact]
    public void Create_InvalidSemanticVersionMinorOnly_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => VersionInfo.Create("1.2"));
    }

    [Fact]
    public void CreateDefault_ReturnsDefaultVersion()
    {
        // Act
        var result = VersionInfo.CreateDefault();

        // Assert
        Assert.Equal("0.1.0", result.SemanticVersion);
        Assert.Null(result.BuildMetadata);
    }

    [Fact]
    public void ToString_WithoutBuildMetadata_ReturnsSemanticVersion()
    {
        // Arrange
        var versionInfo = VersionInfo.Create("1.2.3");

        // Act
        var result = versionInfo.ToString();

        // Assert
        Assert.Equal("1.2.3", result);
    }

    [Fact]
    public void ToString_WithBuildMetadata_ReturnsSemanticVersionPlusMetadata()
    {
        // Arrange
        var versionInfo = VersionInfo.Create("1.2.3", "20210101");

        // Act
        var result = versionInfo.ToString();

        // Assert
        Assert.Equal("1.2.3+20210101", result);
    }

    [Fact]
    public void ToString_WithEmptyBuildMetadata_ReturnsSemanticVersion()
    {
        // Arrange
        var versionInfo = new VersionInfo("1.2.3", "");

        // Act
        var result = versionInfo.ToString();

        // Assert
        Assert.Equal("1.2.3", result);
    }
}
