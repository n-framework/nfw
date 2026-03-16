namespace NFramework.NFW.Application.Features.Cli.Logging;

public sealed class DiagnosticLogger
{
    private bool _isVerboseEnabled;

    public void EnableVerbose()
    {
        _isVerboseEnabled = true;
    }

    public void Write(string message)
    {
        if (!_isVerboseEnabled)
        {
            return;
        }

        Console.Error.WriteLine($"[verbose] {message}");
    }
}
