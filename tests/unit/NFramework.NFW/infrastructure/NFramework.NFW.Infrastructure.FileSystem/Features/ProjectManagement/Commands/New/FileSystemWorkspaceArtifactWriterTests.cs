using Moq;
using NFramework.Core.Template.Abstractions;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Abstractions;
using NFramework.NFW.Infrastructure.FileSystem.Features.ProjectManagement.Commands.New;
using Xunit;

namespace NFramework.NFW.Infrastructure.FileSystem.Tests.Features.ProjectManagement.Commands.New;

[Collection("Cli command tests")]
public sealed class FileSystemWorkspaceArtifactWriterTests
{
    [Fact]
    public async Task CreateWorkspace_WritesExpectedFiles()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create();
        Mock<ITemplateRenderer> templateRendererMock = new();
        Mock<IWorkspaceTemplateProvider> templateProviderMock = new();
        _ = templateProviderMock
            .Setup(p => p.GetTemplateFilesAsync(It.IsAny<string>(), It.IsAny<CancellationToken>()))
            .ReturnsAsync([]);

        FileSystemWorkspaceArtifactWriter writer = new(templateRendererMock.Object, templateProviderMock.Object);
        string workspacePath = writer.GetWorkspacePath("sample");

        await writer.CreateWorkspace(
            new WorkspaceArtifacts(workspacePath, "sample", "blank", "Blank Workspace", "Minimal starter")
        );

        Directory.Exists(workspacePath).ShouldBeTrue();
        File.ReadAllText(Path.Combine(workspacePath, "nfw.yaml")).ShouldContain("template: blank");
    }

    private sealed class TemporaryWorkingDirectory : IDisposable
    {
        private readonly string _originalDirectory = Directory.GetCurrentDirectory();

        private TemporaryWorkingDirectory(string path)
        {
            Path = path;
            Directory.SetCurrentDirectory(path);
        }

        public string Path { get; }

        public static TemporaryWorkingDirectory Create()
        {
            string path = System.IO.Path.Combine(System.IO.Path.GetTempPath(), System.IO.Path.GetRandomFileName());
            _ = Directory.CreateDirectory(path);
            return new TemporaryWorkingDirectory(path);
        }

        public void Dispose()
        {
            Directory.SetCurrentDirectory(_originalDirectory);
            if (Directory.Exists(Path))
            {
                Directory.Delete(Path, recursive: true);
            }
        }
    }
}
