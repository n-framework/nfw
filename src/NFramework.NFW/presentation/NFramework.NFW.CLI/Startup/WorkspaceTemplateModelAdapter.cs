using NFramework.Core.Template.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Models;

namespace NFramework.NFW.CLI.Startup;

public sealed class WorkspaceTemplateModelAdapter(WorkspaceTemplateModel model) : ITemplateData
{
    public string WorkspaceName => model.WorkspaceName;

    public string TemplateIdentifier => model.TemplateIdentifier;

    public string TemplateDisplayName => model.TemplateDisplayName;

    public string TemplateDescription => model.TemplateDescription;

    public DateTime CreatedAt => model.CreatedAt;
}
