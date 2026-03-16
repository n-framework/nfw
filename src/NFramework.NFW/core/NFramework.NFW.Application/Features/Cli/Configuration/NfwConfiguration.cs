namespace NFramework.NFW.Application.Features.Cli.Configuration;

public sealed class NfwConfiguration
{
    public NfwConfiguration(
        string filePath,
        IReadOnlyDictionary<string, string> values,
        IReadOnlyDictionary<string, string> sources
    )
    {
        FilePath = filePath;
        Values = values;
        Sources = sources;
    }

    public string FilePath { get; }

    public IReadOnlyDictionary<string, string> Values { get; }

    public IReadOnlyDictionary<string, string> Sources { get; }

    public static NfwConfiguration Empty(string filePath)
    {
        return new NfwConfiguration(filePath, new Dictionary<string, string>(), new Dictionary<string, string>());
    }
}
