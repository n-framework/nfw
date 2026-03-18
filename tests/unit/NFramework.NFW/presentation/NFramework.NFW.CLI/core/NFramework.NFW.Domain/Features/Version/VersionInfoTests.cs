using NFramework.NFW.Domain.Features.Version;
using Xunit;

namespace NFramework.NFW.CLI.Tests.core.NFramework.NFW.Domain.Features.Version;

public class VersionInfoTests
{
    [Fact]
    public void Create_ValidSemanticVersion_ReturnsVersionInfo()
    {
        // Act
        VersionInfo result = VersionInfo.Create("1.2.3");

        // Assert
        result.SemanticVersion.ShouldBe("1.2.3");
        result.BuildMetadata.ShouldBeNull();
    }

    [Fact]
    public void Create_ValidSemanticVersionWithPrerelease_ReturnsVersionInfo()
    {
        // Act
        VersionInfo result = VersionInfo.Create("1.2.3-alpha.1");

        // Assert
        result.SemanticVersion.ShouldBe("1.2.3-alpha.1");
    }

    [Fact]
    public void Create_WithBuildMetadata_ReturnsVersionInfo()
    {
        // Act
        VersionInfo result = VersionInfo.Create("1.2.3", "20210101");

        // Assert
        result.SemanticVersion.ShouldBe("1.2.3");
        result.BuildMetadata.ShouldBe("20210101");
    }

    [Fact]
    public void Create_TrimsSemanticVersion()
    {
        // Act
        VersionInfo result = VersionInfo.Create("  1.2.3  ");

        // Assert
        result.SemanticVersion.ShouldBe("1.2.3");
    }

    [Fact]
    public void Create_TrimsBuildMetadata()
    {
        // Act
        VersionInfo result = VersionInfo.Create("1.2.3", "  20210101  ");

        // Assert
        result.BuildMetadata.ShouldBe("20210101");
    }

    [Fact]
    public void Create_EmptySemanticVersion_ThrowsArgumentException()
    {
        // Act & Assert
        _ = Should.Throw<ArgumentException>(() => VersionInfo.Create(""));
    }

    [Fact]
    public void Create_WhitespaceSemanticVersion_ThrowsArgumentException()
    {
        // Act & Assert
        _ = Should.Throw<ArgumentException>(() => VersionInfo.Create("   "));
    }

    [Fact]
    public void Create_NullSemanticVersion_ThrowsArgumentException()
    {
        // Act & Assert
        _ = Should.Throw<ArgumentException>(() => VersionInfo.Create(null!));
    }

    [Fact]
    public void Create_InvalidSemanticVersion_ThrowsArgumentException()
    {
        // Act & Assert
        _ = Should.Throw<ArgumentException>(() => VersionInfo.Create("invalid"));
    }

    [Fact]
    public void Create_InvalidSemanticVersionMajorOnly_ThrowsArgumentException()
    {
        // Act & Assert
        _ = Should.Throw<ArgumentException>(() => VersionInfo.Create("1"));
    }

    [Fact]
    public void Create_InvalidSemanticVersionMinorOnly_ThrowsArgumentException()
    {
        // Act & Assert
        _ = Should.Throw<ArgumentException>(() => VersionInfo.Create("1.2"));
    }

    [Fact]
    public void CreateDefault_ReturnsDefaultVersion()
    {
        // Act
        VersionInfo result = VersionInfo.CreateDefault();

        // Assert
        result.SemanticVersion.ShouldBe("0.1.0");
        result.BuildMetadata.ShouldBeNull();
    }

    [Fact]
    public void ToString_WithoutBuildMetadata_ReturnsSemanticVersion()
    {
        // Arrange
        VersionInfo versionInfo = VersionInfo.Create("1.2.3");

        // Act
        string result = versionInfo.ToString();

        // Assert
        result.ShouldBe("1.2.3");
    }

    [Fact]
    public void ToString_WithBuildMetadata_ReturnsSemanticVersionPlusMetadata()
    {
        // Arrange
        VersionInfo versionInfo = VersionInfo.Create("1.2.3", "20210101");

        // Act
        string result = versionInfo.ToString();

        // Assert
        result.ShouldBe("1.2.3+20210101");
    }

    [Fact]
    public void ToString_WithEmptyBuildMetadata_ReturnsSemanticVersion()
    {
        // Arrange
        VersionInfo versionInfo = new("1.2.3", "");

        // Act
        string result = versionInfo.ToString();

        // Assert
        result.ShouldBe("1.2.3");
    }
}
