using NFramework.Core.CLI.Abstractions;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;

public sealed class NewCliCommandSettings
{
    [CliArgument(0, "[workspace-name]")]
    public string? WorkspaceName { get; init; }

    [CliOption("--template <IDENTIFIER>")]
    public string? TemplateIdentifier { get; init; }

    [CliOption("--no-input")]
    public bool NoInput { get; init; }
}
