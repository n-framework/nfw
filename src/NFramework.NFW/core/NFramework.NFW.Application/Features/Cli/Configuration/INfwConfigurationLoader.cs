namespace NFramework.NFW.Application.Features.Cli.Configuration;

public interface INfwConfigurationLoader
{
    Result<NfwConfiguration> Load();
}
