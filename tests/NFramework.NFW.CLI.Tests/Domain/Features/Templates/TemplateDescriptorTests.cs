using NFramework.NFW.Domain.Features.Templates;
using Xunit;

namespace NFramework.NFW.CLI.Tests.Domain.Features.Templates;

public class TemplateDescriptorTests
{
    [Fact]
    public void Create_ValidNameAndDescription_ReturnsTemplateDescriptor()
    {
        // Act
        var result = TemplateDescriptor.Create("my-template", "A test template");

        // Assert
        Assert.Equal("my-template", result.Name);
        Assert.Equal("A test template", result.Description);
    }

    [Fact]
    public void Create_WithNullDescription_UsesDefaultDescription()
    {
        // Act
        var result = TemplateDescriptor.Create("my-template", null);

        // Assert
        Assert.Equal("my-template", result.Name);
        Assert.Equal("No description provided.", result.Description);
    }

    [Fact]
    public void Create_WithEmptyDescription_UsesDefaultDescription()
    {
        // Act
        var result = TemplateDescriptor.Create("my-template", "");

        // Assert
        Assert.Equal("my-template", result.Name);
        Assert.Equal("No description provided.", result.Description);
    }

    [Fact]
    public void Create_WithWhitespaceDescription_UsesDefaultDescription()
    {
        // Act
        var result = TemplateDescriptor.Create("my-template", "   ");

        // Assert
        Assert.Equal("my-template", result.Name);
        Assert.Equal("No description provided.", result.Description);
    }

    [Fact]
    public void Create_TrimsName()
    {
        // Act
        var result = TemplateDescriptor.Create("  my-template  ", "description");

        // Assert
        Assert.Equal("my-template", result.Name);
    }

    [Fact]
    public void Create_TrimsDescription()
    {
        // Act
        var result = TemplateDescriptor.Create("my-template", "  description  ");

        // Assert
        Assert.Equal("description", result.Description);
    }

    [Fact]
    public void Create_EmptyName_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => TemplateDescriptor.Create("", "description"));
    }

    [Fact]
    public void Create_WhitespaceName_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => TemplateDescriptor.Create("   ", "description"));
    }

    [Fact]
    public void Create_NullName_ThrowsArgumentException()
    {
        // Act & Assert
        Assert.Throws<ArgumentException>(() => TemplateDescriptor.Create(null!, "description"));
    }
}
