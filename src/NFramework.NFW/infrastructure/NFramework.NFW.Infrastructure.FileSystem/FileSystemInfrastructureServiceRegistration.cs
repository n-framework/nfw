using Microsoft.Extensions.DependencyInjection;
using NFramework.NFW.Application.Features.Cli.Configuration.Abstractions;
using NFramework.NFW.Application.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Application.Features.TemplateManagement.Services.Abstractions;
using NFramework.NFW.Infrastructure.FileSystem.Features.Cli.Configuration;
using NFramework.NFW.Infrastructure.FileSystem.Features.ProjectManagement.Commands.New;
using NFramework.NFW.Infrastructure.FileSystem.Features.TemplateManagement.Services;

namespace NFramework.NFW.Infrastructure.FileSystem;

public static class FileSystemInfrastructureServiceRegistration
{
    public static IServiceCollection AddNfwFileSystemInfrastructure(this IServiceCollection services)
    {
        _ = services.AddSingleton<INfwConfigurationLoader, NfwConfigurationLoader>();
        _ = services.AddSingleton<ILocalTemplateCatalogSource, LocalTemplatesSubmoduleReader>();
        _ = services.AddSingleton<IWorkspaceArtifactWriter, FileSystemWorkspaceArtifactWriter>();

        return services;
    }
}
