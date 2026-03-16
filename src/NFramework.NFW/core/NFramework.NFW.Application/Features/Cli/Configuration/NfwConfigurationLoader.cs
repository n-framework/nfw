using System.Collections;
using YamlDotNet.Serialization;

namespace NFramework.NFW.Application.Features.Cli.Configuration;

public sealed class NfwConfigurationLoader : INfwConfigurationLoader
{
    private const string ConfigFileName = "nfw.yaml";
    private const string EnvironmentPrefix = "NFW_";

    private readonly IDeserializer _deserializer;

    public NfwConfigurationLoader()
    {
        _deserializer = new DeserializerBuilder().Build();
    }

    public NfwConfiguration Load()
    {
        var configFilePath = Path.Combine(Directory.GetCurrentDirectory(), ConfigFileName);
        var values = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
        var sources = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);

        LoadFromYamlFile(configFilePath, values, sources);
        ApplyEnvironmentOverrides(values, sources);

        return new NfwConfiguration(configFilePath, values, sources);
    }

    private void LoadFromYamlFile(
        string configFilePath,
        IDictionary<string, string> values,
        IDictionary<string, string> sources
    )
    {
        if (!File.Exists(configFilePath))
        {
            return;
        }

        try
        {
            var yamlContent = File.ReadAllText(configFilePath);
            var parsedYaml = _deserializer.Deserialize<object?>(yamlContent);
            Flatten("", parsedYaml, values, sources);
        }
        catch (Exception exception)
        {
            Console.Error.WriteLine($"Configuration parse error in '{ConfigFileName}': {exception.Message}");
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
