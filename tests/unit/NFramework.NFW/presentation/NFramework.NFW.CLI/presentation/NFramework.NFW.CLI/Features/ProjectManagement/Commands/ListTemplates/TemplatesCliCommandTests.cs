using NFramework.Core.CLI.Abstractions;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;
using NFramework.NFW.Application.Features.Versioning.Abstractions;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.ListTemplates;
using NFramework.NFW.Domain.Features.Version;
using Spectre.Console;
using Xunit;

namespace NFramework.NFW.CLI.Tests.presentation.NFramework.NFW.CLI.Features.ProjectManagement.Commands.ListTemplates;

[Collection("Cli command tests")]
public class TemplatesCliCommandTests
{
    [Fact]
    public async Task ExecuteAsync_RendersIdentifierDisplayNameAndDescriptionInCatalogOrder()
    {
        StringWriter output = new();
        FakeTerminalSession terminalSession = new(output);
        TemplatesCliCommand command = new(
            CreateListTemplatesQueryHandler(
                """
                templates:
                  - identifier: blank
                    displayName: Blank Workspace
                    description: Minimal starter
                  - identifier: minimal
                    displayName: Minimal API
                    description: API starter
                """
            ),
            terminalSession
        );

        int exitCode = await command.ExecuteAsync(null!, new TemplatesCliCommandSettings(), CancellationToken.None);
        string rendered = output.ToString();

        exitCode.ShouldBe(0);
        rendered.ShouldContain("blank");
        rendered.ShouldContain("Blank Workspace");
        rendered.ShouldContain("Minimal starter");
        (
            rendered.IndexOf("blank", StringComparison.Ordinal) < rendered.IndexOf("minimal", StringComparison.Ordinal)
        ).ShouldBeTrue();
    }

    [Fact]
    public async Task ExecuteAsync_WithEmptyCatalog_WritesNoTemplatesMessage()
    {
        StringWriter output = new();
        FakeTerminalSession terminalSession = new(output);
        TemplatesCliCommand command = new(CreateListTemplatesQueryHandler("templates: []"), terminalSession);

        int exitCode = await command.ExecuteAsync(null!, new TemplatesCliCommandSettings(), CancellationToken.None);

        exitCode.ShouldBe(0);
        output.ToString().ShouldContain("No templates available");
    }

    private static ListTemplatesQueryHandler CreateListTemplatesQueryHandler(string catalogContent)
    {
        ILocalTemplateCatalogSource localSource = new UnavailableLocalTemplateCatalogSource();
        return new ListTemplatesQueryHandler(
            new TemplatesService(
                new TemplateCatalogParser(),
                localSource,
                new FakeRemoteTemplateCatalogSource(catalogContent),
                new TemplateCatalogSourceResolver(localSource)
            ),
            new FakeVersionProvider(),
            new DiagnosticLogger()
        );
    }

    private sealed class UnavailableLocalTemplateCatalogSource : ILocalTemplateCatalogSource
    {
        public string? ReadCatalog() => null;

        public string? TryGetCatalogPath() => null;
    }

    private sealed class FakeRemoteTemplateCatalogSource(string catalogContent) : IRemoteTemplateCatalogSource
    {
        public Task<string> FetchCatalogAsync(string cliVersion, CancellationToken cancellationToken)
        {
            return Task.FromResult(catalogContent);
        }
    }

    private sealed class FakeVersionProvider : IVersionProvider
    {
        public VersionInfo GetVersionInfo()
        {
            return VersionInfo.Create("1.2.3");
        }
    }

    private sealed class FakeTerminalSession(StringWriter output) : ITerminalSession
    {
        public bool IsInteractive => false;

        public Task<TerminalTextInputResult> PromptForTextAsync(
            TerminalTextPrompt prompt,
            CancellationToken cancellationToken
        )
        {
            throw new InvalidOperationException("Prompting should not occur in templates tests.");
        }

        public Task<TerminalSelectionResult> PromptForSelectionAsync(
            TerminalSelectionPrompt prompt,
            CancellationToken cancellationToken
        )
        {
            throw new InvalidOperationException("Prompting should not occur in templates tests.");
        }

        public void RenderTable(TerminalTable table)
        {
            IAnsiConsole console = AnsiConsole.Create(
                new AnsiConsoleSettings
                {
                    Ansi = AnsiSupport.No,
                    ColorSystem = ColorSystemSupport.NoColors,
                    Out = new AnsiConsoleOutput(output),
                }
            );

            Spectre.Console.Table spectreTable = new();
            foreach (string column in table.Columns)
            {
                _ = spectreTable.AddColumn(column);
            }

            foreach (TerminalTableRow row in table.Rows)
            {
                _ = spectreTable.AddRow(row.Cells.ToArray());
            }

            console.Live(spectreTable).Start(_ => { });
        }

        public void WriteLine(string message) => output.WriteLine(message);

        public void WriteErrorLine(string message) => output.WriteLine(message);
    }
}
