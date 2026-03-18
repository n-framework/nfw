using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;

namespace NFramework.NFW.Application.Features.TemplateManagement.Services;

public enum TemplateSelectionMode
{
    Explicit,
}

public enum TemplateSelectionFailureReason
{
    MissingTemplateIdentifier,
    UnknownTemplateIdentifier,
    EmptyCatalog,
}

public sealed record TemplateSelectionFailure(
    TemplateSelectionFailureReason Reason,
    string Message,
    int ExitCode = ExitCodes.UsageError
);

public sealed record TemplateSelectionResult(
    TemplateDescriptor? SelectedTemplate,
    TemplateSelectionMode? ResolutionMode,
    TemplateSelectionFailure? Failure
)
{
    public bool IsSuccess => SelectedTemplate is not null && Failure is null;

    public static TemplateSelectionResult Success(
        TemplateDescriptor selectedTemplate,
        TemplateSelectionMode resolutionMode
    )
    {
        ArgumentNullException.ThrowIfNull(selectedTemplate);
        return new TemplateSelectionResult(selectedTemplate, resolutionMode, null);
    }

    public static TemplateSelectionResult FailureResult(
        TemplateSelectionFailureReason reason,
        string message,
        int exitCode = ExitCodes.UsageError
    )
    {
        return new TemplateSelectionResult(null, null, new TemplateSelectionFailure(reason, message, exitCode));
    }
}
