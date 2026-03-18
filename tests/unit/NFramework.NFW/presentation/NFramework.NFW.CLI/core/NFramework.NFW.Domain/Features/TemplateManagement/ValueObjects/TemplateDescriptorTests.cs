using NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;
using Xunit;

namespace NFramework.NFW.CLI.Tests.core.NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;

public class TemplateDescriptorTests
{
    [Fact]
    public void Create_WithExplicitDisplayNameAndDescription_ReturnsNormalizedDescriptor()
    {
        TemplateDescriptor descriptor = TemplateDescriptor.Create(
            "  blank  ",
            "  Blank Workspace  ",
            "  Minimal starter  "
        );

        descriptor.Identifier.ShouldBe("blank");
        descriptor.DisplayName.ShouldBe("Blank Workspace");
        descriptor.Description.ShouldBe("Minimal starter");
    }

    [Fact]
    public void Create_WithoutDisplayName_UsesIdentifierAsFallback()
    {
        TemplateDescriptor descriptor = TemplateDescriptor.Create("blank", null, "Starter");

        descriptor.DisplayName.ShouldBe("blank");
    }

    [Fact]
    public void Create_WithoutDescription_UsesDefaultDescription()
    {
        TemplateDescriptor descriptor = TemplateDescriptor.Create("blank", "Blank Workspace", null);

        descriptor.Description.ShouldBe("No description provided.");
    }

    [Fact]
    public void Create_WithoutIdentifier_ThrowsArgumentException()
    {
        _ = Should.Throw<ArgumentException>(() => TemplateDescriptor.Create("   ", "Blank Workspace", "Starter"));
    }
}
