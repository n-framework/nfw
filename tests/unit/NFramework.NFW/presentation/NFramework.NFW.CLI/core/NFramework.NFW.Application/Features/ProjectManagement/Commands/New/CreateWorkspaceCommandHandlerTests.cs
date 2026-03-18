using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;
using NFramework.NFW.Application.Features.Versioning.Abstractions;
using NFramework.NFW.Domain.Features.Version;
using Xunit;

namespace NFramework.NFW.CLI.Tests.core.NFramework.NFW.Application.Features.ProjectManagement.Commands.New;

public class CreateWorkspaceCommandHandlerTests
{
    [Fact]
    public async Task HandleAsync_WithValidTemplate_CreatesWorkspaceArtifacts()
    {
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        CreateWorkspaceCommandHandler handler = CreateHandler(
            """
            templates:
              - identifier: blank
                displayName: Blank Workspace
                description: Minimal starter
            """,
            workspaceWriter
        );

        CreateWorkspaceCommandResult result = await handler.HandleAsync(
            new CreateWorkspaceCommand("sample", "BLANK"),
            CancellationToken.None
        );

        result.IsSuccess.ShouldBeTrue(result.Failure?.Message);
        CreatedWorkspace createdWorkspace = result.Workspace.ShouldNotBeNull();
        WorkspaceArtifacts writtenWorkspace = workspaceWriter.CreatedWorkspace.ShouldNotBeNull();
        createdWorkspace.TemplateIdentifier.ShouldBe("blank");
        writtenWorkspace.WorkspacePath.ShouldBe("/virtual/sample");
        writtenWorkspace.TemplateIdentifier.ShouldBe("blank");
    }

    [Fact]
    public async Task HandleAsync_WithoutTemplateIdentifier_ReturnsUsageFailure()
    {
        FakeWorkspaceArtifactWriter workspaceWriter = new();
        CreateWorkspaceCommandResult result = await CreateHandler(
                """
                templates:
                  - identifier: blank
                    displayName: Blank Workspace
                    description: Minimal starter
                """,
                workspaceWriter
            )
            .HandleAsync(new CreateWorkspaceCommand("sample", string.Empty), CancellationToken.None);

        result.IsSuccess.ShouldBeFalse();
        result.Failure!.Reason.ShouldBe(WorkspaceCreationFailureReason.MissingTemplateIdentifier);
        workspaceWriter.CreatedWorkspace.ShouldBeNull();
    }

    private static CreateWorkspaceCommandHandler CreateHandler(
        string catalogContent,
        FakeWorkspaceArtifactWriter workspaceWriter
    )
    {
        ILocalTemplateCatalogSource localSource = new UnavailableLocalTemplateCatalogSource();
        return new CreateWorkspaceCommandHandler(
            new TemplatesService(
                new TemplateCatalogParser(),
                localSource,
                new FakeRemoteTemplateCatalogSource(catalogContent),
                new TemplateCatalogSourceResolver(localSource)
            ),
            workspaceWriter,
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

        public void CreateWorkspace(WorkspaceArtifacts artifacts)
        {
            CreatedWorkspace = artifacts;
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
