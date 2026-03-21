using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;

public sealed class InteractiveTemplatePrompt(INfwTerminalSession terminalSession)
{
    private readonly INfwTerminalSession _terminalSession = terminalSession;

    public async Task<TerminalTemplateSelectionResult> PromptForTemplateSelectionAsync(
        IReadOnlyList<ListedTemplate> templates,
        CancellationToken cancellationToken
    )
    {
        return await _terminalSession.PromptForTemplateSelectionAsync(templates, cancellationToken);
    }

    public async Task<TerminalTextInputResult> PromptForWorkspaceNameAsync(CancellationToken cancellationToken)
    {
        return await _terminalSession.PromptForWorkspaceNameAsync(cancellationToken);
    }
}
