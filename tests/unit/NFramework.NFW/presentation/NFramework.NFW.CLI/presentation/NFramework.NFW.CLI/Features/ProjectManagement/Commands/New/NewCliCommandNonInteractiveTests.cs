using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;
using NFramework.NFW.Application.Features.Versioning.Abstractions;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Domain.Features.Version;
using Spectre.Console;
using Xunit;

namespace NFramework.NFW.CLI.Tests.presentation.NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;

[Collection("Cli command tests")]
public class NewCliCommandNonInteractiveTests
{
    [Fact]
    public async Task ExecuteAsync_WithExplicitTemplate_CreatesWorkspaceWithoutPrompt()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            new FakeTerminalSession(isInteractive: false)
        );

        int exitCode = await command.ExecuteAsync(
            null!,
            new NewCliCommandSettings { WorkspaceName = "sample-explicit", TemplateIdentifier = "BLANK" },
            CancellationToken.None
        );

        WorkspaceArtifacts createdWorkspace = workspaceWriter.CreatedWorkspace.ShouldNotBeNull();
        exitCode.ShouldBe(0);
        createdWorkspace.WorkspaceName.ShouldBe("sample-explicit");
        createdWorkspace.TemplateIdentifier.ShouldBe("blank");
        output.ToString().ShouldContain("blank");
    }

    [Fact]
    public async Task ExecuteAsync_WithoutWorkspaceNameInNonInteractiveSession_FailsBeforeCreatingFiles()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            new FakeTerminalSession(isInteractive: false)
        );

        int exitCode = await command.ExecuteAsync(null!, new NewCliCommandSettings(), CancellationToken.None);

        exitCode.ShouldBe(2);
        workspaceWriter.CreatedWorkspace.ShouldBeNull();
        output.ToString().ShouldNotContain("Created workspace");
        stderr.CapturedText.ShouldContain("Workspace name is required");
    }

    [Fact]
    public async Task ExecuteAsync_WithoutTemplateInNonInteractiveSession_FailsBeforeCreatingFiles()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            new FakeTerminalSession(isInteractive: false)
        );

        int exitCode = await command.ExecuteAsync(
            null!,
            new NewCliCommandSettings { WorkspaceName = "sample-missing-template" },
            CancellationToken.None
        );

        exitCode.ShouldBe(2);
        workspaceWriter.CreatedWorkspace.ShouldBeNull();
        stderr.CapturedText.ShouldContain("--template");
    }

    [Fact]
    public async Task ExecuteAsync_WithNoInputAndMissingTemplate_FailsBeforeCreatingFiles()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            new FakeTerminalSession(isInteractive: false)
        );

        int exitCode = await command.ExecuteAsync(
            null!,
            new NewCliCommandSettings { WorkspaceName = "sample-no-input", NoInput = true },
            CancellationToken.None
        );

        exitCode.ShouldBe(2);
        workspaceWriter.CreatedWorkspace.ShouldBeNull();
        stderr.CapturedText.ShouldContain("--template");
        stderr.CapturedText.ShouldContain("interactive prompts are disabled");
    }

    [Fact]
    public async Task ExecuteAsync_WithUnknownTemplate_FailsBeforeCreatingFiles()
    {
        using ConsoleErrorScope stderr = new ConsoleErrorScope();
        StringWriter output = new();
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        NewCliCommand command = new(
            CreateListTemplatesQueryHandler(),
            CreateCreateWorkspaceCommandHandler(workspaceWriter),
            new FakeTerminalSession(isInteractive: false)
        );

        int exitCode = await command.ExecuteAsync(
            null!,
            new NewCliCommandSettings { WorkspaceName = "sample-unknown-template", TemplateIdentifier = "missing" },
            CancellationToken.None
        );

        exitCode.ShouldBe(2);
        workspaceWriter.CreatedWorkspace.ShouldBeNull();
        stderr.CapturedText.ShouldContain("missing");
        stderr.CapturedText.ShouldContain("nfw templates");
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

    private sealed class FakeTerminalSession(bool isInteractive) : INfwTerminalSession
    {
        public bool IsInteractive { get; } = isInteractive;

        public Task<TerminalTextInputResult> PromptForWorkspaceNameAsync(CancellationToken cancellationToken)
        {
            throw new InvalidOperationException("Prompting should not occur in non-interactive tests.");
        }

        public Task<TerminalTemplateSelectionResult> PromptForTemplateSelectionAsync(
            IReadOnlyList<ListedTemplate> templates,
            CancellationToken cancellationToken
        )
        {
            throw new InvalidOperationException("Prompting should not occur in non-interactive tests.");
        }

        public void WriteLine(string message) { }

        public void WriteErrorLine(string message) { }
    }
}
