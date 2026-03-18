using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.CLI.IoC;
using Spectre.Console.Cli;

namespace NFramework.NFW.CLI.Startup;

internal static class NfwCliApplication
{
    public static int Run(string[] args)
    {
        ConsoleConfigurator.Configure();

        ParsedArguments parsedArguments = ParsedArguments.Parse(args);
        if (CommandLineFeedback.TryWriteValidationError(parsedArguments, out int validationExitCode))
        {
            return validationExitCode;
        }

        CliServices cliServices = CliServiceCollectionFactory.Create(parsedArguments);
        CliBootstrapResult bootstrapResult = CliBootstrapper.Bootstrap(cliServices);
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

        CommandApp commandApp = CommandAppFactory.Create(cliServices.Services);
        return RunCommandApp(commandApp, parsedArguments.BuildForwardedArguments(), cliServices.DiagnosticLogger);
    }

    private static int RunCommandApp(
        CommandApp commandApp,
        string[] forwardedArguments,
        Application.Features.Cli.Logging.DiagnosticLogger diagnosticLogger
    )
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
            diagnosticLogger.Write($"Executing CLI with args: {string.Join(" ", forwardedArguments)}");
            int exitCode = commandApp.Run(forwardedArguments);

            if (interruptedBySignal)
            {
                return ExitCodes.Interrupted;
            }

            return NormalizeExitCode(exitCode);
        }
        catch (Exception exception)
        {
            Console.Error.WriteLine(exception.Message);
            diagnosticLogger.Write(exception.ToString());
            return ExitCodes.RuntimeFailure;
        }
        finally
        {
            Console.CancelKeyPress -= onCancel;
        }
    }

    private static int NormalizeExitCode(int exitCode)
    {
        if (exitCode == ExitCodes.Success)
        {
            return ExitCodes.Success;
        }

        return exitCode < 0 ? ExitCodes.UsageError : exitCode;
    }
}
