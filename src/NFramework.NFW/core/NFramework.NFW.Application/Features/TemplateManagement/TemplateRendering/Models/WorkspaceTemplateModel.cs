using NFramework.Core.Template.Abstractions;

namespace NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Models;

public sealed record WorkspaceTemplateModel(
    string WorkspaceName,
    string TemplateIdentifier,
    string TemplateDisplayName,
    string TemplateDescription,
    DateTime CreatedAt
) : ITemplateData;
