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
            (Core.CLI.Abstractions.ITerminalSession)CreateConsole(output)
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
        TemplatesCliCommand command = new(CreateListTemplatesQueryHandler("templates: []"), (Core.CLI.Abstractions.ITerminalSession)CreateConsole(output));

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

    private static IAnsiConsole CreateConsole(StringWriter output)
    {
        return AnsiConsole.Create(
            new AnsiConsoleSettings
            {
                Ansi = AnsiSupport.No,
                ColorSystem = ColorSystemSupport.NoColors,
                Out = new AnsiConsoleOutput(output),
            }
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
}
