using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;
using Xunit;

namespace NFramework.NFW.CLI.Tests.core.NFramework.NFW.Application.Features.TemplateManagement.Services;

public class TemplateCatalogParserTests
{
    private readonly TemplateCatalogParser _parser = new();

    [Fact]
    public void Parse_WithStableIdentifierMetadata_ReturnsOrderedCatalog()
    {
        string yaml = """
            templates:
              - identifier: blank
                displayName: Blank Workspace
                description: Minimal starter
              - identifier: minimal
                displayName: Minimal API
                description: API starter
            """;

        TemplateCatalog result = _parser.Parse(yaml);

        result.Entries.Count.ShouldBe(2);
        result.Entries[0].Identifier.ShouldBe("blank");
        result.Entries[0].DisplayName.ShouldBe("Blank Workspace");
        result.Entries[1].Identifier.ShouldBe("minimal");
    }

    [Fact]
    public void Parse_WithLegacyNameField_UsesNameAsIdentifierAndDisplayName()
    {
        string yaml = """
            templates:
              - name: blank
                description: Minimal starter
            """;

        TemplateCatalog result = _parser.Parse(yaml);

        result.Entries.Count.ShouldBe(1);
        TemplateDescriptor entry = result.Entries[0];
        entry.Identifier.ShouldBe("blank");
        entry.DisplayName.ShouldBe("blank");
    }

    [Fact]
    public void Parse_WithDuplicateIdentifiersIgnoringCase_ThrowsTemplateCatalogException()
    {
        string yaml = """
            templates:
              - identifier: blank
                displayName: Blank Workspace
              - identifier: BLANK
                displayName: Duplicate Blank
            """;

        TemplateCatalogException exception = Should.Throw<TemplateCatalogException>(() => _parser.Parse(yaml));

        exception.Message.ShouldContain("Duplicate template identifier");
    }

    [Fact]
    public void Parse_WithMissingDescription_UsesDefaultDescription()
    {
        string yaml = """
            templates:
              - identifier: blank
                displayName: Blank Workspace
            """;

        TemplateCatalog result = _parser.Parse(yaml);

        result.Entries[0].Description.ShouldBe("No description provided.");
    }

    [Fact]
    public void Parse_InvalidYaml_ThrowsTemplateCatalogException()
    {
        string yaml = """
            templates:
              - identifier: blank
                description: [invalid, yaml, syntax
            """;

        TemplateCatalogException exception = Should.Throw<TemplateCatalogException>(() => _parser.Parse(yaml));

        exception.Message.ShouldContain("Line");
        exception.Message.ShouldContain("Column");
    }
}
