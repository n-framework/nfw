using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Models;
using CoreTemplateRenderer = NFramework.Core.Template.Abstractions.ITemplateRenderer;

namespace NFramework.NFW.CLI.Startup;

public sealed class ScribanTemplateRendererAdapter(
    CoreTemplateRenderer scribanRenderer
) : NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Abstractions.ITemplateRenderer
{
    private readonly CoreTemplateRenderer _scribanRenderer = scribanRenderer;

    public async Task<string> RenderAsync(
        string template,
        WorkspaceTemplateModel model,
        CancellationToken cancellationToken = default
    )
    {
        NFramework.Core.Template.Abstractions.ITemplateData data = new WorkspaceTemplateModelAdapter(model);
        return await _scribanRenderer.RenderAsync(template, data, cancellationToken);
    }
}
