using NFramework.Core.Template.Abstractions;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Models;

namespace NFramework.NFW.Infrastructure.FileSystem.Features.ProjectManagement.Commands.New;

public sealed class FileSystemWorkspaceArtifactWriter(
    ITemplateRenderer templateRenderer,
    IWorkspaceTemplateProvider workspaceTemplateProvider
) : IWorkspaceArtifactWriter
{
    private readonly ITemplateRenderer _templateRenderer = templateRenderer;
    private readonly IWorkspaceTemplateProvider _workspaceTemplateProvider = workspaceTemplateProvider;

    public string GetWorkspacePath(string workspaceName)
    {
        return Path.GetFullPath(workspaceName);
    }

    public bool WorkspaceExists(string workspacePath)
    {
        return Directory.Exists(workspacePath);
    }

    public async Task CreateWorkspace(WorkspaceArtifacts artifacts, CancellationToken cancellationToken = default)
    {
        _ = Directory.CreateDirectory(artifacts.WorkspacePath);

        WorkspaceTemplateModel model = new(
            artifacts.WorkspaceName,
            artifacts.TemplateIdentifier,
            artifacts.TemplateDisplayName,
            artifacts.TemplateDescription,
            DateTime.UtcNow
        );

        IReadOnlyList<WorkspaceTemplateFile> templateFiles = await _workspaceTemplateProvider.GetTemplateFilesAsync(
            artifacts.TemplateIdentifier,
            cancellationToken
        );

        if (templateFiles.Count == 0)
        {
            CreateBasicWorkspaceFiles(artifacts);
            return;
        }

        foreach (WorkspaceTemplateFile templateFile in templateFiles)
        {
            string renderedPath = await _templateRenderer.RenderAsync(
                templateFile.RelativePath,
                model,
                cancellationToken
            );

            string outputPath = Path.Combine(artifacts.WorkspacePath, renderedPath);
            string? outputDirectory = Path.GetDirectoryName(outputPath);
            if (!string.IsNullOrEmpty(outputDirectory))
            {
                _ = Directory.CreateDirectory(outputDirectory);
            }

            string renderedContent = await _templateRenderer.RenderAsync(
                templateFile.Content,
                model,
                cancellationToken
            );

            await File.WriteAllTextAsync(outputPath, renderedContent, cancellationToken);
        }
    }

    private void CreateBasicWorkspaceFiles(WorkspaceArtifacts artifacts)
    {
        string configurationContent = $"""
            workspace:
              name: {artifacts.WorkspaceName}
              template: {artifacts.TemplateIdentifier}
            """;
        File.WriteAllText(Path.Combine(artifacts.WorkspacePath, "nfw.yaml"), configurationContent);

        string readmeContent = $"""
            # {artifacts.WorkspaceName}

            Created by `nfw new` using template `{artifacts.TemplateIdentifier}`.

            Template: {artifacts.TemplateDisplayName}
            Description: {artifacts.TemplateDescription}
            """;
        File.WriteAllText(Path.Combine(artifacts.WorkspacePath, "README.md"), readmeContent);
    }
}
