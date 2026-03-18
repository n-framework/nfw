using Microsoft.Extensions.DependencyInjection;
using NFramework.NFW.Application.Features.Cli.Configuration;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Application.Features.TemplateManagement.Queries.ListTemplates;
using NFramework.NFW.Application.Features.TemplateManagement.Services;
using NFramework.NFW.Application.Features.Versioning;
using NFramework.NFW.Application.Features.Versioning.Abstractions;

namespace NFramework.NFW.Application;

public static class ApplicationServiceRegistration
{
    public static IServiceCollection AddNfwApplication(this IServiceCollection services)
    {
        _ = services.AddSingleton<RequiredConfigurationValidator>(_ => new RequiredConfigurationValidator());
        _ = services.AddSingleton<DiagnosticLogger>();
        _ = services.AddSingleton<IVersionProvider, VersionProvider>();

        _ = services.AddSingleton<TemplateCatalogParser>();
        _ = services.AddSingleton<TemplateCatalogSourceResolver>();
        _ = services.AddSingleton<TemplatesService>();
        _ = services.AddSingleton<ListTemplatesQueryHandler>();
        _ = services.AddSingleton<CreateWorkspaceCommandHandler>();

        return services;
    }
}
