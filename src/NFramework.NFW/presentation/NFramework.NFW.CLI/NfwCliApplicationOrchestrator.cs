using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.DependencyInjection.Extensions;
using NFramework.Core.CLI.Abstractions;
using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.CLI.Startup;

namespace NFramework.NFW.CLI;

/// <summary>
/// Orchestrates CLI application startup, configuration, and execution.
/// Separated from the generated application class to keep source generation clean.
/// </summary>
public sealed class NfwCliApplicationOrchestrator(IServiceProvider serviceProvider, DiagnosticLogger diagnosticLogger)
{
    private readonly IServiceProvider _serviceProvider = serviceProvider;
    private readonly DiagnosticLogger _diagnosticLogger = diagnosticLogger;

    public int Run(string[] args)
    {
        Startup.ConsoleConfigurator.Configure();

        Startup.ParsedArguments parsedArguments = Startup.ParsedArguments.Parse(args);
        if (Startup.CommandLineFeedback.TryWriteValidationError(parsedArguments, out int validationExitCode))
        {
            return validationExitCode;
        }

        // Create CliServices for bootstrapping
        ServiceCollection bootstrapServices = new();
        foreach (ServiceDescriptor service in _serviceProvider.GetServices<ServiceDescriptor>())
        {
            _ = bootstrapServices.Add(service);
        }

        Startup.CliServices cliServices = new(bootstrapServices, _diagnosticLogger);

        CliBootstrapResult bootstrapResult = Startup.CliBootstrapper.Bootstrap(cliServices);
        if (bootstrapResult.IsFailure)
        {
            Console.Error.WriteLine(bootstrapResult.ErrorMessage);
            return bootstrapResult.ExitCode;
        }

        if (parsedArguments.ShouldShowVersionOnly())
        {
            Console.WriteLine(bootstrapResult.VersionText);
            return ExitCodes.Success;
        }

        // Return the forwarded arguments for the source-generated command runner
        return runCore(parsedArguments.BuildForwardedArguments());
    }

    private int runCore(string[] forwardedArguments)
    {
        bool interruptedBySignal = false;
        void onCancel(object? _, ConsoleCancelEventArgs eventArgs)
        {
            interruptedBySignal = true;
            eventArgs.Cancel = true;
        }

        Console.CancelKeyPress += onCancel;

        try
        {
            _diagnosticLogger.Write($"Executing CLI with args: {string.Join(" ", forwardedArguments)}");

            // The source generator will handle command execution from here
            // Call the generated Run method
            ICliApplication cliApplication = _serviceProvider.GetRequiredService<ICliApplication>();
            int exitCode = cliApplication.Run(forwardedArguments);

            return interruptedBySignal ? ExitCodes.Interrupted : NormalizeExitCode(exitCode);
        }
        catch (Exception exception)
        {
            Console.Error.WriteLine(exception.Message);
            _diagnosticLogger.Write(exception.ToString());
            return ExitCodes.RuntimeFailure;
        }
        finally
        {
            Console.CancelKeyPress -= onCancel;
        }
    }

    private static int NormalizeExitCode(int exitCode)
    {
        return exitCode == ExitCodes.Success ? ExitCodes.Success
            : exitCode < 0 ? ExitCodes.UsageError
            : exitCode;
    }
}
