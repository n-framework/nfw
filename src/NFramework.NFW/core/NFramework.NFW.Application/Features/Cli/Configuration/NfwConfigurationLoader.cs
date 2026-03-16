using System.Collections;
using YamlDotNet.Core;
using YamlDotNet.Serialization;

namespace NFramework.NFW.Application.Features.Cli.Configuration;

public sealed class NfwConfigurationLoader : INfwConfigurationLoader
{
    private const string ConfigFileName = "nfw.yaml";
    private const string EnvironmentPrefix = "NFW_";

    public Result<NfwConfiguration> Load()
    {
        var configFilePath = Path.Combine(Directory.GetCurrentDirectory(), ConfigFileName);
        var values = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
        var sources = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);

        var loadResult = LoadFromYamlFile(configFilePath, values, sources);
        if (loadResult.IsFailure)
        {
            return loadResult;
        }

        ApplyEnvironmentOverrides(values, sources);

        var configuration = new NfwConfiguration(configFilePath, values, sources);
        return Result<NfwConfiguration>.Success(configuration);
    }

    private static Result<NfwConfiguration> LoadFromYamlFile(
        string configFilePath,
        IDictionary<string, string> values,
        IDictionary<string, string> sources
    )
    {
        if (!File.Exists(configFilePath))
        {
            return Result<NfwConfiguration>.Success(default!); // File is optional
        }

        try
        {
            var yamlContent = File.ReadAllText(configFilePath);
            var deserializer = new DeserializerBuilder().Build();
            var parsedYaml = deserializer.Deserialize<object?>(yamlContent);
            Flatten("", parsedYaml, values, sources);
            return Result<NfwConfiguration>.Success(default!);
        }
        catch (FileNotFoundException exception)
        {
            return Result<NfwConfiguration>.Failure($"Configuration file not found: {configFilePath}", exception);
        }
        catch (IOException exception)
        {
            return Result<NfwConfiguration>.Failure($"Unable to read configuration file: {configFilePath}", exception);
        }
        catch (YamlException exception)
        {
            return Result<NfwConfiguration>.Failure(
                $"Configuration file has invalid YAML syntax: {ConfigFileName}",
                exception
            );
        }
        catch (Exception exception)
        {
            return Result<NfwConfiguration>.Failure(
                $"Unexpected error loading configuration: {exception.Message}",
                exception
            );
        }
    }

    private static void Flatten(
        string prefix,
        object? value,
        IDictionary<string, string> values,
        IDictionary<string, string> sources
    )
    {
        switch (value)
        {
            case null:
                return;
            case IDictionary<object, object> objectDictionary:
                foreach (var pair in objectDictionary)
                {
                    var key = pair.Key?.ToString();
                    if (string.IsNullOrWhiteSpace(key))
                    {
                        continue;
                    }

                    var nestedPrefix = BuildKey(prefix, key);
                    Flatten(nestedPrefix, pair.Value, values, sources);
                }

                return;
            case IDictionary<string, object> stringDictionary:
                foreach (var pair in stringDictionary)
                {
                    var nestedPrefix = BuildKey(prefix, pair.Key);
                    Flatten(nestedPrefix, pair.Value, values, sources);
                }

                return;
            case IList list:
                for (var index = 0; index < list.Count; index += 1)
                {
                    var nestedPrefix = BuildKey(prefix, index.ToString());
                    Flatten(nestedPrefix, list[index], values, sources);
                }

                return;
            default:
                if (string.IsNullOrWhiteSpace(prefix))
                {
                    return;
                }

                values[prefix] = value.ToString() ?? string.Empty;
                sources[prefix] = "file";
                return;
        }
    }

    private static void ApplyEnvironmentOverrides(
        IDictionary<string, string> values,
        IDictionary<string, string> sources
    )
    {
        foreach (DictionaryEntry pair in Environment.GetEnvironmentVariables())
        {
            var key = pair.Key?.ToString();
            var value = pair.Value?.ToString();
            if (string.IsNullOrWhiteSpace(key) || value is null)
            {
                continue;
            }

            if (!key.StartsWith(EnvironmentPrefix, StringComparison.OrdinalIgnoreCase))
            {
                continue;
            }

            var normalizedKey = key[EnvironmentPrefix.Length..]
                .Replace("__", ":", StringComparison.Ordinal)
                .ToLowerInvariant();
            values[normalizedKey] = value;
            sources[normalizedKey] = "env";
        }
    }

    private static string BuildKey(string prefix, string key)
    {
        var normalizedKey = key.Trim().ToLowerInvariant();
        return string.IsNullOrWhiteSpace(prefix) ? normalizedKey : $"{prefix}:{normalizedKey}";
    }
}
