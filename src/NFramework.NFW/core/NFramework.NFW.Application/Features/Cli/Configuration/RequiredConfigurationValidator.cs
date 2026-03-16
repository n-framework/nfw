namespace NFramework.NFW.Application.Features.Cli.Configuration;

public sealed class RequiredConfigurationValidator
{
    private static readonly string[] RequiredKeys = [];

    public IReadOnlyList<string> Validate(NfwConfiguration configuration)
    {
        return RequiredKeys.Where(requiredKey => !configuration.Values.ContainsKey(requiredKey)).ToArray();
    }
}
