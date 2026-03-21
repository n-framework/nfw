using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Models;

namespace NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Abstractions;

public interface IWorkspaceTemplateProvider
{
    Task<IReadOnlyList<WorkspaceTemplateFile>> GetTemplateFilesAsync(
        string templateIdentifier,
        CancellationToken cancellationToken = default
    );
}
