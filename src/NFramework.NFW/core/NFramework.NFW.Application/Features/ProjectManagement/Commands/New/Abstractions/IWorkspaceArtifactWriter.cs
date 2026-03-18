namespace NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;

public interface IWorkspaceArtifactWriter
{
    string GetWorkspacePath(string workspaceName);

    bool WorkspaceExists(string workspacePath);

    void CreateWorkspace(WorkspaceArtifacts artifacts);
}

public sealed record WorkspaceArtifacts(
    string WorkspacePath,
    string WorkspaceName,
    string TemplateIdentifier,
    string TemplateDisplayName,
    string TemplateDescription
);
