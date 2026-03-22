using System.Collections.Generic;
using Microsoft.Extensions.DependencyInjection;
using NFramework.Core.CLI.Abstractions;
using NFramework.Core.CLI.SpectreConsoleUI.IoC;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.ListTemplates;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;
using Spectre.Console;
using Spectre.Console.Cli;

namespace NFramework.NFW.CLI.Startup;

internal static class SpectreConsoleCommandRegistration
{
    public static ICliApplication CreateCommandApp(IServiceCollection services)
    {
        _ = services.AddSingleton<NewCliCommand>();
        _ = services.AddSingleton<TemplatesCliCommand>();

        CommandApp commandApp = new(new TypeRegistrar(services));
        commandApp.Configure(configuration =>
        {
            _ = configuration.SetApplicationName("nfw");
            _ = configuration.ValidateExamples();

            _ = configuration
                .AddCommand<NewCommandAdapter>("new")
                .WithDescription("Create a new workspace from a template");

            _ = configuration
                .AddCommand<TemplatesCommandAdapter>("templates")
                .WithDescription("List available templates");
        });

        return new SpectreConsoleCliApplication(commandApp);
    }
}

internal sealed class SpectreConsoleCliApplication(CommandApp commandApp) : ICliApplication
{
    private readonly CommandApp _commandApp = commandApp;

    public int Run(string[] args)
    {
        ArgumentNullException.ThrowIfNull(args);
        return _commandApp.Run(args);
    }
}

internal sealed class NewCommandAdapter(NewCliCommand command) : AsyncCommand<NewCommandSettings>
{
    private readonly NewCliCommand _command = command;

    public override Task<int> ExecuteAsync(
        CommandContext context,
        NewCommandSettings settings,
        CancellationToken cancellationToken
    )
    {
        CliCommandContext cliContext = new("new", new List<string>(context.Arguments));
        NewCliCommandSettings cliSettings = new()
        {
            WorkspaceName = settings.WorkspaceName,
            TemplateIdentifier = settings.TemplateIdentifier,
            NoInput = settings.NoInput,
        };
        return _command.ExecuteAsync(cliContext, cliSettings, cancellationToken);
    }
}

internal sealed class NewCommandSettings : CommandSettings
{
    [CommandArgument(0, "[workspace-name]")]
    public string? WorkspaceName { get; set; }

    [CommandOption("--template <IDENTIFIER>")]
    public string? TemplateIdentifier { get; set; }

    [CommandOption("--no-input")]
    public bool NoInput { get; set; }
}

internal sealed class TemplatesCommandAdapter(TemplatesCliCommand command) : AsyncCommand<TemplatesCommandSettings>
{
    private readonly TemplatesCliCommand _command = command;

    public override Task<int> ExecuteAsync(
        CommandContext context,
        TemplatesCommandSettings settings,
        CancellationToken cancellationToken
    )
    {
        CliCommandContext cliContext = new("templates", new List<string>(context.Arguments));
        TemplatesCliCommandSettings cliSettings = new();
        return _command.ExecuteAsync(cliContext, cliSettings, cancellationToken);
    }
}

internal sealed class TemplatesCommandSettings : CommandSettings;
