using NFramework.NFW.Application.Features.Templates;
using NFramework.NFW.Domain.Features.Templates;
using Xunit;

namespace NFramework.NFW.CLI.Tests.Application.Features.Templates;

public class TemplateCatalogParserTests
{
    private readonly TemplateCatalogParser _parser = new();

    [Fact]
    public void Parse_ValidYaml_ReturnsTemplates()
    {
        // Arrange
        var yaml = """
            templates:
              - name: web-api
                description: ASP.NET Core Web API template
              - name: worker-service
                description: Background worker service template
            """;

        // Act
        var result = _parser.Parse(yaml);

        // Assert
        Assert.Equal(2, result.Count);
        Assert.Contains(result, t => t.Name == "web-api");
        Assert.Contains(result, t => t.Name == "worker-service");
    }

    [Fact]
    public void Parse_EmptyYaml_ReturnsEmptyList()
    {
        // Arrange
        var yaml = "";

        // Act
        var result = _parser.Parse(yaml);

        // Assert
        Assert.Empty(result);
    }

    [Fact]
    public void Parse_NullYaml_ReturnsEmptyList()
    {
        // Act
        var result = _parser.Parse(null!);

        // Assert
        Assert.Empty(result);
    }

    [Fact]
    public void Parse_WhitespaceYaml_ReturnsEmptyList()
    {
        // Arrange
        var yaml = "   ";

        // Act
        var result = _parser.Parse(yaml);

        // Assert
        Assert.Empty(result);
    }

    [Fact]
    public void Parse_YamlWithNoTemplates_ReturnsEmptyList()
    {
        // Arrange
        var yaml = "templates: []";

        // Act
        var result = _parser.Parse(yaml);

        // Assert
        Assert.Empty(result);
    }

    [Fact]
    public void Parse_YamlWithNullTemplates_ReturnsEmptyList()
    {
        // Arrange
        var yaml = "other: value";

        // Act
        var result = _parser.Parse(yaml);

        // Assert
        Assert.Empty(result);
    }

    [Fact]
    public void Parse_TemplateWithNoDescription_UsesDefault()
    {
        // Arrange
        var yaml = """
            templates:
              - name: web-api
            """;

        // Act
        var result = _parser.Parse(yaml);

        // Assert
        Assert.Single(result);
        Assert.Equal("web-api", result[0].Name);
        Assert.Equal("No description provided.", result[0].Description);
    }

    [Fact]
    public void Parse_TemplateWithEmptyDescription_UsesDefault()
    {
        // Arrange
        var yaml = """
            templates:
              - name: web-api
                description: ""
            """;

        // Act
        var result = _parser.Parse(yaml);

        // Assert
        Assert.Equal("No description provided.", result[0].Description);
    }

    [Fact]
    public void Parse_InvalidYaml_ThrowsTemplateCatalogException()
    {
        // Arrange
        var yaml = """
            templates:
              - name: web-api
                description: [invalid, unclosed, list
            """;

        // Act & Assert
        Assert.Throws<TemplateCatalogException>(() => _parser.Parse(yaml));
    }

    [Fact]
    public void Parse_YamlWithInvalidSyntax_IncludesLineAndColumnInError()
    {
        // Arrange
        var yaml = """
            templates:
              - name: web-api
                description: [invalid, yaml, syntax
            """;

        // Act
        var exception = Assert.Throws<TemplateCatalogException>(() => _parser.Parse(yaml));

        // Assert
        Assert.Contains("Line", exception.Message);
        Assert.Contains("Column", exception.Message);
    }

    [Fact]
    public void Parse_SkipsTemplatesWithEmptyName()
    {
        // Arrange
        var yaml = """
            templates:
              - name: web-api
                description: Valid template
              - name: ""
                description: Invalid template
              - name: worker-service
                description: Another valid template
            """;

        // Act
        var result = _parser.Parse(yaml);

        // Assert
        Assert.Equal(2, result.Count);
        Assert.DoesNotContain(result, t => t.Name == "");
    }

    [Fact]
    public void Parse_SkipsTemplatesWithWhitespaceName()
    {
        // Arrange
        var yaml = """
            templates:
              - name: web-api
                description: Valid template
              - name: "   "
                description: Invalid template
            """;

        // Act
        var result = _parser.Parse(yaml);

        // Assert
        Assert.Single(result);
        Assert.Equal("web-api", result[0].Name);
    }
}
