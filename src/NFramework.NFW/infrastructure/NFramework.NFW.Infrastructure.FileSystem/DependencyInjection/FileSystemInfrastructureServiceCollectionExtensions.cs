using Microsoft.Extensions.DependencyInjection;

namespace NFramework.NFW.Infrastructure.FileSystem.DependencyInjection;

/// <summary>
/// Dependency injection extensions for FileSystem infrastructure services.
/// </summary>
public static class FileSystemInfrastructureServiceCollectionExtensions
{
    /// <summary>
    /// Adds FileSystem infrastructure services to the service collection.
    /// </summary>
    public static IServiceCollection AddNfwFileSystemInfrastructure(this IServiceCollection services)
    {
        return FileSystemInfrastructureServiceRegistration.AddNfwFileSystemInfrastructure(services);
    }
}
