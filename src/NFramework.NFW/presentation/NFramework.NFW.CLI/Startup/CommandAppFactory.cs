using Microsoft.Extensions.DependencyInjection;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.ListTemplates;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;
using NFramework.NFW.CLI.IoC;
using Spectre.Console.Cli;

namespace NFramework.NFW.CLI.Startup;

internal static class CommandAppFactory
{
    public static CommandApp Create(IServiceCollection services)
    {
        CommandApp commandApp = new(new TypeRegistrar(services));
        commandApp.Configure(configuration =>
        {
            _ = configuration.SetApplicationName("nfw");
            _ = configuration.ValidateExamples();

            _ = configuration
                .AddCommand<TemplatesCliCommand>("templates")
                .WithDescription("List available templates from the official template catalog.")
                .WithExample("templates")
                .WithExample("templates", "--help");

            _ = configuration
                .AddCommand<NewCliCommand>("new")
                .WithDescription("Create a workspace from a selected template.")
                .WithExample("new", "my-workspace", "--template", "blank")
                .WithExample("new", "my-workspace")
                .WithExample("new", "--no-input", "my-workspace", "--template", "blank");
        });

        return commandApp;
    }
}
