using System.Reflection;
using NFramework.NFW.Domain.Features.Version;

namespace NFramework.NFW.Application.Features.Versioning;

public sealed class VersionProvider : IVersionProvider
{
    public VersionInfo GetVersionInfo()
    {
        var versionValue = ResolveVersionValue();
        var plusIndex = versionValue.IndexOf('+', StringComparison.Ordinal);
        if (plusIndex < 0)
        {
            return VersionInfo.Create(versionValue);
        }

        var semanticVersion = versionValue[..plusIndex];
        var metadata = versionValue[(plusIndex + 1)..];
        return VersionInfo.Create(semanticVersion, metadata);
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
            return VersionInfo.CreateDefault().SemanticVersion;
        }

        return $"{assemblyVersion.Major}.{Math.Max(assemblyVersion.Minor, 0)}.{Math.Max(assemblyVersion.Build, 0)}";
    }
}
