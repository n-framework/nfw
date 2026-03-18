using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;

public sealed class CliTerminalSession(InteractiveTemplatePrompt interactiveTemplatePrompt) : ITerminalSession
{
    private readonly InteractiveTemplatePrompt _interactiveTemplatePrompt = interactiveTemplatePrompt;

    public bool IsInteractive => !Console.IsInputRedirected && !Console.IsOutputRedirected;

    public Task<TerminalTextInputResult> PromptForWorkspaceNameAsync(CancellationToken cancellationToken)
    {
        cancellationToken.ThrowIfCancellationRequested();
        return Task.FromResult(_interactiveTemplatePrompt.PromptForWorkspaceName());
    }

    public Task<TerminalTemplateSelectionResult> PromptForTemplateSelectionAsync(
        IReadOnlyList<ListedTemplate> templates,
        CancellationToken cancellationToken
    )
    {
        cancellationToken.ThrowIfCancellationRequested();
        return Task.FromResult(_interactiveTemplatePrompt.PromptForTemplateSelection(templates));
    }
}
