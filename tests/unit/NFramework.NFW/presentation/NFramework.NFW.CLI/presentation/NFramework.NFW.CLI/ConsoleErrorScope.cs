namespace NFramework.NFW.CLI.Tests.presentation.NFramework.NFW.CLI;

public sealed class ConsoleErrorScope : IDisposable
{
    private readonly TextWriter _originalError = Console.Error;
    private readonly StringWriter _capture = new();

    public ConsoleErrorScope()
    {
        Console.SetError(_capture);
    }

    public string CapturedText => _capture.ToString();

    public void Dispose()
    {
        Console.SetError(_originalError);
        _capture.Dispose();
    }
}
