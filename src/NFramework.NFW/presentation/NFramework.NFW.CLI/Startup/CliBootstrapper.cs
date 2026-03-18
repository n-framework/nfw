using Microsoft.Extensions.DependencyInjection;
using NFramework.NFW.Application.Features.Cli;
using NFramework.NFW.Application.Features.Cli.Configuration;
using NFramework.NFW.Application.Features.Cli.Configuration.Abstractions;
using NFramework.NFW.Application.Features.Versioning.Abstractions;

namespace NFramework.NFW.CLI.Startup;

internal static class CliBootstrapper
{
    public static CliBootstrapResult Bootstrap(CliServices cliServices)
    {
        using ServiceProvider serviceProvider = cliServices.Services.BuildServiceProvider();
        INfwConfigurationLoader configurationLoader = serviceProvider.GetRequiredService<INfwConfigurationLoader>();
        RequiredConfigurationValidator requiredConfigurationValidator =
            serviceProvider.GetRequiredService<RequiredConfigurationValidator>();
        IVersionProvider versionProvider = serviceProvider.GetRequiredService<IVersionProvider>();

        cliServices.DiagnosticLogger.Write("Loading configuration from nfw.yaml and environment.");

        Result<NfwConfiguration> configurationResult = configurationLoader.Load();
        if (configurationResult.IsFailure)
            return CliBootstrapResult.Failure(
                $"Configuration error: {configurationResult.Error}",
                ExitCodes.RuntimeFailure
            );

        IReadOnlyList<string> missingConfigurationKeys = requiredConfigurationValidator.Validate(
            configurationResult.Value!
        );
        if (missingConfigurationKeys.Count > 0)
        {
            string message = $"Missing required configuration values: {string.Join(", ", missingConfigurationKeys)}";
            return CliBootstrapResult.Failure(message, ExitCodes.RuntimeFailure);
        }

        return CliBootstrapResult.Success(versionProvider.GetVersionInfo().ToString());
    }
}

internal sealed record CliBootstrapResult(string? VersionText, string? ErrorMessage, int ExitCode)
{
    public bool IsFailure => ErrorMessage is not null;

    public static CliBootstrapResult Failure(string errorMessage, int exitCode)
    {
        return new CliBootstrapResult(null, errorMessage, exitCode);
    }

    public static CliBootstrapResult Success(string versionText)
    {
        return new CliBootstrapResult(versionText, null, ExitCodes.Success);
    }
}
