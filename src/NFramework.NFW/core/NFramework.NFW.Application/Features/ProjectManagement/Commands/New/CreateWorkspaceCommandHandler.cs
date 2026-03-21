using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Application.Features.Versioning.Abstractions;
using NFramework.NFW.Domain.Features.TemplateManagement.ValueObjects;
using NFramework.NFW.Domain.Features.Version;

namespace NFramework.NFW.Application.Features.ProjectManagement.Commands.New;

public sealed class CreateWorkspaceCommandHandler(
    TemplatesService templatesService,
    IWorkspaceArtifactWriter workspaceArtifactWriter,
    IVersionProvider versionProvider,
    DiagnosticLogger diagnosticLogger
)
{
    private readonly TemplatesService _templatesService = templatesService;
    private readonly IWorkspaceArtifactWriter _workspaceArtifactWriter = workspaceArtifactWriter;
    private readonly IVersionProvider _versionProvider = versionProvider;
    private readonly DiagnosticLogger _diagnosticLogger = diagnosticLogger;

    public async Task<CreateWorkspaceCommandResult> HandleAsync(
        CreateWorkspaceCommand command,
        CancellationToken cancellationToken
    )
    {
        ArgumentNullException.ThrowIfNull(command);

        if (string.IsNullOrWhiteSpace(command.WorkspaceName))
        {
            return CreateWorkspaceCommandResult.FailureResult(
                WorkspaceCreationFailureReason.MissingWorkspaceName,
                "Workspace name is required."
            );
        }

        string workspaceName = command.WorkspaceName.Trim();
        string workspacePath = _workspaceArtifactWriter.GetWorkspacePath(workspaceName);
        if (_workspaceArtifactWriter.WorkspaceExists(workspacePath))
        {
            return CreateWorkspaceCommandResult.FailureResult(
                WorkspaceCreationFailureReason.WorkspaceAlreadyExists,
                $"Workspace directory already exists: {workspacePath}"
            );
        }

        try
        {
            VersionInfo versionInfo = _versionProvider.GetVersionInfo();
            TemplateCatalog catalog = await _templatesService.GetCatalogAsync(
                versionInfo.SemanticVersion,
                cancellationToken
            );
            TemplateSelectionResult selectionResult = TemplateSelectionService.ResolveExplicitSelection(
                catalog,
                command.TemplateIdentifier
            );
            if (!selectionResult.IsSuccess)
            {
                TemplateSelectionFailure failure = selectionResult.Failure!;
                return CreateWorkspaceCommandResult.FailureResult(
                    MapFailureReason(failure.Reason),
                    failure.Message,
                    failure.ExitCode
                );
            }

            TemplateDescriptor selectedTemplate = selectionResult.SelectedTemplate!;
            await _workspaceArtifactWriter.CreateWorkspace(
                new WorkspaceArtifacts(
                    workspacePath,
                    workspaceName,
                    selectedTemplate.Identifier,
                    selectedTemplate.DisplayName,
                    selectedTemplate.Description
                ),
                cancellationToken
            );

            return CreateWorkspaceCommandResult.Success(
                new CreatedWorkspace(
                    workspaceName,
                    workspacePath,
                    selectedTemplate.Identifier,
                    selectedTemplate.DisplayName
                )
            );
        }
        catch (TemplateCatalogException exception)
        {
            _diagnosticLogger.Write(exception.ToString());
            return CreateWorkspaceCommandResult.FailureResult(
                WorkspaceCreationFailureReason.RuntimeFailure,
                exception.Message,
                ExitCodes.RuntimeFailure
            );
        }
    }

    private static WorkspaceCreationFailureReason MapFailureReason(TemplateSelectionFailureReason reason)
    {
        return reason switch
        {
            TemplateSelectionFailureReason.MissingTemplateIdentifier =>
                WorkspaceCreationFailureReason.MissingTemplateIdentifier,
            TemplateSelectionFailureReason.UnknownTemplateIdentifier =>
                WorkspaceCreationFailureReason.UnknownTemplateIdentifier,
            TemplateSelectionFailureReason.EmptyCatalog => WorkspaceCreationFailureReason.EmptyCatalog,
            _ => WorkspaceCreationFailureReason.RuntimeFailure,
        };
    }
}
