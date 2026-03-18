using Spectre.Console.Cli;

namespace NFramework.NFW.CLI.Features.ProjectManagement.Commands.New;

public sealed class NewCliCommandSettings : CommandSettings
{
    [CommandArgument(0, "[workspace-name]")]
    public string? WorkspaceName { get; init; }

    [CommandOption("--template <IDENTIFIER>")]
    public string? TemplateIdentifier { get; init; }

    [CommandOption("--no-input")]
    public bool NoInput { get; init; }
}
