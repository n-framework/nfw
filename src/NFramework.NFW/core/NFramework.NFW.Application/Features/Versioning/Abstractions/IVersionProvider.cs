using NFramework.NFW.Domain.Features.Version;

namespace NFramework.NFW.Application.Features.Versioning.Abstractions;

public interface IVersionProvider
{
    VersionInfo GetVersionInfo();
}
