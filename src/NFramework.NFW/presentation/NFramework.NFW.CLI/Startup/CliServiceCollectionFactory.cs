using Microsoft.Extensions.DependencyInjection;
using NFramework.Core.CLI.Abstractions;
using NFramework.Core.CLI.SpectreConsoleUI.DependencyInjection;
using NFramework.NFW.Application;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;
using NFramework.NFW.CLI.Features.ProjectManagement.Commands.New.Abstractions;
using NFramework.NFW.Infrastructure.FileSystem;
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

        // Core-cli Spectre.Console UI (includes ITerminalSession, IAnsiConsole)
        _ = services.AddCoreCliSpectreConsoleUi();

        // NFW-specific terminal adapter (wraps core-cli's ITerminalSession)
        _ = services.AddSingleton<INfwTerminalSession, NfwTerminalSessionAdapter>();

        // Scriban template rendering
        _ = services.AddScribanTemplateRendering();

        // Register the orchestrator and application
        _ = services.AddSingleton<NfwCliApplicationOrchestrator>();
        _ = services.AddSingleton<ICliApplication, NfwCliApplication>();

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
