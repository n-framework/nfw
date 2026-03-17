using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.Templates;
using NFramework.NFW.Application.Features.Versioning;
using Spectre.Console;
using Spectre.Console.Cli;

namespace NFramework.NFW.CLI.Features.Templates;

public sealed class TemplatesCliCommand : AsyncCommand<TemplatesCliCommandSettings>
{
    private readonly TemplatesService _templatesService;
    private readonly IVersionProvider _versionProvider;
    private readonly DiagnosticLogger _diagnosticLogger;

    public TemplatesCliCommand(
        TemplatesService templatesService,
        IVersionProvider versionProvider,
        DiagnosticLogger diagnosticLogger
    )
    {
        _templatesService = templatesService;
        _versionProvider = versionProvider;
        _diagnosticLogger = diagnosticLogger;
    }

    public override async Task<int> ExecuteAsync(
        CommandContext context,
        TemplatesCliCommandSettings settings,
        CancellationToken cancellationToken
    )
    {
        try
        {
            _diagnosticLogger.Write("Resolving template catalog source.");
            var versionInfo = _versionProvider.GetVersionInfo();
            var templates = await _templatesService.GetTemplatesAsync(versionInfo.SemanticVersion, cancellationToken);

            if (templates.Count == 0)
            {
                AnsiConsole.MarkupLine("No templates available.");
                return ExitCodes.Success;
            }

            var table = new Table().Border(TableBorder.Rounded);
            table.AddColumn("Template");
            table.AddColumn("Description");

            foreach (var template in templates)
            {
                table.AddRow(template.Name, template.Description);
            }

            AnsiConsole.Write(table);
            return ExitCodes.Success;
        }
        catch (TemplateCatalogException exception)
        {
            Console.Error.WriteLine(exception.Message);
            _diagnosticLogger.Write(exception.ToString());
            return ExitCodes.RuntimeFailure;
        }
    }
}
