using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions;

public interface ITerminalSession
{
    bool IsInteractive { get; }

    Task<TerminalTextInputResult> PromptForWorkspaceNameAsync(CancellationToken cancellationToken);

    Task<TerminalTemplateSelectionResult> PromptForTemplateSelectionAsync(
        IReadOnlyList<ListedTemplate> templates,
        CancellationToken cancellationToken
    );
}

public sealed class TerminalTextInputResult
{
    private TerminalTextInputResult(string? value, bool wasCancelled)
    {
        Value = value;
        WasCancelled = wasCancelled;
    }

    public string? Value { get; }

    public bool WasCancelled { get; }

    public static TerminalTextInputResult Submitted(string value)
    {
        if (string.IsNullOrWhiteSpace(value))
            throw new ArgumentException("Terminal input cannot be empty or whitespace.", nameof(value));

        return new TerminalTextInputResult(value.Trim(), false);
    }

    public static TerminalTextInputResult Cancelled()
    {
        return new TerminalTextInputResult(null, true);
    }
}

public sealed class TerminalTemplateSelectionResult
{
    private TerminalTemplateSelectionResult(string? templateIdentifier, bool wasCancelled)
    {
        TemplateIdentifier = templateIdentifier;
        WasCancelled = wasCancelled;
    }

    public string? TemplateIdentifier { get; }

    public bool WasCancelled { get; }

    public static TerminalTemplateSelectionResult Selected(string templateIdentifier)
    {
        if (string.IsNullOrWhiteSpace(templateIdentifier))
            throw new ArgumentException(
                "Template identifier cannot be empty or whitespace.",
                nameof(templateIdentifier)
            );

        return new TerminalTemplateSelectionResult(templateIdentifier.Trim(), false);
    }

    public static TerminalTemplateSelectionResult Cancelled()
    {
        return new TerminalTemplateSelectionResult(null, true);
    }
}
