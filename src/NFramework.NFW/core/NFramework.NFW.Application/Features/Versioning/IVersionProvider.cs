using NFramework.NFW.Domain.Features.Version;

namespace NFramework.NFW.Application.Features.Versioning;

public interface IVersionProvider
{
    VersionInfo GetVersionInfo();
}
