using FluentAssertions;
using NFramework.NFW.Application.Features.Cli.Configuration;
using Xunit;

namespace NFramework.NFW.CLI.Tests.Cli;

public sealed class RequiredConfigurationValidatorTests
{
    [Fact]
    public void Validate_ReturnsEmpty_WhenNoRequiredKeysAreDefined()
    {
        var validator = new RequiredConfigurationValidator();
        var configuration = NfwConfiguration.Empty("nfw.yaml");

        var missingKeys = validator.Validate(configuration);

        missingKeys.Should().BeEmpty();
    }
}
