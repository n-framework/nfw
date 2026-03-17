using Microsoft.Extensions.DependencyInjection;
using NFramework.NFW.Application.Features.Cli.Configuration;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.Templates;
using NFramework.NFW.Application.Features.Versioning;

namespace NFramework.NFW.Application;

public static class ApplicationServiceRegistration
{
    public static IServiceCollection AddNfwApplication(this IServiceCollection services)
    {
        services.AddSingleton<INfwConfigurationLoader, NfwConfigurationLoader>();
        services.AddSingleton<RequiredConfigurationValidator>(_ => new RequiredConfigurationValidator());
        services.AddSingleton<DiagnosticLogger>();
        services.AddSingleton<IVersionProvider, VersionProvider>();

        services.AddSingleton<TemplateCatalogParser>();
        services.AddSingleton<LocalTemplatesSubmoduleReader>();
        services.AddHttpClient<GitHubTemplatesReleaseClient>();
        services.AddSingleton<TemplateCatalogSourceResolver>();
        services.AddSingleton<TemplatesService>();

        return services;
    }
}
