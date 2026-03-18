namespace NFramework.NFW.Application.Features.Cli.Configuration.Abstractions;

public interface INfwConfigurationLoader
{
    Result<NfwConfiguration> Load();
}
