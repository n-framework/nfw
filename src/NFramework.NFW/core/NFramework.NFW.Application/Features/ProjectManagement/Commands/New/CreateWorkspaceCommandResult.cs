using NFramework.NFW.Application.Features.Cli;

namespace NFramework.NFW.Application.Features.ProjectManagement.Commands.New;

public enum WorkspaceCreationFailureReason
{
    MissingWorkspaceName,
    WorkspaceAlreadyExists,
    MissingTemplateIdentifier,
    UnknownTemplateIdentifier,
    EmptyCatalog,
    RuntimeFailure,
}

public sealed record WorkspaceCreationFailure(
    WorkspaceCreationFailureReason Reason,
    string Message,
    int ExitCode = ExitCodes.UsageError
);

public sealed record CreatedWorkspace(
    string WorkspaceName,
    string WorkspacePath,
    string TemplateIdentifier,
    string TemplateDisplayName
);

public sealed record CreateWorkspaceCommandResult(CreatedWorkspace? Workspace, WorkspaceCreationFailure? Failure)
{
    public bool IsSuccess => Workspace is not null && Failure is null;

    public static CreateWorkspaceCommandResult Success(CreatedWorkspace workspace)
    {
        ArgumentNullException.ThrowIfNull(workspace);
        return new CreateWorkspaceCommandResult(workspace, null);
    }

    public static CreateWorkspaceCommandResult FailureResult(
        WorkspaceCreationFailureReason reason,
        string message,
        int exitCode = ExitCodes.UsageError
    )
    {
        return new CreateWorkspaceCommandResult(null, new WorkspaceCreationFailure(reason, message, exitCode));
    }
}
