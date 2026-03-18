using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;

namespace NFramework.NFW.Application.Features.TemplateManagement.Services;

public sealed class TemplateSelectionService
{
    public static TemplateSelectionResult ResolveExplicitSelection(
        TemplateCatalog catalog,
        string? explicitTemplateIdentifier
    )
    {
        ArgumentNullException.ThrowIfNull(catalog);

        if (catalog.Entries.Count == 0)
        {
            return TemplateSelectionResult.FailureResult(
                TemplateSelectionFailureReason.EmptyCatalog,
                "No templates are available. Run `nfw templates` after restoring a catalog source.",
                ExitCodes.RuntimeFailure
            );
        }

        if (string.IsNullOrWhiteSpace(explicitTemplateIdentifier))
        {
            return TemplateSelectionResult.FailureResult(
                TemplateSelectionFailureReason.MissingTemplateIdentifier,
                "Template selection requires `--template <identifier>`."
            );
        }

        TemplateDescriptor? selectedTemplate = catalog.Entries.FirstOrDefault(template =>
            string.Equals(template.Identifier, explicitTemplateIdentifier, StringComparison.OrdinalIgnoreCase)
        );

        if (selectedTemplate is null)
        {
            return TemplateSelectionResult.FailureResult(
                TemplateSelectionFailureReason.UnknownTemplateIdentifier,
                $"Template '{explicitTemplateIdentifier}' is not available. Run `nfw templates` to see valid identifiers."
            );
        }

        return TemplateSelectionResult.Success(selectedTemplate, TemplateSelectionMode.Explicit);
    }
}
