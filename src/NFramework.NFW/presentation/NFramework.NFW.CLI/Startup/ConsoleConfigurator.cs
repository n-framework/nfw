using System.Text;

namespace NFramework.NFW.CLI.Startup;

internal static class ConsoleConfigurator
{
    private const int MaxConsoleWidth = 80;

    public static void Configure()
    {
        Console.OutputEncoding = Encoding.UTF8;
        Console.InputEncoding = Encoding.UTF8;
        EnsureReadableConsoleWidth();
    }

    private static void EnsureReadableConsoleWidth()
    {
        if (Console.IsOutputRedirected || !OperatingSystem.IsWindows())
            return;

        try
        {
            if (Console.WindowWidth > MaxConsoleWidth)
                Console.WindowWidth = MaxConsoleWidth;
        }
        catch (PlatformNotSupportedException) { }
        catch (IOException) { }
    }
}
