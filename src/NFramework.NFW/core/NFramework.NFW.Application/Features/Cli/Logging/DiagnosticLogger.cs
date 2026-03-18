namespace NFramework.NFW.Application.Features.Cli.Logging;

public enum LogLevel
{
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

public sealed class DiagnosticLogger
{
    private bool _isVerboseEnabled;

    public void EnableVerbose()
    {
        _isVerboseEnabled = true;
    }

    public void WriteDebug(string message)
    {
        Write(message, LogLevel.Debug);
    }

    public void WriteInfo(string message)
    {
        Write(message, LogLevel.Info);
    }

    public void WriteWarning(string message)
    {
        Write(message, LogLevel.Warning);
    }

    public void WriteError(string message)
    {
        Write(message, LogLevel.Error);
    }

    public void WriteCritical(string message)
    {
        Write(message, LogLevel.Critical);
    }

    public void Write(string message, LogLevel level = LogLevel.Debug)
    {
        if (level == LogLevel.Debug && !_isVerboseEnabled)
        {
            return;
        }

        string prefix = level switch
        {
            LogLevel.Debug => "[verbose]",
            LogLevel.Info => "[info]",
            LogLevel.Warning => "[warning]",
            LogLevel.Error => "[error]",
            LogLevel.Critical => "[critical]",
            _ => "[log]",
        };

        Console.Error.WriteLine($"{prefix} {message}");
    }
}
