using Microsoft.Extensions.DependencyInjection;
using NFramework.Core.Template.Abstractions;
using NFramework.Core.Template.Scriban;
using NFramework.NFW.Application.Features.TemplateManagement.TemplateRendering.Abstractions;

namespace NFramework.NFW.CLI.Startup;

internal static class TemplateRenderingServiceCollectionExtensions
{
    public static IServiceCollection AddScribanTemplateRendering(this IServiceCollection services)
    {
        _ = services.AddSingleton<ScribanTemplateRenderer>();
        _ = services.AddSingleton<ITemplateRenderer>(sp => sp.GetRequiredService<ScribanTemplateRenderer>());
        _ = services.AddSingleton<IWorkspaceTemplateProvider, FileSystemWorkspaceTemplateProvider>();

        return services;
    }
}
