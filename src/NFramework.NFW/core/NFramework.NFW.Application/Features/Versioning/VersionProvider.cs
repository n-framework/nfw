using System.Reflection;
using System.Text.RegularExpressions;
using NFramework.NFW.Domain.Features.Version;

namespace NFramework.NFW.Application.Features.Versioning;

public sealed class VersionProvider : IVersionProvider
{
    private static readonly Regex SemanticVersionRegex = new(
        "^[0-9]+\\.[0-9]+\\.[0-9]+(?:-[0-9A-Za-z.-]+)?$",
        RegexOptions.Compiled
    );

    public VersionInfo GetVersionInfo()
    {
        var versionValue = ResolveVersionValue();
        var plusIndex = versionValue.IndexOf('+', StringComparison.Ordinal);
        if (plusIndex < 0)
        {
            return new VersionInfo(NormalizeSemanticVersion(versionValue), null);
        }

        var semanticVersion = versionValue[..plusIndex];
        var metadata = versionValue[(plusIndex + 1)..];
        return new VersionInfo(NormalizeSemanticVersion(semanticVersion), metadata);
    }

    private static string ResolveVersionValue()
    {
        var assembly = Assembly.GetEntryAssembly() ?? Assembly.GetExecutingAssembly();
        var informationalVersion = assembly
            .GetCustomAttribute<AssemblyInformationalVersionAttribute>()
            ?.InformationalVersion;

        if (!string.IsNullOrWhiteSpace(informationalVersion))
        {
            return informationalVersion.Trim();
        }

        var assemblyVersion = assembly.GetName().Version;
        if (assemblyVersion is null)
        {
            return "0.1.0";
        }

        return $"{assemblyVersion.Major}.{Math.Max(assemblyVersion.Minor, 0)}.{Math.Max(assemblyVersion.Build, 0)}";
    }

    private static string NormalizeSemanticVersion(string versionValue)
    {
        if (SemanticVersionRegex.IsMatch(versionValue))
        {
            return versionValue;
        }

        return "0.1.0";
    }
}
