namespace NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;

public interface IWorkspaceArtifactWriter
{
    string GetWorkspacePath(string workspaceName);

    bool WorkspaceExists(string workspacePath);

    Task CreateWorkspace(WorkspaceArtifacts artifacts, CancellationToken cancellationToken = default);
}

public sealed record WorkspaceArtifacts(
    string WorkspacePath,
    string WorkspaceName,
    string TemplateIdentifier,
    string TemplateDisplayName,
    string TemplateDescription
);
