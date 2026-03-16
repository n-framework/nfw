using System.Text;
using Microsoft.Extensions.DependencyInjection;
using NFramework.NFW.Application;
using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Application.Features.Cli.Configuration;
using NFramework.NFW.Application.Features.Cli.Logging;
using NFramework.NFW.Application.Features.Versioning;
using NFramework.NFW.CLI.Features.Templates;
using NFramework.NFW.CLI.IoC;
using Spectre.Console.Cli;

namespace NFramework.NFW.CLI;

public static class Program
{
    private const int MaxConsoleWidth = 80;
    private const int MaxSuggestionCount = 3;
    private const int SuggestionThreshold = 3;

    private static readonly string[] KnownCommands = ["templates"];

    public static int Main(string[] rawArgs)
    {
        Console.OutputEncoding = Encoding.UTF8;
        Console.InputEncoding = Encoding.UTF8;
        EnsureReadableConsoleWidth();

        var parsedArguments = ParsedArguments.Parse(rawArgs);
        if (!string.IsNullOrWhiteSpace(parsedArguments.UnknownCommand))
        {
            WriteUnknownCommand(parsedArguments.UnknownCommand!);
            return ExitCodes.UsageError;
        }

        if (!string.IsNullOrWhiteSpace(parsedArguments.UnknownOption))
        {
            WriteUnknownOption(parsedArguments.UnknownOption!, parsedArguments.CommandName);
            return ExitCodes.UsageError;
        }

        var serviceCollection = new ServiceCollection();
        serviceCollection.AddNfwApplication();

        var diagnosticLogger = new DiagnosticLogger();
        if (parsedArguments.VerboseEnabled)
        {
            diagnosticLogger.EnableVerbose();
        }

        serviceCollection.AddSingleton(diagnosticLogger);

        using var serviceProvider = serviceCollection.BuildServiceProvider();
        var configurationLoader = serviceProvider.GetRequiredService<INfwConfigurationLoader>();
        var requiredConfigurationValidator = serviceProvider.GetRequiredService<RequiredConfigurationValidator>();
        var versionProvider = serviceProvider.GetRequiredService<IVersionProvider>();

        diagnosticLogger.Write("Loading configuration from nfw.yaml and environment.");
        var configurationResult = configurationLoader.Load();
        if (configurationResult.IsFailure)
        {
            Console.Error.WriteLine($"Configuration error: {configurationResult.Error}");
            return ExitCodes.RuntimeFailure;
        }

        var configuration = configurationResult.Value!;
        var missingConfigurationKeys = requiredConfigurationValidator.Validate(configuration);
        if (missingConfigurationKeys.Count > 0)
        {
            Console.Error.WriteLine(
                $"Missing required configuration values: {string.Join(", ", missingConfigurationKeys)}"
            );
            return ExitCodes.RuntimeFailure;
        }

        if (parsedArguments.ShowVersion && !parsedArguments.ShowHelp && parsedArguments.CommandName is null)
        {
            Console.WriteLine(versionProvider.GetVersionInfo().ToString());
            return ExitCodes.Success;
        }

        var forwardedArguments = BuildForwardedArguments(parsedArguments);
        var typeRegistrar = new TypeRegistrar(serviceCollection);
        var commandApp = BuildCommandApp(typeRegistrar);

        var interruptedBySignal = false;
        ConsoleCancelEventHandler onCancel = (_, eventArgs) =>
        {
            interruptedBySignal = true;
            eventArgs.Cancel = true;
        };
        Console.CancelKeyPress += onCancel;

        try
        {
            diagnosticLogger.Write($"Executing CLI with args: {string.Join(" ", forwardedArguments)}");
            var exitCode = commandApp.Run(forwardedArguments);

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

    private static CommandApp BuildCommandApp(ITypeRegistrar typeRegistrar)
    {
        var commandApp = new CommandApp(typeRegistrar);
        commandApp.Configure(configuration =>
        {
            configuration.SetApplicationName("nfw");
            configuration.ValidateExamples();

            configuration
                .AddCommand<TemplatesCliCommand>("templates")
                .WithDescription("List available templates from the official template catalog.")
                .WithExample("templates")
                .WithExample("templates", "--help");
        });

        return commandApp;
    }

    private static string[] BuildForwardedArguments(ParsedArguments parsedArguments)
    {
        if (parsedArguments.ForwardedArguments.Count == 0)
        {
            return ["--help"];
        }

        if (parsedArguments.ShowHelp && parsedArguments.ShowVersion)
        {
            return parsedArguments
                .ForwardedArguments.Where(argument =>
                    !string.Equals(argument, "--version", StringComparison.OrdinalIgnoreCase)
                    && !string.Equals(argument, "-v", StringComparison.OrdinalIgnoreCase)
                )
                .ToArray();
        }

        return parsedArguments.ForwardedArguments.ToArray();
    }

    private static int NormalizeExitCode(int exitCode)
    {
        if (exitCode == ExitCodes.Success)
        {
            return ExitCodes.Success;
        }

        if (exitCode < 0)
        {
            return ExitCodes.UsageError;
        }

        return exitCode;
    }

    private static void EnsureReadableConsoleWidth()
    {
        if (Console.IsOutputRedirected)
        {
            return;
        }

        if (!OperatingSystem.IsWindows())
        {
            return;
        }

        try
        {
            if (Console.WindowWidth > MaxConsoleWidth)
            {
                Console.WindowWidth = MaxConsoleWidth;
            }
        }
        catch (PlatformNotSupportedException)
        {
            // Terminal does not support width operations; continue.
        }
        catch (IOException)
        {
            // Console output may be redirected; continue.
        }
    }

    private static void WriteUnknownCommand(string command)
    {
        Console.Error.WriteLine($"Unknown command: {command}");
        var suggestions = BuildSuggestions(command);
        if (suggestions.Count > 0)
        {
            Console.Error.WriteLine($"Did you mean: {string.Join(", ", suggestions)}");
        }
        else
        {
            Console.Error.WriteLine("Run nfw --help to see valid commands.");
        }
    }

    private static void WriteUnknownOption(string option, string? commandName)
    {
        var validFlags = string.IsNullOrWhiteSpace(commandName) ? "--help, -h, --version, -v, --verbose" : "--help, -h";

        Console.Error.WriteLine($"Unknown flag: {option}");
        Console.Error.WriteLine($"Valid flags: {validFlags}");
    }

    private static IReadOnlyList<string> BuildSuggestions(string command)
    {
        return KnownCommands
            .Select(candidate => new { Candidate = candidate, Score = CalculateDistance(command, candidate) })
            .Where(item =>
                item.Candidate.StartsWith(command, StringComparison.OrdinalIgnoreCase)
                || item.Score <= SuggestionThreshold
            )
            .OrderBy(item => item.Score)
            .ThenBy(item => item.Candidate, StringComparer.OrdinalIgnoreCase)
            .Select(item => item.Candidate)
            .Take(MaxSuggestionCount)
            .ToArray();
    }

    private static int CalculateDistance(string first, string second)
    {
        var firstLength = first.Length;
        var secondLength = second.Length;
        var distance = new int[firstLength + 1, secondLength + 1];

        for (var row = 0; row <= firstLength; row += 1)
        {
            distance[row, 0] = row;
        }

        for (var column = 0; column <= secondLength; column += 1)
        {
            distance[0, column] = column;
        }

        for (var row = 1; row <= firstLength; row += 1)
        {
            for (var column = 1; column <= secondLength; column += 1)
            {
                var substitutionCost = first[row - 1] == second[column - 1] ? 0 : 1;
                distance[row, column] = new[]
                {
                    distance[row - 1, column] + 1,
                    distance[row, column - 1] + 1,
                    distance[row - 1, column - 1] + substitutionCost,
                }.Min();
            }
        }

        return distance[firstLength, secondLength];
    }

    private sealed record ParsedArguments(
        bool VerboseEnabled,
        bool ShowHelp,
        bool ShowVersion,
        string? CommandName,
        string? UnknownCommand,
        string? UnknownOption,
        IReadOnlyList<string> ForwardedArguments
    )
    {
        private static readonly string[] GlobalFlags = ["--help", "-h", "--version", "-v", "--verbose"];
        private static readonly string[] TemplatesCommandFlags = ["--help", "-h"];

        public static ParsedArguments Parse(IEnumerable<string> rawArguments)
        {
            var args = new List<string>();
            var verboseEnabled = false;
            string? commandName = null;
            string? unknownCommand = null;
            string? unknownOption = null;

            foreach (var argument in rawArguments)
            {
                if (string.Equals(argument, "--verbose", StringComparison.OrdinalIgnoreCase))
                {
                    verboseEnabled = true;
                    continue;
                }

                args.Add(argument);

                if (commandName is null && !argument.StartsWith("-", StringComparison.Ordinal))
                {
                    commandName = argument;
                    if (!KnownCommands.Contains(commandName, StringComparer.OrdinalIgnoreCase))
                    {
                        unknownCommand = commandName;
                    }
                }

                if (unknownOption is null && argument.StartsWith("-", StringComparison.Ordinal))
                {
                    if (!IsKnownOption(argument, commandName))
                    {
                        unknownOption = argument;
                    }
                }
            }

            var showHelp = args.Any(argument =>
                string.Equals(argument, "--help", StringComparison.OrdinalIgnoreCase)
                || string.Equals(argument, "-h", StringComparison.OrdinalIgnoreCase)
            );
            var showVersion = args.Any(argument =>
                string.Equals(argument, "--version", StringComparison.OrdinalIgnoreCase)
                || string.Equals(argument, "-v", StringComparison.OrdinalIgnoreCase)
            );

            if (showHelp)
            {
                unknownCommand = null;
            }

            return new ParsedArguments(
                verboseEnabled,
                showHelp,
                showVersion,
                commandName,
                unknownCommand,
                unknownOption,
                args
            );
        }

        private static bool IsKnownOption(string option, string? commandName)
        {
            if (GlobalFlags.Contains(option, StringComparer.OrdinalIgnoreCase))
            {
                return true;
            }

            if (string.Equals(commandName, "templates", StringComparison.OrdinalIgnoreCase))
            {
                return TemplatesCommandFlags.Contains(option, StringComparer.OrdinalIgnoreCase);
            }

            return false;
        }
    }
}
