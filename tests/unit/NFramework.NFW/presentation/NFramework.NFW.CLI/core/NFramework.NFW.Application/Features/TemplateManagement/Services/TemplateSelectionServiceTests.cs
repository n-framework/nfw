using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;
using Xunit;

namespace NFramework.NFW.CLI.Tests.core.NFramework.NFW.Application.Features.TemplateManagement.Services;

public class TemplateSelectionServiceTests
{
    [Fact]
    public void ResolveExplicitSelection_WithCaseInsensitiveExplicitIdentifier_ReturnsCanonicalTemplate()
    {
        TemplateCatalog catalog = CreateCatalog();

        TemplateSelectionResult result = TemplateSelectionService.ResolveExplicitSelection(catalog, "BLANK");

        result.IsSuccess.ShouldBeTrue();
        result.SelectedTemplate!.Identifier.ShouldBe("blank");
        result.ResolutionMode.ShouldBe(TemplateSelectionMode.Explicit);
    }

    [Fact]
    public void ResolveExplicitSelection_WithoutExplicitIdentifier_ReturnsUsageFailure()
    {
        TemplateSelectionResult result = TemplateSelectionService.ResolveExplicitSelection(CreateCatalog(), null);

        result.IsSuccess.ShouldBeFalse();
        result.Failure!.Reason.ShouldBe(TemplateSelectionFailureReason.MissingTemplateIdentifier);
        result.Failure.ExitCode.ShouldBe(ExitCodes.UsageError);
        result.Failure.Message.ShouldContain("--template");
    }

    [Fact]
    public void ResolveExplicitSelection_WithEmptyCatalog_ReturnsRuntimeFailure()
    {
        TemplateSelectionResult result = TemplateSelectionService.ResolveExplicitSelection(
            TemplateCatalog.Empty,
            "blank"
        );

        result.IsSuccess.ShouldBeFalse();
        result.Failure!.Reason.ShouldBe(TemplateSelectionFailureReason.EmptyCatalog);
        result.Failure.ExitCode.ShouldBe(ExitCodes.RuntimeFailure);
    }

    [Fact]
    public void ResolveExplicitSelection_WithUnknownIdentifier_ReturnsUsageFailure()
    {
        TemplateSelectionResult result = TemplateSelectionService.ResolveExplicitSelection(CreateCatalog(), "missing");

        result.IsSuccess.ShouldBeFalse();
        result.Failure!.Reason.ShouldBe(TemplateSelectionFailureReason.UnknownTemplateIdentifier);
        result.Failure.ExitCode.ShouldBe(ExitCodes.UsageError);
        result.Failure.Message.ShouldContain("missing");
    }

    private static TemplateCatalog CreateCatalog()
    {
        return TemplateCatalog.Create([
            TemplateDescriptor.Create("blank", "Blank Workspace", "Minimal starter"),
            TemplateDescriptor.Create("minimal", "Minimal API", "API starter"),
        ]);
    }
}
