using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Models;

namespace NFramework.NFW.CLI.Startup;

public sealed class FileSystemWorkspaceTemplateProvider : IWorkspaceTemplateProvider
{
    public Task<IReadOnlyList<WorkspaceTemplateFile>> GetTemplateFilesAsync(
        string templateIdentifier,
        CancellationToken cancellationToken = default
    )
    {
        IReadOnlyList<WorkspaceTemplateFile> files = [];
        return Task.FromResult(files);
    }
}
