using System.Text.RegularExpressions;

namespace NFramework.NFW.Domain.Features.Version;

public sealed record VersionInfo(string SemanticVersion, string? BuildMetadata)
{
    private static readonly Regex SemanticVersionRegex = new(
        "^[0-9]+\\.[0-9]+\\.[0-9]+(?:-[0-9A-Za-z.-]+)?$",
        RegexOptions.Compiled
    );

    public override string ToString()
    {
        return string.IsNullOrWhiteSpace(BuildMetadata) ? SemanticVersion : $"{SemanticVersion}+{BuildMetadata}";
    }

    public static VersionInfo Create(string semanticVersion, string? buildMetadata = null)
    {
        if (string.IsNullOrWhiteSpace(semanticVersion))
        {
            throw new ArgumentException("Semantic version cannot be empty or whitespace.", nameof(semanticVersion));
        }

        string normalized = semanticVersion.Trim();
        if (!SemanticVersionRegex.IsMatch(normalized))
        {
            throw new ArgumentException($"'{normalized}' is not a valid semantic version.", nameof(semanticVersion));
        }

        return new VersionInfo(normalized, buildMetadata?.Trim());
    }

    public static VersionInfo CreateDefault()
    {
        return new VersionInfo("0.1.0", null);
    }
}
