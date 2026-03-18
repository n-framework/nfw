using System.Collections;
using NFramework.NFW.Application.Features.Cli.Configuration;
using NFramework.NFW.Application.Features.Cli.Configuration.Abstractions;
using YamlDotNet.Core;
using YamlDotNet.Serialization;

namespace NFramework.NFW.Infrastructure.FileSystem.Features.Cli.Configuration;

public sealed class NfwConfigurationLoader : INfwConfigurationLoader
{
    private const string ConfigFileName = "nfw.yaml";
    private const string EnvironmentPrefix = "NFW_";

    public Result<NfwConfiguration> Load()
    {
        string configFilePath = Path.Combine(Directory.GetCurrentDirectory(), ConfigFileName);
        Dictionary<string, string> values = new(StringComparer.OrdinalIgnoreCase);
        Dictionary<string, string> sources = new(StringComparer.OrdinalIgnoreCase);

        Result<NfwConfiguration> loadResult = LoadFromYamlFile(configFilePath, values, sources);
        if (loadResult.IsFailure)
        {
            return loadResult;
        }

        ApplyEnvironmentOverrides(values, sources);

        NfwConfiguration configuration = new(configFilePath, values, sources);
        return Result<NfwConfiguration>.Success(configuration);
    }

    private static Result<NfwConfiguration> LoadFromYamlFile(
        string configFilePath,
        Dictionary<string, string> values,
        Dictionary<string, string> sources
    )
    {
        if (!File.Exists(configFilePath))
        {
            return Result<NfwConfiguration>.Success(default!);
        }

        try
        {
            string yamlContent = File.ReadAllText(configFilePath);
            IDeserializer deserializer = new DeserializerBuilder().Build();
            object? parsedYaml = deserializer.Deserialize<object?>(yamlContent);
            Flatten(string.Empty, parsedYaml, values, sources);
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
                foreach (KeyValuePair<object, object> pair in objectDictionary)
                {
                    string? key = pair.Key?.ToString();
                    if (string.IsNullOrWhiteSpace(key))
                    {
                        continue;
                    }

                    string nestedPrefix = BuildKey(prefix, key);
                    Flatten(nestedPrefix, pair.Value, values, sources);
                }

                return;
            case IDictionary<string, object> stringDictionary:
                foreach (KeyValuePair<string, object> pair in stringDictionary)
                {
                    string nestedPrefix = BuildKey(prefix, pair.Key);
                    Flatten(nestedPrefix, pair.Value, values, sources);
                }

                return;
            case IList list:
                for (int index = 0; index < list.Count; index += 1)
                {
                    string nestedPrefix = BuildKey(prefix, index.ToString());
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

    private static void ApplyEnvironmentOverrides(Dictionary<string, string> values, Dictionary<string, string> sources)
    {
        foreach (DictionaryEntry pair in Environment.GetEnvironmentVariables())
        {
            string? key = pair.Key?.ToString();
            string? value = pair.Value?.ToString();
            if (string.IsNullOrWhiteSpace(key) || value is null)
            {
                continue;
            }

            if (!key.StartsWith(EnvironmentPrefix, StringComparison.OrdinalIgnoreCase))
            {
                continue;
            }

            string normalizedKey = key[EnvironmentPrefix.Length..]
                .Replace("__", ":", StringComparison.Ordinal)
                .ToLowerInvariant();
            values[normalizedKey] = value;
            sources[normalizedKey] = "env";
        }
    }

    private static string BuildKey(string prefix, string key)
    {
        string normalizedKey = key.Trim().ToLowerInvariant();
        return string.IsNullOrWhiteSpace(prefix) ? normalizedKey : $"{prefix}:{normalizedKey}";
    }
}
