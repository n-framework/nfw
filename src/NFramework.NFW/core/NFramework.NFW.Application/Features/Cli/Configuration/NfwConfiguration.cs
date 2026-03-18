namespace NFramework.NFW.Application.Features.Cli.Configuration;

public sealed class NfwConfiguration(
    string filePath,
    IReadOnlyDictionary<string, string> values,
    IReadOnlyDictionary<string, string> sources
)
{
    public string FilePath { get; } = filePath;

    public IReadOnlyDictionary<string, string> Values { get; } = values;

    public IReadOnlyDictionary<string, string> Sources { get; } = sources;

    public static NfwConfiguration Empty(string filePath)
    {
        return new NfwConfiguration(filePath, new Dictionary<string, string>(), new Dictionary<string, string>());
    }
}
