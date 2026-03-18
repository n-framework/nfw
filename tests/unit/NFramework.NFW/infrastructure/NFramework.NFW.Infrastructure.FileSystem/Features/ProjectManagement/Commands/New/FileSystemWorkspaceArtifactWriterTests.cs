using NFramework.NFW.Application.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Infrastructure.FileSystem.Features.ProjectManagement.Commands.New;
using Xunit;

namespace NFramework.NFW.Infrastructure.FileSystem.Tests.Features.ProjectManagement.Commands.New;

[Collection("Cli command tests")]
public sealed class FileSystemWorkspaceArtifactWriterTests
{
    [Fact]
    public void CreateWorkspace_WritesExpectedFiles()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create();
        FileSystemWorkspaceArtifactWriter writer = new();
        string workspacePath = writer.GetWorkspacePath("sample");

        writer.CreateWorkspace(
            new WorkspaceArtifacts(workspacePath, "sample", "blank", "Blank Workspace", "Minimal starter")
        );

        Directory.Exists(workspacePath).ShouldBeTrue();
        File.ReadAllText(Path.Combine(workspacePath, "nfw.yaml")).ShouldContain("template: blank");
        File.ReadAllText(Path.Combine(workspacePath, "README.md")).ShouldContain("Blank Workspace");
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
