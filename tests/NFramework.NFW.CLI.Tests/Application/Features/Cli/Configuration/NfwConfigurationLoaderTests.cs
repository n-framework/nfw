using NFramework.NFW.Application.Features.Cli.Configuration;
using Xunit;

namespace NFramework.NFW.CLI.Tests.Application.Features.Cli.Configuration;

public class NfwConfigurationLoaderTests
{
    private readonly NfwConfigurationLoader _loader = new();

    [Fact]
    public void Load_WhenNoConfigFile_ReturnsSuccessWithEmptyConfiguration()
    {
        // Arrange - ensure no config file exists in current directory
        var currentDir = Directory.GetCurrentDirectory();
        var configFile = Path.Combine(currentDir, "nfw.yaml");
        var tempDir = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
        Directory.CreateDirectory(tempDir);

        try
        {
            Directory.SetCurrentDirectory(tempDir);

            // Act
            var result = _loader.Load();

            // Assert
            Assert.True(result.IsSuccess);
            Assert.NotNull(result.Value);
        }
        finally
        {
            Directory.SetCurrentDirectory(currentDir);
            if (Directory.Exists(tempDir))
            {
                Directory.Delete(tempDir, true);
            }
        }
    }

    [Fact]
    public void Load_WithValidYaml_ReturnsSuccess()
    {
        // Arrange
        var currentDir = Directory.GetCurrentDirectory();
        var tempDir = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
        Directory.CreateDirectory(tempDir);

        try
        {
            Directory.SetCurrentDirectory(tempDir);
            File.WriteAllText("nfw.yaml", "key: value");

            // Act
            var result = _loader.Load();

            // Assert
            Assert.True(result.IsSuccess);
            Assert.True(result.Value!.Values.ContainsKey("key"));
            Assert.Equal("value", result.Value!.Values["key"]);
        }
        finally
        {
            Directory.SetCurrentDirectory(currentDir);
            if (Directory.Exists(tempDir))
            {
                Directory.Delete(tempDir, true);
            }
        }
    }

    [Fact]
    public void Load_WithNestedYaml_FlattenCorrectly()
    {
        // Arrange
        var currentDir = Directory.GetCurrentDirectory();
        var tempDir = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
        Directory.CreateDirectory(tempDir);

        try
        {
            Directory.SetCurrentDirectory(tempDir);
            File.WriteAllText(
                "nfw.yaml",
                """
                parent:
                  child: value
                """
            );

            // Act
            var result = _loader.Load();

            // Assert
            Assert.True(result.IsSuccess);
            Assert.True(result.Value!.Values.ContainsKey("parent:child"));
            Assert.Equal("value", result.Value!.Values["parent:child"]);
        }
        finally
        {
            Directory.SetCurrentDirectory(currentDir);
            if (Directory.Exists(tempDir))
            {
                Directory.Delete(tempDir, true);
            }
        }
    }

    [Fact]
    public void Load_WithList_FlattensWithIndex()
    {
        // Arrange
        var currentDir = Directory.GetCurrentDirectory();
        var tempDir = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
        Directory.CreateDirectory(tempDir);

        try
        {
            Directory.SetCurrentDirectory(tempDir);
            File.WriteAllText(
                "nfw.yaml",
                """
                items:
                  - first
                  - second
                """
            );

            // Act
            var result = _loader.Load();

            // Assert
            Assert.True(result.IsSuccess);
            Assert.True(result.Value!.Values.ContainsKey("items:0"));
            Assert.Equal("first", result.Value!.Values["items:0"]);
            Assert.True(result.Value!.Values.ContainsKey("items:1"));
            Assert.Equal("second", result.Value!.Values["items:1"]);
        }
        finally
        {
            Directory.SetCurrentDirectory(currentDir);
            if (Directory.Exists(tempDir))
            {
                Directory.Delete(tempDir, true);
            }
        }
    }

    [Fact]
    public void Load_WithEnvironmentVariables_OverridesYaml()
    {
        // Arrange
        var currentDir = Directory.GetCurrentDirectory();
        var tempDir = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
        Directory.CreateDirectory(tempDir);

        try
        {
            Directory.SetCurrentDirectory(tempDir);
            File.WriteAllText("nfw.yaml", "key: from-yaml");
            Environment.SetEnvironmentVariable("NFW_KEY", "from-env");

            // Act
            var result = _loader.Load();

            // Assert
            Assert.True(result.IsSuccess);
            Assert.Equal("from-env", result.Value!.Values["key"]);
        }
        finally
        {
            Directory.SetCurrentDirectory(currentDir);
            Environment.SetEnvironmentVariable("NFW_KEY", null);
            if (Directory.Exists(tempDir))
            {
                Directory.Delete(tempDir, true);
            }
        }
    }

    [Fact]
    public void Load_WithNestedEnvironmentVariable_OverridesCorrectly()
    {
        // Arrange
        var currentDir = Directory.GetCurrentDirectory();
        var tempDir = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
        Directory.CreateDirectory(tempDir);

        try
        {
            Directory.SetCurrentDirectory(tempDir);
            File.WriteAllText(
                "nfw.yaml",
                """
                database:
                  connection: from-yaml
                """
            );
            Environment.SetEnvironmentVariable("NFW_DATABASE__CONNECTION", "from-env");

            // Act
            var result = _loader.Load();

            // Assert
            Assert.True(result.IsSuccess);
            Assert.Equal("from-env", result.Value!.Values["database:connection"]);
        }
        finally
        {
            Directory.SetCurrentDirectory(currentDir);
            Environment.SetEnvironmentVariable("NFW_DATABASE__CONNECTION", null);
            if (Directory.Exists(tempDir))
            {
                Directory.Delete(tempDir, true);
            }
        }
    }

    [Fact]
    public void Load_WithInvalidYaml_ReturnsFailure()
    {
        // Arrange
        var currentDir = Directory.GetCurrentDirectory();
        var tempDir = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
        Directory.CreateDirectory(tempDir);

        try
        {
            Directory.SetCurrentDirectory(tempDir);
            File.WriteAllText("nfw.yaml", "invalid: yaml: content: [unclosed");

            // Act
            var result = _loader.Load();

            // Assert
            Assert.True(result.IsFailure);
            Assert.Contains("invalid YAML syntax", result.Error);
        }
        finally
        {
            Directory.SetCurrentDirectory(currentDir);
            if (Directory.Exists(tempDir))
            {
                Directory.Delete(tempDir, true);
            }
        }
    }

    [Fact]
    public void Load_WithSourceTracking_TracksFileAndEnv()
    {
        // Arrange
        var currentDir = Directory.GetCurrentDirectory();
        var tempDir = Path.Combine(Path.GetTempPath(), Guid.NewGuid().ToString());
        Directory.CreateDirectory(tempDir);

        try
        {
            Directory.SetCurrentDirectory(tempDir);
            File.WriteAllText(
                "nfw.yaml",
                """
                from-file: value1
                """
            );
            Environment.SetEnvironmentVariable("NFW_FROM_ENV", "value2");

            // Act
            var result = _loader.Load();

            // Assert
            Assert.True(result.IsSuccess);
            Assert.Equal("file", result.Value!.Sources["from-file"]);
            Assert.Equal("env", result.Value!.Sources["from_env"]);
        }
        finally
        {
            Directory.SetCurrentDirectory(currentDir);
            Environment.SetEnvironmentVariable("NFW_FROM_ENV", null);
            if (Directory.Exists(tempDir))
            {
                Directory.Delete(tempDir, true);
            }
        }
    }
}
