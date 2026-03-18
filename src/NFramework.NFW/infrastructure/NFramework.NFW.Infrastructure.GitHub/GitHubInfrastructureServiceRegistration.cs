using Microsoft.Extensions.DependencyInjection;
using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;
using NFramework.NFW.Infrastructure.GitHub.Features.TemplateManagement.Services;

namespace NFramework.NFW.Infrastructure.GitHub;

public static class GitHubInfrastructureServiceRegistration
{
    public static IServiceCollection AddNfwGitHubInfrastructure(this IServiceCollection services)
    {
        _ = services.AddHttpClient<IRemoteTemplateCatalogSource, GitHubTemplatesReleaseClient>();

        return services;
    }
}
