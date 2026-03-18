using Spectre.Console.Cli;

namespace NFramework.NFW.CLI.IoC;

public sealed class TypeResolver(IServiceProvider serviceProvider) : ITypeResolver, IDisposable
{
    public IServiceProvider ServiceProvider { get; } = serviceProvider;

    public object? Resolve(Type? type)
    {
        return type is null ? null : ServiceProvider.GetService(type);
    }

    public void Dispose()
    {
        if (ServiceProvider is IDisposable disposable)
            disposable.Dispose();
    }
}
