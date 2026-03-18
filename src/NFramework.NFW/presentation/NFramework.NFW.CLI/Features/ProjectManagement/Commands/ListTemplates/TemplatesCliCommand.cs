using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using Spectre.Console;
using Spectre.Console.Cli;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.ListTemplates;

public sealed class TemplatesCliCommand(ListTemplatesQueryHandler listTemplatesQueryHandler, IAnsiConsole console)
    : AsyncCommand<TemplatesCliCommandSettings>
{
    private readonly ListTemplatesQueryHandler _listTemplatesQueryHandler = listTemplatesQueryHandler;
    private readonly IAnsiConsole _console = console;

    public override async Task<int> ExecuteAsync(
        CommandContext context,
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
            Console.Error.WriteLine(result.Failure!.Message);
            return result.Failure.ExitCode;
        }

        IReadOnlyList<ListedTemplate> templates = result.Templates!;
        if (templates.Count == 0)
        {
            _console.MarkupLine("No templates available.");
            return ExitCodes.Success;
        }

        Table table = new Table().Border(TableBorder.Rounded);
        _ = table.AddColumn("Identifier");
        _ = table.AddColumn("Name");
        _ = table.AddColumn("Description");

        foreach (ListedTemplate template in templates)
            _ = table.AddRow(template.Identifier, template.DisplayName, template.Description);

        _console.Write(table);
        return ExitCodes.Success;
    }
}
