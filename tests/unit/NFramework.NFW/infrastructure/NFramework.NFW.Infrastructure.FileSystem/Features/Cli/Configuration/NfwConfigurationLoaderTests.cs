using NFramework.NFW.Application.Features.Cli.Configuration;
using NFramework.NFW.Infrastructure.FileSystem.Features.Cli.Configuration;
using Xunit;

namespace NFramework.NFW.Infrastructure.FileSystem.Tests.Features.Cli.Configuration;

[Collection("Cli command tests")]
public sealed class NfwConfigurationLoaderTests
{
    private readonly NfwConfigurationLoader _loader = new();

    [Fact]
    public void Load_WhenNoConfigFile_ReturnsSuccessWithEmptyConfiguration()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create();

        Result<NfwConfiguration> result = _loader.Load();

        result.IsSuccess.ShouldBeTrue();
        _ = result.Value.ShouldNotBeNull();
    }

    [Fact]
    public void Load_WithValidYaml_ReturnsSuccess()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create("key: value");

        Result<NfwConfiguration> result = _loader.Load();

        result.IsSuccess.ShouldBeTrue();
        result.Value!.Values["key"].ShouldBe("value");
    }

    [Fact]
    public void Load_WithNestedYaml_FlattensCorrectly()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create(
            """
            parent:
              child: value
            """
        );

        Result<NfwConfiguration> result = _loader.Load();

        result.IsSuccess.ShouldBeTrue();
        result.Value!.Values["parent:child"].ShouldBe("value");
    }

    [Fact]
    public void Load_WithList_FlattensWithIndex()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create(
            """
            items:
              - first
              - second
            """
        );

        Result<NfwConfiguration> result = _loader.Load();

        result.IsSuccess.ShouldBeTrue();
        result.Value!.Values["items:0"].ShouldBe("first");
        result.Value!.Values["items:1"].ShouldBe("second");
    }

    [Fact]
    public void Load_WithEnvironmentVariables_OverridesYaml()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create("key: from-yaml");
        Environment.SetEnvironmentVariable("NFW_KEY", "from-env");

        try
        {
            Result<NfwConfiguration> result = _loader.Load();

            result.IsSuccess.ShouldBeTrue();
            result.Value!.Values["key"].ShouldBe("from-env");
        }
        finally
        {
            Environment.SetEnvironmentVariable("NFW_KEY", null);
        }
    }

    [Fact]
    public void Load_WithNestedEnvironmentVariable_OverridesCorrectly()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create(
            """
            database:
              connection: from-yaml
            """
        );
        Environment.SetEnvironmentVariable("NFW_DATABASE__CONNECTION", "from-env");

        try
        {
            Result<NfwConfiguration> result = _loader.Load();

            result.IsSuccess.ShouldBeTrue();
            result.Value!.Values["database:connection"].ShouldBe("from-env");
        }
        finally
        {
            Environment.SetEnvironmentVariable("NFW_DATABASE__CONNECTION", null);
        }
    }

    [Fact]
    public void Load_WithInvalidYaml_ReturnsFailure()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create(
            "invalid: yaml: content: [unclosed"
        );

        Result<NfwConfiguration> result = _loader.Load();

        result.IsFailure.ShouldBeTrue();
        result.Error.ShouldNotBeNull().ShouldContain("invalid YAML syntax");
    }

    [Fact]
    public void Load_WithSourceTracking_TracksFileAndEnv()
    {
        using TemporaryWorkingDirectory workingDirectory = TemporaryWorkingDirectory.Create("key: from-yaml");
        Environment.SetEnvironmentVariable("NFW_KEY", "from-env");

        try
        {
            Result<NfwConfiguration> result = _loader.Load();

            result.IsSuccess.ShouldBeTrue();
            result.Value!.Sources["key"].ShouldBe("env");
        }
        finally
        {
            Environment.SetEnvironmentVariable("NFW_KEY", null);
        }
    }

    private sealed class TemporaryWorkingDirectory : IDisposable
    {
        private readonly string _originalDirectory = Directory.GetCurrentDirectory();

        private TemporaryWorkingDirectory(string path)
        {
            Path = path;
            Directory.SetCurrentDirectory(path);
        }

        public string Path { get; }

        public static TemporaryWorkingDirectory Create(string? configContent = null)
        {
            string path = System.IO.Path.Combine(System.IO.Path.GetTempPath(), Guid.NewGuid().ToString("N"));
            _ = Directory.CreateDirectory(path);
            if (configContent is not null)
            {
                File.WriteAllText(System.IO.Path.Combine(path, "nfw.yaml"), configContent);
            }

            return new TemporaryWorkingDirectory(path);
        }

        public void Dispose()
        {
            Directory.SetCurrentDirectory(_originalDirectory);
            if (Directory.Exists(Path))
            {
                Directory.Delete(Path, recursive: true);
            }
        }
    }
}
