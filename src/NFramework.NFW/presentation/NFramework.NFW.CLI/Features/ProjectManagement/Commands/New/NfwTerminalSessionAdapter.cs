using NFramework.Core.CLI.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;

/// <summary>
/// Adapter that wraps core-cli's ITerminalSession and provides NFW-specific prompt methods.
/// </summary>
public sealed class NfwTerminalSessionAdapter(ITerminalSession coreTerminalSession) : INfwTerminalSession
{
    private readonly ITerminalSession _coreTerminalSession = coreTerminalSession;

    public bool IsInteractive => _coreTerminalSession.IsInteractive;

    public Task<NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions.TerminalTextInputResult> PromptForWorkspaceNameAsync(
        CancellationToken cancellationToken
    )
    {
        TerminalTextPrompt prompt = new("Workspace name:", "Workspace name is required.");

        return TranslateToDomainResult(_coreTerminalSession.PromptForTextAsync(prompt, cancellationToken));
    }

    public Task<NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions.TerminalTemplateSelectionResult> PromptForTemplateSelectionAsync(
        IReadOnlyList<ListedTemplate> templates,
        CancellationToken cancellationToken
    )
    {
        TerminalSelectionOption[] options = templates
            .Select(t => new TerminalSelectionOption(
                t.Identifier,
                $"{t.Identifier} | {t.DisplayName} | {t.Description}"
            ))
            .ToArray();

        TerminalSelectionPrompt prompt = new("Select a template", options);

        return TranslateToSelectionResult(_coreTerminalSession.PromptForSelectionAsync(prompt, cancellationToken));
    }

    public void WriteLine(string message) => _coreTerminalSession.WriteLine(message);

    public void WriteErrorLine(string message) => _coreTerminalSession.WriteErrorLine(message);

    private static async Task<NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions.TerminalTextInputResult> TranslateToDomainResult(
        Task<NFramework.Core.CLI.Abstractions.TerminalTextInputResult> coreTask
    )
    {
        NFramework.Core.CLI.Abstractions.TerminalTextInputResult coreResult = await coreTask;

        return coreResult.WasCancelled
            ? NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions.TerminalTextInputResult.Cancelled()
            : NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions.TerminalTextInputResult.Submitted(
                coreResult.Value!
            );
    }

    private static async Task<NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions.TerminalTemplateSelectionResult> TranslateToSelectionResult(
        Task<NFramework.Core.CLI.Abstractions.TerminalSelectionResult> coreTask
    )
    {
        NFramework.Core.CLI.Abstractions.TerminalSelectionResult coreResult = await coreTask;

        return coreResult.WasCancelled
            ? NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions.TerminalTemplateSelectionResult.Cancelled()
            : NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions.TerminalTemplateSelectionResult.Selected(
                coreResult.SelectedValue!
            );
    }
}
