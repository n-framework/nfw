using Microsoft.Extensions.DependencyInjection;
using NFramework.NFW.Application;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Infrastructure.FileSystem;
using NFramework.NFW.Infrastructure.GitHub;
using Spectre.Console;

namespace NFramework.NFW.CLI.Startup;

internal static class CliServiceCollectionFactory
{
    public static CliServices Create(ParsedArguments parsedArguments)
    {
        ServiceCollection services = new();
        _ = services.AddNfwApplication();
        _ = services.AddNfwFileSystemInfrastructure();
        _ = services.AddNfwGitHubInfrastructure();
        _ = services.AddSingleton<IAnsiConsole>(_ => AnsiConsole.Console);
        _ = services.AddSingleton<InteractiveTemplatePrompt>();
        _ = services.AddSingleton<ITerminalSession, CliTerminalSession>();

        DiagnosticLogger diagnosticLogger = new();
        if (parsedArguments.VerboseEnabled)
        {
            diagnosticLogger.EnableVerbose();
        }

        _ = services.AddSingleton(diagnosticLogger);
        return new CliServices(services, diagnosticLogger);
    }
}

internal sealed record CliServices(IServiceCollection Services, DiagnosticLogger DiagnosticLogger);
