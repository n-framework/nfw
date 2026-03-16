using Spectre.Console.Cli;

namespace NFramework.NFW.CLI.IoC;

public sealed class TypeResolver : ITypeResolver, IDisposable
{
    public TypeResolver(IServiceProvider serviceProvider)
    {
        ServiceProvider = serviceProvider;
    }

    public IServiceProvider ServiceProvider { get; }

    public object? Resolve(Type? type)
    {
        return type is null ? null : ServiceProvider.GetService(type);
    }

    public void Dispose()
    {
        if (ServiceProvider is IDisposable disposable)
        {
            disposable.Dispose();
        }
    }
}
