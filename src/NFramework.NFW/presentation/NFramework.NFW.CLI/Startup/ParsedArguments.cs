namespace NFramework.NFW.CLI.Startup;

internal sealed record ParsedArguments(
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
    private static readonly string[] NewCommandFlags = ["--help", "-h", "--template", "--no-input"];

    internal static IReadOnlyList<string> KnownCommands => ["templates", "new"];

    public static ParsedArguments Parse(IEnumerable<string> rawArguments)
    {
        List<string> arguments = [];
        bool verboseEnabled = false;
        string? commandName = null;
        string? unknownCommand = null;
        string? unknownOption = null;

        foreach (string argument in rawArguments)
        {
            if (string.Equals(argument, "--verbose", StringComparison.OrdinalIgnoreCase))
            {
                verboseEnabled = true;
                continue;
            }

            arguments.Add(argument);

            if (commandName is null && !argument.StartsWith('-', StringComparison.Ordinal))
            {
                commandName = argument;
                if (!KnownCommands.Contains(commandName, StringComparer.OrdinalIgnoreCase))
                {
                    unknownCommand = commandName;
                }
            }

            if (
                unknownOption is null
                && argument.StartsWith('-', StringComparison.Ordinal)
                && !IsKnownOption(argument, commandName)
            )
            {
                unknownOption = argument;
            }
        }

        bool showHelp = arguments.Any(argument =>
            string.Equals(argument, "--help", StringComparison.OrdinalIgnoreCase)
            || string.Equals(argument, "-h", StringComparison.OrdinalIgnoreCase)
        );
        bool showVersion = arguments.Any(argument =>
            string.Equals(argument, "--version", StringComparison.OrdinalIgnoreCase)
            || string.Equals(argument, "-v", StringComparison.OrdinalIgnoreCase)
        );

        if (showHelp)
            unknownCommand = null;

        return new ParsedArguments(
            verboseEnabled,
            showHelp,
            showVersion,
            commandName,
            unknownCommand,
            unknownOption,
            arguments
        );
    }

    public bool ShouldShowVersionOnly()
    {
        return ShowVersion && !ShowHelp && CommandName is null;
    }

    public string[] BuildForwardedArguments()
    {
        if (ForwardedArguments.Count == 0)
            return ["--help"];

        if (ShowHelp && ShowVersion)
        {
            return
            [
                .. ForwardedArguments.Where(argument =>
                    !string.Equals(argument, "--version", StringComparison.OrdinalIgnoreCase)
                    && !string.Equals(argument, "-v", StringComparison.OrdinalIgnoreCase)
                ),
            ];
        }

        return [.. ForwardedArguments];
    }

    private static bool IsKnownOption(string option, string? commandName)
    {
        if (GlobalFlags.Contains(option, StringComparer.OrdinalIgnoreCase))
            return true;

        if (string.Equals(commandName, "templates", StringComparison.OrdinalIgnoreCase))
            return TemplatesCommandFlags.Contains(option, StringComparer.OrdinalIgnoreCase);

        if (string.Equals(commandName, "new", StringComparison.OrdinalIgnoreCase))
            return NewCommandFlags.Contains(option, StringComparer.OrdinalIgnoreCase);

        return false;
    }
}
