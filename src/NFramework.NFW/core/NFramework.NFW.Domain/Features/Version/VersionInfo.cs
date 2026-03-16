namespace NFramework.NFW.Domain.Features.Version;

public sealed record VersionInfo(string SemanticVersion, string? BuildMetadata)
{
    public override string ToString()
    {
        return string.IsNullOrWhiteSpace(BuildMetadata) ? SemanticVersion : $"{SemanticVersion}+{BuildMetadata}";
    }
}
