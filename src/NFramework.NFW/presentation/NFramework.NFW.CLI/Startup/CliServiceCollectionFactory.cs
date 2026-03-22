using Microsoft.Extensions.DependencyInjection;
using NFramework.Core.CLI.Abstractions;
using NFramework.Core.CLI.SpectreConsoleUI.DependencyInjection;
using NFramework.NFW.Application;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Infrastructure.FileSystem.DependencyInjection;
using NFramework.NFW.Infrastructure.GitHub;

namespace NFramework.NFW.CLI.Startup;

internal static class CliServiceCollectionFactory
{
    public static CliServices Create(ParsedArguments parsedArguments)
    {
        ServiceCollection services = new();

        // Framework layers
        _ = services.AddNfwApplication();
        _ = services.AddNfwFileSystemInfrastructure();
        _ = services.AddNfwGitHubInfrastructure();

        // NFramework.Core.CLI Spectre.Console UI (includes ITerminalSession, IAnsiConsole)
        _ = services.AddCoreCliSpectreConsoleUi();

        // Scriban template rendering
        _ = services.AddScribanTemplateRendering();

        DiagnosticLogger diagnosticLogger = new();
        if (parsedArguments.VerboseEnabled)
        {
            diagnosticLogger.EnableVerbose();
        }

        _ = services.AddSingleton(diagnosticLogger);

        // Source-generated: registers Spectre.Console commands and creates the command app
        ICliApplication generatedCliApplication = SpectreConsoleCommandRegistration.CreateCommandApp(services);

        // Register the orchestrator and application
        _ = services.AddSingleton<NfwCliApplicationOrchestrator>();
        _ = services.AddSingleton<ICliApplication>(generatedCliApplication);

        CliServices cliServices = new(services, diagnosticLogger, generatedCliApplication);
        _ = services.AddSingleton(cliServices);

        return cliServices;
    }
}

internal sealed record CliServices(
    IServiceCollection Services,
    DiagnosticLogger DiagnosticLogger,
    ICliApplication GeneratedCliApplication
);
