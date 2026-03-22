using NFramework.Core.CLI.Abstractions;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;
using NFramework.NFW.Application.Features.Versioning.Abstractions;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Domain.Features.Version;
using Spectre.Console;
using Xunit;

namespace NFramework.NFW.CLI.Tests.presentation.NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;

[Collection("Cli command tests")]
public class NewCliCommandInteractiveTests
{
    [Fact]
    public async Task ExecuteAsync_WithoutWorkspaceName_PromptsForWorkspaceNameAndTemplate()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        FakeTerminalSession terminalSession = new(
            output,
            isInteractive: true,
            workspaceNameResult: TerminalTextInputResult.Submitted("prompted-workspace"),
            templateSelectionResult: TerminalSelectionResult.Selected("blank")
        );
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            terminalSession
        );

        int exitCode = await command.ExecuteAsync(null!, new NewCliCommandSettings(), CancellationToken.None);

        exitCode.ShouldBe(0);
        terminalSession.WorkspacePromptCount.ShouldBe(1);
        terminalSession.TemplatePromptCount.ShouldBe(1);
        workspaceWriter.CreatedWorkspace!.WorkspaceName.ShouldBe("prompted-workspace");
    }

    [Fact]
    public async Task ExecuteAsync_WithNoInputAndMissingWorkspaceName_FailsBeforePrompting()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        FakeTerminalSession terminalSession = new(
            output,
            isInteractive: true,
            workspaceNameResult: TerminalTextInputResult.Submitted("ignored-workspace"),
            templateSelectionResult: TerminalSelectionResult.Selected("blank")
        );
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            terminalSession
        );

        int exitCode = await command.ExecuteAsync(
            null!,
            new NewCliCommandSettings { NoInput = true },
            CancellationToken.None
        );

        exitCode.ShouldBe(2);
        terminalSession.WorkspacePromptCount.ShouldBe(0);
        terminalSession.TemplatePromptCount.ShouldBe(0);
        workspaceWriter.CreatedWorkspace.ShouldBeNull();
        stderr.CapturedText.ShouldContain("interactive prompts are disabled");
    }

    [Fact]
    public async Task ExecuteAsync_WhenWorkspacePromptIsCancelled_DoesNotPromptForTemplate()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        FakeTerminalSession terminalSession = new(
            output,
            isInteractive: true,
            workspaceNameResult: TerminalTextInputResult.Cancelled(),
            templateSelectionResult: TerminalSelectionResult.Selected("blank")
        );
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            terminalSession
        );

        int exitCode = await command.ExecuteAsync(null!, new NewCliCommandSettings(), CancellationToken.None);

        exitCode.ShouldBe(130);
        terminalSession.WorkspacePromptCount.ShouldBe(1);
        terminalSession.TemplatePromptCount.ShouldBe(0);
        workspaceWriter.CreatedWorkspace.ShouldBeNull();
        stderr.CapturedText.ShouldContain("cancelled");
    }

    [Fact]
    public async Task ExecuteAsync_WithoutTemplateInInteractiveSession_PromptsAndCreatesWorkspace()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        FakeTerminalSession terminalSession = new(
            output,
            isInteractive: true,
            templateSelectionResult: TerminalSelectionResult.Selected("minimal")
        );
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            terminalSession
        );

        int exitCode = await command.ExecuteAsync(
            null!,
            new NewCliCommandSettings { WorkspaceName = "sample-interactive" },
            CancellationToken.None
        );

        exitCode.ShouldBe(0);
        terminalSession.WorkspacePromptCount.ShouldBe(0);
        terminalSession.TemplatePromptCount.ShouldBe(1);
        workspaceWriter.CreatedWorkspace!.TemplateIdentifier.ShouldBe("minimal");
        output.ToString().ShouldContain("interactive");
    }

    [Fact]
    public async Task ExecuteAsync_WhenInteractiveSelectionIsCancelled_DoesNotCreateWorkspace()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        FakeTerminalSession terminalSession = new(
            output,
            isInteractive: true,
            templateSelectionResult: TerminalSelectionResult.Cancelled()
        );
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            terminalSession
        );

        int exitCode = await command.ExecuteAsync(
            null!,
            new NewCliCommandSettings { WorkspaceName = "sample-cancelled" },
            CancellationToken.None
        );

        exitCode.ShouldBe(130);
        terminalSession.TemplatePromptCount.ShouldBe(1);
        workspaceWriter.CreatedWorkspace.ShouldBeNull();
    }

    private static ListTemplatesQueryHandler CreateListTemplatesQueryHandler()
    {
        ILocalTemplateCatalogSource localSource = new UnavailableLocalTemplateCatalogSource();
        return new ListTemplatesQueryHandler(
            new TemplatesService(
                new TemplateCatalogParser(),
                localSource,
                new FakeRemoteTemplateCatalogSource(
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
                new TemplateCatalogSourceResolver(localSource)
            ),
            new FakeVersionProvider(),
            new DiagnosticLogger()
        );
    }

    private static CreateWorkspaceCommandHandler CreateCreateWorkspaceCommandHandler(
        FakeWorkspaceArtifactWriter workspaceWriter
    )
    {
        ILocalTemplateCatalogSource localSource = new UnavailableLocalTemplateCatalogSource();
        return new CreateWorkspaceCommandHandler(
            new TemplatesService(
                new TemplateCatalogParser(),
                localSource,
                new FakeRemoteTemplateCatalogSource(
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
                new TemplateCatalogSourceResolver(localSource)
            ),
            workspaceWriter,
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

    private sealed class FakeWorkspaceArtifactWriter : IWorkspaceArtifactWriter
    {
        public WorkspaceArtifacts? CreatedWorkspace { get; private set; }

        public string GetWorkspacePath(string workspaceName)
        {
            return $"/virtual/{workspaceName}";
        }

        public bool WorkspaceExists(string workspacePath)
        {
            return false;
        }

        public Task CreateWorkspace(WorkspaceArtifacts artifacts, CancellationToken cancellationToken = default)
        {
            CreatedWorkspace = artifacts;
            return Task.CompletedTask;
        }
    }

    private sealed class FakeVersionProvider : IVersionProvider
    {
        public VersionInfo GetVersionInfo()
        {
            return VersionInfo.Create("1.2.3");
        }
    }

    private sealed class FakeTerminalSession(
        StringWriter output,
        bool isInteractive,
        TerminalTextInputResult? workspaceNameResult = null,
        TerminalSelectionResult? templateSelectionResult = null
    ) : ITerminalSession
    {
        private readonly TerminalSelectionResult _templateSelectionResult =
            templateSelectionResult ?? TerminalSelectionResult.Cancelled();
        private readonly TerminalTextInputResult _workspaceNameResult =
            workspaceNameResult ?? TerminalTextInputResult.Submitted("sample-interactive");

        public bool IsInteractive { get; } = isInteractive;

        public int WorkspacePromptCount { get; private set; }

        public int TemplatePromptCount { get; private set; }

        public Task<TerminalTextInputResult> PromptForTextAsync(
            TerminalTextPrompt prompt,
            CancellationToken cancellationToken
        )
        {
            WorkspacePromptCount += 1;
            return Task.FromResult(_workspaceNameResult);
        }

        public Task<TerminalSelectionResult> PromptForSelectionAsync(
            TerminalSelectionPrompt prompt,
            CancellationToken cancellationToken
        )
        {
            TemplatePromptCount += 1;
            return Task.FromResult(_templateSelectionResult);
        }

        public void RenderTable(TerminalTable table) { }

        public void WriteLine(string message) => output.WriteLine(message);

        public void WriteErrorLine(string message) => Console.Error.WriteLine(message);
    }
}
