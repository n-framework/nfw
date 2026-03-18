using Microsoft.Extensions.DependencyInjection;
using Spectre.Console.Cli;

namespace NFramework.NFW.CLI.IoC;

public sealed class TypeRegistrar(IServiceCollection services) : ITypeRegistrar
{
    private readonly IServiceCollection _services = services;

    public ITypeResolver Build()
    {
        return new TypeResolver(_services.BuildServiceProvider());
    }

    public void Register(Type service, Type implementation)
    {
        _ = _services.AddSingleton(service, implementation);
    }

    public void RegisterInstance(Type service, object implementation)
    {
        _ = _services.AddSingleton(service, implementation);
    }

    public void RegisterLazy(Type service, Func<object> factory)
    {
        _ = _services.AddSingleton(service, _ => factory());
    }
}
