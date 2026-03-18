namespace NFramework.NFW.CLI.Startup;

internal static class CommandLineFeedback
{
    private const int MaxSuggestionCount = 3;
    private const int SuggestionThreshold = 3;

    public static bool TryWriteValidationError(ParsedArguments parsedArguments, out int exitCode)
    {
        if (!string.IsNullOrWhiteSpace(parsedArguments.UnknownCommand))
        {
            WriteUnknownCommand(parsedArguments.UnknownCommand);
            exitCode = Application.Features.Cli.ExitCodes.UsageError;
            return true;
        }

        if (!string.IsNullOrWhiteSpace(parsedArguments.UnknownOption))
        {
            WriteUnknownOption(parsedArguments.UnknownOption, parsedArguments.CommandName);
            exitCode = Application.Features.Cli.ExitCodes.UsageError;
            return true;
        }

        exitCode = Application.Features.Cli.ExitCodes.Success;
        return false;
    }

    private static void WriteUnknownCommand(string command)
    {
        Console.Error.WriteLine($"Unknown command: {command}");
        IReadOnlyList<string> suggestions = BuildSuggestions(command);
        Console.Error.WriteLine(
            suggestions.Count > 0
                ? $"Did you mean: {string.Join(", ", suggestions)}"
                : "Run nfw --help to see valid commands."
        );
    }

    private static void WriteUnknownOption(string option, string? commandName)
    {
        string validFlags =
            string.IsNullOrWhiteSpace(commandName) ? "--help, -h, --version, -v, --verbose"
            : string.Equals(commandName, "new", StringComparison.OrdinalIgnoreCase)
                ? "--help, -h, --template, --no-input"
            : "--help, -h";

        Console.Error.WriteLine($"Unknown flag: {option}");
        Console.Error.WriteLine($"Valid flags: {validFlags}");
    }

    private static IReadOnlyList<string> BuildSuggestions(string command)
    {
        return
        [
            .. ParsedArguments
                .KnownCommands.Select(candidate => new
                {
                    Candidate = candidate,
                    Score = CalculateDistance(command, candidate),
                })
                .Where(item =>
                    item.Candidate.StartsWith(command, StringComparison.OrdinalIgnoreCase)
                    || item.Score <= SuggestionThreshold
                )
                .OrderBy(item => item.Score)
                .ThenBy(item => item.Candidate, StringComparer.OrdinalIgnoreCase)
                .Select(item => item.Candidate)
                .Take(MaxSuggestionCount),
        ];
    }

    private static int CalculateDistance(string first, string second)
    {
        int firstLength = first.Length;
        int secondLength = second.Length;
        int[,] distance = new int[firstLength + 1, secondLength + 1];

        for (int row = 0; row <= firstLength; row += 1)
            distance[row, 0] = row;

        for (int column = 0; column <= secondLength; column += 1)
            distance[0, column] = column;

        for (int row = 1; row <= firstLength; row += 1)
        {
            for (int column = 1; column <= secondLength; column += 1)
            {
                int cost = char.ToLowerInvariant(first[row - 1]) == char.ToLowerInvariant(second[column - 1]) ? 0 : 1;
                distance[row, column] = Math.Min(
                    Math.Min(distance[row - 1, column] + 1, distance[row, column - 1] + 1),
                    distance[row - 1, column - 1] + cost
                );
            }
        }

        return distance[firstLength, secondLength];
    }
}
