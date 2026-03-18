using System.Text;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;

namespace NFramework.NFW.Infrastructure.FileSystem.Features.ProjectManagement.Commands.New;

public sealed class FileSystemWorkspaceArtifactWriter : IWorkspaceArtifactWriter
{
    public string GetWorkspacePath(string workspaceName)
    {
        return Path.GetFullPath(workspaceName);
    }

    public bool WorkspaceExists(string workspacePath)
    {
        return Directory.Exists(workspacePath);
    }

    public void CreateWorkspace(WorkspaceArtifacts artifacts)
    {
        _ = Directory.CreateDirectory(artifacts.WorkspacePath);

        string configurationContent = $"""
            workspace:
              name: {artifacts.WorkspaceName}
              template: {artifacts.TemplateIdentifier}
            """;
        File.WriteAllText(Path.Combine(artifacts.WorkspacePath, "nfw.yaml"), configurationContent, Encoding.UTF8);

        string readmeContent = $"""
            # {artifacts.WorkspaceName}

            Created by `nfw new` using template `{artifacts.TemplateIdentifier}`.

            Template: {artifacts.TemplateDisplayName}
            Description: {artifacts.TemplateDescription}
            """;
        File.WriteAllText(Path.Combine(artifacts.WorkspacePath, "README.md"), readmeContent, Encoding.UTF8);
    }
}
