using System.Reflection;
using NFramework.NFW.Application.Features.Versioning.Abstractions;
using NFramework.NFW.Domain.Features.Version;

namespace NFramework.NFW.Application.Features.Versioning;

public sealed class VersionProvider : IVersionProvider
{
    public VersionInfo GetVersionInfo()
    {
        string versionValue = ResolveVersionValue();
        int plusIndex = versionValue.IndexOf('+', StringComparison.Ordinal);
        if (plusIndex < 0)
        {
            return VersionInfo.Create(versionValue);
        }

        string semanticVersion = versionValue[..plusIndex];
        string metadata = versionValue[(plusIndex + 1)..];
        return VersionInfo.Create(semanticVersion, metadata);
    }

    private static string ResolveVersionValue()
    {
        Assembly assembly = Assembly.GetEntryAssembly() ?? Assembly.GetExecutingAssembly();
        string? informationalVersion = assembly
            .GetCustomAttribute<AssemblyInformationalVersionAttribute>()
            ?.InformationalVersion;

        if (!string.IsNullOrWhiteSpace(informationalVersion))
        {
            return informationalVersion.Trim();
        }

        Version? assemblyVersion = assembly.GetName().Version;
        if (assemblyVersion is null)
        {
            return VersionInfo.CreateDefault().SemanticVersion;
        }

        return $"{assemblyVersion.Major}.{Math.Max(assemblyVersion.Minor, 0)}.{Math.Max(assemblyVersion.Build, 0)}";
    }
}
