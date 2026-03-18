using NFramework.NFW.Application.Features.Cli.Configuration;
using Xunit;

namespace NFramework.NFW.CLI.Tests.core.NFramework.NFW.Application.Features.Cli.Configuration;

public sealed class RequiredConfigurationValidatorTests
{
    [Fact]
    public void Validate_ReturnsEmpty_WhenNoRequiredKeysAreDefined()
    {
        RequiredConfigurationValidator validator = new RequiredConfigurationValidator();
        NfwConfiguration configuration = NfwConfiguration.Empty("nfw.yaml");

        IReadOnlyList<string> missingKeys = validator.Validate(configuration);

        missingKeys.ShouldBeEmpty();
    }
}
