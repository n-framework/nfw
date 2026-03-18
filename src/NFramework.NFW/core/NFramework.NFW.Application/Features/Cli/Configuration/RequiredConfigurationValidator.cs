namespace NFramework.NFW.Application.Features.Cli.Configuration;

public sealed class RequiredConfigurationValidator(params string[] requiredKeys)
{
    private readonly string[] _requiredKeys = requiredKeys?.ToArray() ?? Array.Empty<string>();

    public IReadOnlyList<string> Validate(NfwConfiguration configuration)
    {
        return _requiredKeys.Where(requiredKey => !configuration.Values.ContainsKey(requiredKey)).ToArray();
    }
}
