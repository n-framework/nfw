using NFramework.Core.CLI.Abstractions;
using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.ListTemplates;

[CliCommand("templates", "List available templates")]
public sealed class TemplatesCliCommand(
    ListTemplatesQueryHandler listTemplatesQueryHandler,
    ITerminalSession terminalSession
) : IAsyncCliCommand<TemplatesCliCommandSettings>
{
    private readonly ListTemplatesQueryHandler _listTemplatesQueryHandler = listTemplatesQueryHandler;
    private readonly ITerminalSession _terminalSession = terminalSession;

    public async Task<int> ExecuteAsync(
        CliCommandContext context,
        TemplatesCliCommandSettings settings,
        CancellationToken cancellationToken
    )
    {
        ListTemplatesQueryResult result = await _listTemplatesQueryHandler.HandleAsync(
            new ListTemplatesQuery(),
            cancellationToken
        );
        if (!result.IsSuccess)
        {
            _terminalSession.WriteErrorLine(result.Failure!.Message);
            return result.Failure.ExitCode;
        }

        IReadOnlyList<ListedTemplate> templates = result.Templates!;
        if (templates.Count == 0)
        {
            _terminalSession.WriteLine("No templates available.");
            return ExitCodes.Success;
        }

        TerminalTable table = new TerminalTable(
            ["Identifier", "Name", "Description"],
            templates.Select(t => new TerminalTableRow([t.Identifier, t.DisplayName, t.Description])).ToList()
        );

        _terminalSession.RenderTable(table);
        return ExitCodes.Success;
    }
}
