using NFramework.NFW.Application.Features.Cli;

namespace NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;

public sealed record ListedTemplate(string Identifier, string DisplayName, string Description);

public sealed record ListTemplatesQueryFailure(string Message, int ExitCode = ExitCodes.RuntimeFailure);

public sealed record ListTemplatesQueryResult(
    IReadOnlyList<ListedTemplate>? Templates,
    ListTemplatesQueryFailure? Failure
)
{
    public bool IsSuccess => Failure is null;

    public static ListTemplatesQueryResult Success(IReadOnlyList<ListedTemplate> templates)
    {
        ArgumentNullException.ThrowIfNull(templates);
        return new ListTemplatesQueryResult(templates, null);
    }

    public static ListTemplatesQueryResult FailureResult(string message, int exitCode = ExitCodes.RuntimeFailure)
    {
        return new ListTemplatesQueryResult(null, new ListTemplatesQueryFailure(message, exitCode));
    }
}
